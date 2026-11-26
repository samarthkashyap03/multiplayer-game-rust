use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use tungstenite::client::IntoClientRequest;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use qrcode::QrCode;

use crate::protocol::{Button, GameMsg, Layout, PROTO, ToGame};

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    Connected { slot: String },
    Disconnected { slot: String },
    ButtonEdge { slot: String, button: Button, down: bool },
    StartGame,
}

/// Raw RGBA pixel data for a QR code, plus its width in pixels.
#[derive(Debug, Clone)]
pub struct QrImageData {
    pub rgba: Vec<u8>,
    pub width: u32,
}

pub struct NetworkManager {
    pub slot_links: HashMap<String, String>,
    /// QR code pixel data per slot, ready to become macroquad Textures.
    pub qr_images: HashMap<String, QrImageData>,
    pub rx: Receiver<NetworkEvent>,
    _stop_tx: Sender<()>,
}

impl NetworkManager {
    pub fn start() -> Result<Self, Box<dyn Error>> {
        let (event_tx, rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        // 1. Determine Relay URL and Token (from environment variables or .env file)
        let base_url = load_env_var("INPUT_MUX_URL")
            .unwrap_or_else(|| "https://scilab-0098.informatik.uni-kl.de".to_string());
        let token = load_env_var("INPUT_MUX_TOKEN")
            .ok_or("INPUT_MUX_TOKEN not set in environment or .env file")?;

        let mut socket = connect_relay(&base_url, &token)?;

        let slots = ["p1", "p2", "p3", "p4"];
        let names = [
            "P1: Barbarossa",
            "P2: Imperial Pike",
            "P3: Casimir",
            "P4: Rudolf",
        ];

        // 2. Open slots & set display names
        for (i, slot) in slots.iter().enumerate() {
            send_msg(
                &mut socket,
                &GameMsg::OpenSlot {
                    req_id: (i + 1) as u64,
                    slot: slot.to_string(),
                },
            )?;
            send_msg(
                &mut socket,
                &GameMsg::SetDisplayName {
                    slot: slot.to_string(),
                    display_name: Some(names[i].to_string()),
                },
            )?;
        }

        // 3. Collect slot URLs
        let mut slot_links = HashMap::new();
        while slot_links.len() < slots.len() {
            match recv_msg(&mut socket)? {
                ToGame::SlotLink { slot, url, .. } => {
                    slot_links.insert(slot, url);
                }
                ToGame::Error { message, .. } => {
                    eprintln!("Relay warning: {message}");
                }
                _ => {}
            }
        }

        // 4. Generate QR code images for each slot link
        let mut qr_images = HashMap::new();
        for (slot, url) in &slot_links {
            if let Some(qr_data) = generate_qr_image(url) {
                qr_images.insert(slot.clone(), qr_data);
            }
        }

        // 5. Spawn listener thread
        let tx = event_tx.clone();
        thread::spawn(move || {
            while stop_rx.try_recv() == Err(TryRecvError::Empty) {
                match recv_msg(&mut socket) {
                    Ok(ToGame::PhoneConnected { slot }) => {
                        let _ = tx.send(NetworkEvent::Connected { slot });
                    }
                    Ok(ToGame::PhoneDisconnected { slot }) => {
                        let _ = tx.send(NetworkEvent::Disconnected { slot });
                    }
                    Ok(ToGame::Frame(frame)) => {
                        for (slot, state) in &frame.slots {
                            for edge in &state.edges {
                                if edge.down && edge.button == Button::Menu {
                                    let _ = tx.send(NetworkEvent::StartGame);
                                } else {
                                    let _ = tx.send(NetworkEvent::ButtonEdge {
                                        slot: slot.clone(),
                                        button: edge.button,
                                        down: edge.down,
                                    });
                                }
                            }
                        }
                    }
                    Ok(ToGame::Error { message, .. }) => {
                        eprintln!("Relay error: {message}");
                    }
                    Err(_) => break,
                    _ => {}
                }
            }
            let _ = send_msg(&mut socket, &GameMsg::EndSession);
            let _ = socket.close(None);
        });

        Ok(Self {
            slot_links,
            qr_images,
            rx,
            _stop_tx: stop_tx,
        })
    }

    pub fn poll_events(&self) -> Vec<NetworkEvent> {
        let mut events = Vec::new();
        while let Ok(evt) = self.rx.try_recv() {
            events.push(evt);
        }
        events
    }
}

fn connect_relay(base: &str, token: &str) -> Result<Socket, Box<dyn Error>> {
    let url = if let Some(host) = base.trim_end_matches('/').strip_prefix("https://") {
        format!("wss://{host}/ws/game")
    } else if let Some(host) = base.trim_end_matches('/').strip_prefix("http://") {
        format!("ws://{host}/ws/game")
    } else {
        return Err(format!("Invalid URL: {base}").into());
    };

    let _ = rustls::crypto::ring::default_provider().install_default();

    let mut request = url.into_client_request()?;
    let bearer = format!("Bearer {token}").parse()?;
    request.headers_mut().insert("Authorization", bearer);

    let mut socket = match tungstenite::connect(request) {
        Ok((socket, _)) => socket,
        Err(e) => return Err(e.into()),
    };

    let hello = GameMsg::Hello {
        proto: PROTO,
        layout: Layout::Landscape,
    };
    send_msg(&mut socket, &hello)?;

    match recv_msg(&mut socket)? {
        ToGame::Session { session, .. } => {
            println!("Relay session connected: {session}");
        }
        other => return Err(format!("Expected session msg: {other:?}").into()),
    }

    Ok(socket)
}

fn send_msg(socket: &mut Socket, msg: &GameMsg) -> Result<(), Box<dyn Error>> {
    socket.send(Message::text(serde_json::to_string(msg)?))?;
    Ok(())
}

fn recv_msg(socket: &mut Socket) -> Result<ToGame, Box<dyn Error>> {
    loop {
        match socket.read()? {
            Message::Text(json) => return Ok(serde_json::from_str(&json)?),
            Message::Close(frame) => return Err(format!("Socket closed: {frame:?}").into()),
            _ => {}
        }
    }
}

/// Reads a configuration variable from OS environment or from a local `.env` file.
fn load_env_var(key: &str) -> Option<String> {
    if let Ok(val) = env::var(key) {
        if !val.trim().is_empty() {
            return Some(val);
        }
    }

    if let Ok(content) = std::fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if k.trim() == key {
                    let val = v.trim().trim_matches('"').trim_matches('\'');
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Generates a QR code from a URL and returns RGBA pixel data.
/// Uses uppercase URL for a smaller (alphanumeric) QR code, matching the
/// reference architecture's approach.
fn generate_qr_image(url: &str) -> Option<QrImageData> {
    // Uppercase makes the QR code smaller (uses alphanumeric mode).
    let upper = url.to_uppercase();
    let code = QrCode::new(upper.as_bytes()).ok()?;
    let modules = code.to_colors();
    let module_count = code.width() as u32;

    // Scale each module to a few pixels so it's scannable on screen.
    let scale: u32 = 4;
    let border: u32 = 4; // quiet zone in modules
    let img_size = (module_count + border * 2) * scale;

    let mut rgba = vec![255u8; (img_size * img_size * 4) as usize];

    for row in 0..module_count {
        for col in 0..module_count {
            let dark = modules[(row * module_count + col) as usize]
                == qrcode::types::Color::Dark;
            if dark {
                let px = (col + border) * scale;
                let py = (row + border) * scale;
                for dy in 0..scale {
                    for dx in 0..scale {
                        let idx = ((py + dy) * img_size + (px + dx)) as usize * 4;
                        rgba[idx] = 0;     // R
                        rgba[idx + 1] = 0; // G
                        rgba[idx + 2] = 0; // B
                        // A stays 255
                    }
                }
            }
        }
    }

    Some(QrImageData {
        rgba,
        width: img_size,
    })
}

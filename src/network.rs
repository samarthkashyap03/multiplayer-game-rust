use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use tungstenite::client::IntoClientRequest;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use crate::protocol::{Button, GameMsg, Layout, PROTO, ToGame};

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    Connected { slot: String },
    Disconnected { slot: String },
    ButtonEdge { slot: String, button: Button, down: bool },
    StartGame,
}

pub struct NetworkManager {
    pub slot_links: HashMap<String, String>,
    pub rx: Receiver<NetworkEvent>,
    _stop_tx: Sender<()>,
}

impl NetworkManager {
    pub fn start() -> Result<Self, Box<dyn Error>> {
        let (event_tx, rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        // 1. Determine Relay URL and Token (from env or TOKEN.md defaults)
        let base_url = env::var("INPUT_MUX_URL")
            .unwrap_or_else(|_| "https://scilab-0098.informatik.uni-kl.de".to_string());
        let token = env::var("INPUT_MUX_TOKEN").unwrap_or_else(|_| {
            "imx_982b58a8_waq7hzro5vzprq6uhh52rmeiwzg5fdsx".to_string()
        });

        let mut socket = connect_relay(&base_url, &token)?;

        let slots = ["p1", "p2", "p3", "p4"];
        let names = [
            "P1: Barbarossa",
            "P2: Kaiserhecht",
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

        // 4. Spawn listener thread
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

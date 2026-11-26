mod bot;
mod constants;
mod history;
mod level;
mod mechanisms;
mod network;
mod player;
#[allow(dead_code)]
mod protocol;
mod render;

use std::collections::HashMap;

use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::texture::Texture2D;
use macroquad::time::get_frame_time;
use macroquad::window::{next_frame, Conf};

use constants::{
    CHECKPOINT_1_Y, CHECKPOINT_2_Y, CHECKPOINT_3_Y, SCREEN_HEIGHT, SCREEN_WIDTH,
    SECTION_1_START_Y, WORLD_BOTTOM_Y,
};
use history::get_checkpoint_quizzes;
use level::Level;
use network::{NetworkEvent, NetworkManager};
use player::Player;
use protocol::Button;

#[derive(Debug, PartialEq, Eq)]
enum GameState {
    Lobby,
    Playing,
    AtCheckpoint { checkpoint_index: usize },
    Victory,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "KaiserClimb - The Imperial Palace of Kaiserslautern".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::Lobby;
    let mut level = Level::build_kaiserpfalz();
    let quizzes = get_checkpoint_quizzes();

    // Initialize 4 players safely on the courtyard
    let slots = ["p1", "p2", "p3", "p4"];
    let mut players = Vec::new();
    for (i, &slot) in slots.iter().enumerate() {
        let spawn_x = 80.0 + (i as f32 * 60.0);
        players.push(Player::new(i, slot, spawn_x, SECTION_1_START_Y));
    }
    // TODO(multiplayer): For now, player 0 starts enabled so the game is
    // playable with just one phone (or keyboard-only). Update this once a
    // proper lobby "ready-up" flow is implemented.
    players[0].connected = true;

    // Start background network manager (connecting to relay or fallback)
    let net = match NetworkManager::start() {
        Ok(mgr) => Some(mgr),
        Err(e) => {
            eprintln!("Relay connection unavailable ({e}); running in local mode.");
            None
        }
    };

    // Build QR code textures from raw image data produced by the network manager
    let qr_textures: HashMap<String, Texture2D> = if let Some(ref net_mgr) = net {
        net_mgr
            .qr_images
            .iter()
            .map(|(slot, qr_data)| {
                let tex = Texture2D::from_rgba8(
                    qr_data.width as u16,
                    qr_data.width as u16,
                    &qr_data.rgba,
                );
                tex.set_filter(macroquad::texture::FilterMode::Nearest);
                (slot.clone(), tex)
            })
            .collect()
    } else {
        HashMap::new()
    };

    let mut camera_y = SECTION_1_START_Y - SCREEN_HEIGHT * 0.6;
    let mut current_section = 1;
    let mut checkpoint_timer = 15.0;
    let mut game_time = 0.0;

    loop {
        let dt = get_frame_time().min(0.05); // Cap delta time to prevent physics tunneling
        game_time += dt;

        // 1. Process Network Events from Smartphones
        if let Some(ref net_mgr) = net {
            for event in net_mgr.poll_events() {
                match event {
                    NetworkEvent::Connected { slot } => {
                        if let Some(p) = players.iter_mut().find(|p| p.slot == slot) {
                            p.connected = true;
                            p.phone_controlled = true;
                            p.is_bot = false;
                            p.disconnect_timer = None;
                        }
                    }
                    NetworkEvent::Disconnected { slot } => {
                        if let Some(p) = players.iter_mut().find(|p| p.slot == slot) {
                            p.connected = false;
                            p.phone_controlled = false;
                            p.disconnect_timer = Some(5.0);
                        }
                    }
                    NetworkEvent::StartGame => {
                        if state == GameState::Lobby {
                            state = GameState::Playing;
                        }
                    }
                    NetworkEvent::ButtonEdge { slot, button, down } => {
                        if let Some(idx) = players.iter().position(|p| p.slot == slot) {
                            handle_player_input(
                                idx,
                                button,
                                down,
                                &mut players,
                                &mut level,
                                &mut state,
                            );
                        }
                    }
                }
            }
        }

        // 2. Local Keyboard Controls (PC Fallback)
        handle_local_keyboard(&mut players, &mut level, &mut state);

        // 3. State Machine Logic
        match state {
            GameState::Lobby => {
                let links = net.as_ref().map(|n| &n.slot_links).cloned().unwrap_or_default();
                render::render_lobby(&players, &links, &qr_textures);

                // TODO(multiplayer): Currently starts with any number of players.
                // Add a minimum-player gate here once the lobby UI shows a
                // ready-up count (e.g., require at least 2 connected phones).
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                // Update players physics
                for p in &mut players {
                    if p.connected || p.is_bot {
                        p.try_attach_to_rope(&level.ropes);
                        p.update_physics(dt, &level.platforms, &level.ropes);

                        // Check hazard collisions (e.g., deep moat)
                        let pb = p.bounds();
                        for h in &level.hazards {
                            if pb.overlaps(h) {
                                p.respawn_to_safe();
                            }
                        }

                    }
                }

                // Update shared mechanisms
                for mech in &mut level.mechanisms {
                    // Check if players are still holding switch
                    if let Some(pa) = mech.player_a {
                        if players.get(pa).map(|p| p.holding_switch != Some(mech.id)).unwrap_or(true) {
                            mech.player_a = None;
                        }
                    }
                    if let Some(pb) = mech.player_b {
                        if players.get(pb).map(|p| p.holding_switch != Some(mech.id)).unwrap_or(true) {
                            mech.player_b = None;
                        }
                    }
                    mech.update(dt);
                }

                // Update carried objects
                for obj in &mut level.carry_objects {
                    if !obj.carried_by.is_empty() && !obj.is_placed {
                        // Position object at carrying player's position
                        let first_carrier = obj.carried_by[0];
                        if let Some(p) = players.get(first_carrier) {
                            obj.x = p.x + 10.0;
                            obj.y = p.y - 15.0;
                        }
                        obj.check_target_placement();
                    }
                }

                // Update bots for disconnected players
                bot::update_bots(&mut players, &mut level, current_section, dt);

                // Smooth camera following team upwards
                let active_ys: Vec<f32> = players
                    .iter()
                    .filter(|p| p.connected || p.is_bot)
                    .map(|p| p.y)
                    .collect();
                if !active_ys.is_empty() {
                    let avg_y = active_ys.iter().sum::<f32>() / active_ys.len() as f32;
                    let target_cam_y = (avg_y - SCREEN_HEIGHT * 0.55)
                        .clamp(0.0, WORLD_BOTTOM_Y - SCREEN_HEIGHT);
                    camera_y += (target_cam_y - camera_y) * 4.0 * dt;
                }

                // Check Section Progress and Checkpoint Triggers
                if current_section == 1 {
                    let team_at_gate = players
                        .iter()
                        .filter(|p| p.connected || p.is_bot)
                        .all(|p| p.y < CHECKPOINT_1_Y + 120.0);
                    if team_at_gate && level.mechanisms[0].is_open {
                        state = GameState::AtCheckpoint { checkpoint_index: 0 };
                        checkpoint_timer = 15.0;
                        current_section = 2;
                    }
                } else if current_section == 2 {
                    let team_at_gate = players
                        .iter()
                        .filter(|p| p.connected || p.is_bot)
                        .all(|p| p.y < CHECKPOINT_2_Y + 120.0);
                    if team_at_gate && level.mechanisms[1].is_open {
                        state = GameState::AtCheckpoint { checkpoint_index: 1 };
                        checkpoint_timer = 15.0;
                        current_section = 3;
                    }
                } else if current_section == 3 {
                    let team_at_summit = players
                        .iter()
                        .filter(|p| p.connected || p.is_bot)
                        .any(|p| p.y < CHECKPOINT_3_Y + 100.0);
                    if team_at_summit && level.mechanisms[2].is_open {
                        state = GameState::AtCheckpoint { checkpoint_index: 2 };
                        checkpoint_timer = 15.0;
                    }
                }

                // Render current game world
                render::render_world(&level, &players, camera_y, game_time, current_section);
            }

            GameState::AtCheckpoint { checkpoint_index } => {
                checkpoint_timer -= dt;
                let quiz = &quizzes[checkpoint_index];

                render::render_world(&level, &players, camera_y, game_time, current_section);
                render::render_checkpoint_modal(quiz, checkpoint_timer, &players);

                // Auto-advance after 15s or when answer is confirmed
                if checkpoint_timer <= 0.0 || is_key_pressed(KeyCode::Enter) {
                    if checkpoint_index == 2 {
                        state = GameState::Victory;
                    } else {
                        state = GameState::Playing;
                    }
                }
            }

            GameState::Victory => {
                render::render_victory_screen();
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    // Reset game
                    state = GameState::Lobby;
                    level = Level::build_kaiserpfalz();
                    current_section = 1;
                    camera_y = SECTION_1_START_Y - SCREEN_HEIGHT * 0.6;
                    for (i, p) in players.iter_mut().enumerate() {
                        let spawn_x = 220.0 + (i as f32 * 140.0);
                        p.x = spawn_x;
                        p.y = SECTION_1_START_Y;
                        p.vx = 0.0;
                        p.vy = 0.0;
                    }
                }
            }
        }

        next_frame().await;
    }
}

fn handle_player_input(
    player_idx: usize,
    button: Button,
    down: bool,
    players: &mut [Player],
    level: &mut Level,
    state: &mut GameState,
) {
    if let GameState::AtCheckpoint { .. } = state {
        if down {
            let choice = match button {
                Button::A => Some(0),
                Button::B => Some(1),
                Button::X => Some(2),
                Button::Y => Some(3),
                _ => None,
            };
            if let Some(c) = choice {
                if let Some(p) = players.get_mut(player_idx) {
                    p.current_answer = Some(c);
                }
            }
        }
        return;
    }

    let p = match players.get_mut(player_idx) {
        Some(p) => p,
        None => return,
    };

    match button {
        Button::Left => {
            if down {
                p.move_horizontal(-1.0);
            } else if p.vx < 0.0 {
                p.stop_horizontal();
            }
        }
        Button::Right => {
            if down {
                p.move_horizontal(1.0);
            } else if p.vx > 0.0 {
                p.stop_horizontal();
            }
        }
        Button::Up => {
            if down {
                p.climb(-1.0);
            } else if p.is_climbing && p.vy < 0.0 {
                p.stop_climbing();
            }
        }
        Button::Down => {
            if down {
                p.climb(1.0);
            } else if p.is_climbing && p.vy > 0.0 {
                p.stop_climbing();
            }
        }
        Button::A => {
            if down {
                p.jump();
            }
        }
        Button::B | Button::X => {
            if down {
                // 1. Check Rope Anchor toggle
                let pb = p.bounds();
                for rope in &mut level.ropes {
                    if pb.overlaps(&rope.anchor_bounds()) {
                        rope.is_lowered = !rope.is_lowered;
                        rope.activated_by = Some(player_idx);
                        if !rope.is_lowered && p.climbing_rope_id == Some(rope.id) {
                            p.detach_from_rope();
                        }
                        return;
                    }
                }

                // 2. Check Carry Object pickup / drop
                if let Some(obj_idx) = p.carrying_object {
                    if let Some(obj) = level.carry_objects.get_mut(obj_idx) {
                        obj.carried_by.retain(|&id| id != player_idx);
                    }
                    p.carrying_object = None;
                    return;
                } else {
                    for obj in &mut level.carry_objects {
                        if !obj.is_placed && pb.overlaps(&obj.bounds()) {
                            obj.carried_by.push(player_idx);
                            p.carrying_object = Some(obj.id);
                            return;
                        }
                    }
                }

                // 3. Check Shared Mechanism Lever
                for mech in &mut level.mechanisms {
                    if pb.overlaps(&mech.switch_a_bounds()) {
                        mech.player_a = Some(player_idx);
                        p.holding_switch = Some(mech.id);
                        return;
                    } else if pb.overlaps(&mech.switch_b_bounds()) {
                        mech.player_b = Some(player_idx);
                        p.holding_switch = Some(mech.id);
                        return;
                    }
                }
            } else {
                // Release switch
                if let Some(mech_id) = p.holding_switch {
                    if let Some(mech) = level.mechanisms.iter_mut().find(|m| m.id == mech_id) {
                        if mech.player_a == Some(player_idx) {
                            mech.player_a = None;
                        }
                        if mech.player_b == Some(player_idx) {
                            mech.player_b = None;
                        }
                    }
                    p.holding_switch = None;
                }
            }
        }
        _ => {}
    }
}

fn handle_local_keyboard(players: &mut [Player], level: &mut Level, state: &mut GameState) {
    if *state == GameState::Lobby || *state == GameState::Victory {
        return;
    }

    // Don't override phone-controlled players with keyboard input
    if players[0].phone_controlled {
        return;
    }

    if let GameState::AtCheckpoint { .. } = state {
        if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::A) {
            players[0].current_answer = Some(0);
        } else if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::B) {
            players[0].current_answer = Some(1);
        } else if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::X) {
            players[0].current_answer = Some(2);
        } else if is_key_pressed(KeyCode::Key4) || is_key_pressed(KeyCode::Y) {
            players[0].current_answer = Some(3);
        }
        return;
    }

    // Player 1 Keyboard Controls (WASD / Space / E)
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        players[0].move_horizontal(-1.0);
    } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        players[0].move_horizontal(1.0);
    } else if players[0].vx != 0.0 {
        players[0].stop_horizontal();
    }

    if players[0].is_climbing {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            players[0].climb(-1.0);
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            players[0].climb(1.0);
        } else {
            players[0].stop_climbing();
        }
    } else if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
        players[0].jump();
    }

    if is_key_pressed(KeyCode::E) {
        let pb = players[0].bounds();
        // Toggle rope
        for rope in &mut level.ropes {
            if pb.overlaps(&rope.anchor_bounds()) {
                rope.is_lowered = !rope.is_lowered;
                rope.activated_by = Some(0);
                if !rope.is_lowered && players[0].climbing_rope_id == Some(rope.id) {
                    players[0].detach_from_rope();
                }
            }
        }
        // Pickup/drop object
        if let Some(obj_idx) = players[0].carrying_object {
            if let Some(obj) = level.carry_objects.get_mut(obj_idx) {
                obj.carried_by.retain(|&id| id != 0);
            }
            players[0].carrying_object = None;
        } else {
            for obj in &mut level.carry_objects {
                if !obj.is_placed && pb.overlaps(&obj.bounds()) {
                    obj.carried_by.push(0);
                    players[0].carrying_object = Some(obj.id);
                    break;
                }
            }
        }
    }

    // Hold switch E
    if is_key_down(KeyCode::E) {
        let pb = players[0].bounds();
        for mech in &mut level.mechanisms {
            if pb.overlaps(&mech.switch_a_bounds()) {
                mech.player_a = Some(0);
                players[0].holding_switch = Some(mech.id);
            } else if pb.overlaps(&mech.switch_b_bounds()) {
                mech.player_b = Some(0);
                players[0].holding_switch = Some(mech.id);
            }
        }
    } else if let Some(mech_id) = players[0].holding_switch {
        if let Some(mech) = level.mechanisms.iter_mut().find(|m| m.id == mech_id) {
            if mech.player_a == Some(0) {
                mech.player_a = None;
            }
            if mech.player_b == Some(0) {
                mech.player_b = None;
            }
        }
        players[0].holding_switch = None;
    }
}

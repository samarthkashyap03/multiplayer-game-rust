use crate::level::Level;
use crate::player::Player;

pub fn update_bots(players: &mut [Player], level: &mut Level, current_section: usize, dt: f32) {
    for i in 0..players.len() {
        // Handle disconnect timer (5-second grace window only for players that were previously connected)
        if !players[i].connected && !players[i].is_bot {
            if let Some(ref mut timer) = players[i].disconnect_timer {
                *timer -= dt;
                if *timer <= 0.0 {
                    players[i].is_bot = true;
                    players[i].disconnect_timer = None;
                }
            }
            continue;
        }

        if !players[i].is_bot {
            continue;
        }

        // Automated cooperative bot logic
        let mut target_x = players[i].x;
        let mut should_jump = false;
        let mut should_interact = false;

        // 1. Check if a human teammate is operating Switch A on a shared mechanism in current section
        let mech_idx = (current_section - 1).min(level.mechanisms.len().saturating_sub(1));
        if let Some(mech) = level.mechanisms.get(mech_idx) {
            if !mech.is_open {
                // If another player is at switch A, bot heads to switch B!
                if mech.player_a.is_some() && mech.player_a != Some(i) {
                    target_x = mech.switch_b.0;
                    if (players[i].x - mech.switch_b.0).abs() < 25.0 {
                        should_interact = true;
                    }
                } else if mech.player_b.is_some() && mech.player_b != Some(i) {
                    target_x = mech.switch_a.0;
                    if (players[i].x - mech.switch_a.0).abs() < 25.0 {
                        should_interact = true;
                    }
                }
            }
        }

        // 2. Check if a rope is lowered nearby and bot needs to climb
        for rope in &level.ropes {
            if rope.is_lowered {
                let dx = (players[i].x - rope.x).abs();
                if dx < 35.0 && players[i].y > rope.y && players[i].y < rope.y + rope.length + 30.0
                {
                    players[i].is_climbing = true;
                    players[i].climbing_rope_id = Some(rope.id);
                    players[i].climb(-1.0); // climb upward
                }
            }
        }

        // If climbing and reached the top of the rope, dismount
        if players[i].is_climbing {
            if let Some(rope_id) = players[i].climbing_rope_id {
                if let Some(rope) = level.ropes.iter().find(|r| r.id == rope_id) {
                    if players[i].y <= rope.y + 10.0 {
                        players[i].is_climbing = false;
                        players[i].climbing_rope_id = None;
                        players[i].jump();
                    }
                }
            }
            continue;
        }

        // 3. Check if unplaced objects need carrying in current section
        for obj in &level.carry_objects {
            if obj.section == current_section && !obj.is_placed {
                if (players[i].x - obj.x).abs() < 30.0 && (players[i].y - obj.y).abs() < 40.0 {
                    should_interact = true;
                    target_x = obj.target_x;
                } else if obj.carried_by.contains(&i) {
                    target_x = obj.target_x;
                    should_interact = true;
                }
                break;
            }
        }

        // Move towards target
        let diff = target_x - players[i].x;
        if diff.abs() > 15.0 {
            players[i].move_horizontal(diff.signum());
            // If blocked against a wall/ledge while moving towards target, jump over it!
            if players[i].on_ground && players[i].vx == 0.0 {
                should_jump = true;
            }
        } else {
            players[i].stop_horizontal();
        }

        // Jump if near platform edge or obstacle
        if should_jump && players[i].on_ground {
            players[i].jump();
        }

        // Apply interaction for mechanism
        if should_interact {
            let mech_idx = (current_section - 1).min(level.mechanisms.len().saturating_sub(1));
            if let Some(mech) = level.mechanisms.get_mut(mech_idx) {
                if mech.switch_a_bounds().overlaps(&players[i].bounds()) {
                    mech.player_a = Some(i);
                    players[i].holding_switch = Some(mech.id);
                } else if mech.switch_b_bounds().overlaps(&players[i].bounds()) {
                    mech.player_b = Some(i);
                    players[i].holding_switch = Some(mech.id);
                }
            }
        }
    }
}

use macroquad::color::{Color, BLACK, GRAY, GREEN, RED, WHITE, YELLOW};
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle, draw_rectangle_lines};
use macroquad::text::{draw_text, measure_text};

use crate::constants::{
    GOLD_ACCENT, PARCHMENT, RED_SANDSTONE, RED_SANDSTONE_DARK, RED_SANDSTONE_LIGHT,
    SCREEN_HEIGHT, SCREEN_WIDTH, WATER_BLUE, WOOD_BROWN, WOOD_LIGHT,
};
use crate::history::CheckpointQuiz;
use crate::level::Level;
use crate::player::Player;

pub fn render_world(
    level: &Level,
    players: &[Player],
    camera_y: f32,
    game_time: f32,
    current_section: usize,
) {
    // 1. Background (Medieval Castle Stone & Parallax)
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.12, 0.10, 0.14, 1.0));

    // Decorative wall arches in background
    let bg_offset = (camera_y * 0.3) % 200.0;
    for y in (-1..=4).map(|i| i as f32 * 200.0 - bg_offset) {
        for x in (0..6).map(|i| i as f32 * 220.0 + 30.0) {
            draw_rectangle_lines(
                x,
                y,
                180.0,
                160.0,
                2.0,
                Color::new(0.20, 0.16, 0.22, 0.4),
            );
        }
    }

    // 2. Render Level Platforms & Sandstone Blocks
    for plat in &level.platforms {
        let screen_y = plat.y - camera_y;
        if screen_y + plat.h < -50.0 || screen_y > SCREEN_HEIGHT + 50.0 {
            continue;
        }

        // Draw sandstone base
        draw_rectangle(plat.x, screen_y, plat.w, plat.h, RED_SANDSTONE);
        draw_rectangle(plat.x, screen_y, plat.w, 4.0, RED_SANDSTONE_LIGHT);
        draw_rectangle(plat.x, screen_y + plat.h - 4.0, plat.w, 4.0, RED_SANDSTONE_DARK);

        // Brick pattern lines
        let brick_w = 40.0;
        let mut bx = plat.x + 10.0;
        while bx < plat.x + plat.w - 10.0 {
            draw_line(bx, screen_y, bx, screen_y + plat.h, 1.5, RED_SANDSTONE_DARK);
            bx += brick_w;
        }
        draw_rectangle_lines(plat.x, screen_y, plat.w, plat.h, 2.0, RED_SANDSTONE_DARK);
    }

    // 3. Render Hazards (Kaiserwoog Fishpond Water)
    for hazard in &level.hazards {
        let screen_y = hazard.y - camera_y;
        if screen_y + hazard.h < -50.0 || screen_y > SCREEN_HEIGHT + 50.0 {
            continue;
        }

        draw_rectangle(hazard.x, screen_y, hazard.w, hazard.h, WATER_BLUE);

        // Animated wave ripples
        let wave_y = screen_y + 6.0 + (game_time * 3.0).sin() * 3.0;
        draw_line(hazard.x, wave_y, hazard.x + hazard.w, wave_y, 2.5, Color::new(0.4, 0.7, 0.95, 0.9));

        // Swimming Kaiserhecht fish
        let fish_x = hazard.x + 40.0 + ((game_time * 40.0) % (hazard.w - 80.0));
        let fish_y = screen_y + 24.0 + (game_time * 2.0).sin() * 6.0;
        draw_rectangle(fish_x, fish_y, 16.0, 7.0, Color::new(0.8, 0.85, 0.9, 0.9));
        draw_circle(fish_x + 14.0, fish_y + 3.5, 3.0, GOLD_ACCENT); // Golden ring!
    }

    // 4. Render Ropes
    for rope in &level.ropes {
        let screen_y = rope.y - camera_y;
        // Anchor winch
        draw_rectangle(rope.x - 14.0, screen_y - 12.0, 28.0, 16.0, WOOD_BROWN);
        draw_circle(rope.x, screen_y - 4.0, 5.0, GOLD_ACCENT);

        if rope.is_lowered {
            // Taut hanging rope
            let rope_end_y = screen_y + rope.length;
            draw_line(rope.x, screen_y, rope.x, rope_end_y, 4.0, WOOD_LIGHT);
            draw_circle(rope.x, rope_end_y, 5.0, WOOD_BROWN); // Knot
        } else {
            // Coiled rope on anchor
            draw_circle(rope.x, screen_y + 4.0, 7.0, WOOD_LIGHT);
        }
    }

    // 5. Render Shared Mechanisms & Portcullis Gates
    for mech in &level.mechanisms {
        let sy_a = mech.switch_a.1 - camera_y;
        let sy_b = mech.switch_b.1 - camera_y;

        // Switch A
        let color_a = if mech.player_a.is_some() { GREEN } else { GOLD_ACCENT };
        draw_rectangle(mech.switch_a.0 - 12.0, sy_a - 14.0, 24.0, 20.0, WOOD_BROWN);
        draw_circle(mech.switch_a.0, sy_a - 4.0, 6.0, color_a);
        draw_text("A", mech.switch_a.0 - 4.0, sy_a + 22.0, 18.0, WHITE);

        // Switch B
        let color_b = if mech.player_b.is_some() { GREEN } else { GOLD_ACCENT };
        draw_rectangle(mech.switch_b.0 - 12.0, sy_b - 14.0, 24.0, 20.0, WOOD_BROWN);
        draw_circle(mech.switch_b.0, sy_b - 4.0, 6.0, color_b);
        draw_text("B", mech.switch_b.0 - 4.0, sy_b + 22.0, 18.0, WHITE);

        // Progress bar between switches if being pressed
        if mech.activation_progress > 0.0 && !mech.is_open {
            let bar_w = 120.0;
            let bar_x = (mech.switch_a.0 + mech.switch_b.0) / 2.0 - bar_w / 2.0;
            let bar_y = sy_a - 30.0;
            draw_rectangle(bar_x, bar_y, bar_w, 10.0, GRAY);
            draw_rectangle(bar_x, bar_y, bar_w * mech.activation_progress, 10.0, GREEN);
        }

        // Portcullis Gate
        let gate_sy = mech.door_rect.y - camera_y;
        draw_rectangle(
            mech.door_rect.x,
            gate_sy,
            mech.door_rect.w,
            mech.door_rect.h,
            Color::new(0.2, 0.2, 0.25, 0.95),
        );
        // Vertical iron bars
        let mut bar_x = mech.door_rect.x + 8.0;
        while bar_x < mech.door_rect.x + mech.door_rect.w - 4.0 {
            draw_line(bar_x, gate_sy, bar_x, gate_sy + mech.door_rect.h, 3.0, Color::new(0.4, 0.4, 0.45, 1.0));
            bar_x += 16.0;
        }
    }

    // 6. Render Carry Objects & Target Sockets
    for obj in &level.carry_objects {
        // Target socket
        let target_sy = obj.target_y - camera_y;
        let socket_color = if obj.is_placed { GREEN } else { Color::new(0.9, 0.8, 0.3, 0.5) };
        draw_rectangle_lines(obj.target_x, target_sy, obj.width, obj.height, 2.5, socket_color);

        // Object itself
        let obj_sy = obj.y - camera_y;
        let color = if obj.is_placed {
            GREEN
        } else if !obj.carried_by.is_empty() {
            GOLD_ACCENT
        } else {
            PARCHMENT
        };

        draw_rectangle(obj.x, obj_sy, obj.width, obj.height, color);
        draw_rectangle_lines(obj.x, obj_sy, obj.width, obj.height, 2.0, BLACK);
        draw_text(&format!("#{}", obj.id + 1), obj.x + 8.0, obj_sy + 24.0, 18.0, BLACK);
    }

    // 7. Render Players
    for p in players {
        if !p.connected && !p.is_bot {
            continue;
        }

        let psy = p.y - camera_y;

        // Player Shadow
        draw_circle(p.x + 15.0, psy + 44.0, 10.0, Color::new(0.0, 0.0, 0.0, 0.35));

        // Player Body / Tunic
        draw_rectangle(p.x + 4.0, psy + 14.0, 22.0, 24.0, p.color);
        draw_rectangle_lines(p.x + 4.0, psy + 14.0, 22.0, 24.0, 1.5, BLACK);

        // Head
        draw_circle(p.x + 15.0, psy + 10.0, 8.0, Color::new(0.95, 0.82, 0.70, 1.0));

        // Hat / Crown (Medieval Staufer Cap)
        let hat_color = if p.id == 0 { GOLD_ACCENT } else { p.color };
        draw_rectangle(p.x + 8.0, psy + 2.0, 14.0, 6.0, hat_color);

        // Eyes (direction facing)
        let eye_offset = if p.facing_right { 3.0 } else { -3.0 };
        draw_circle(p.x + 15.0 + eye_offset, psy + 9.0, 1.5, BLACK);

        if p.is_climbing {
            let climb_motion = if p.vy.abs() > 1.0 {
                (p.anim_timer * 10.0).sin() * 5.0
            } else {
                0.0
            };
            draw_line(p.x + 10.0, psy + 38.0, p.x + 10.0 + climb_motion, psy + 46.0, 3.0, BLACK);
            draw_line(p.x + 20.0, psy + 38.0, p.x + 20.0 - climb_motion, psy + 46.0, 3.0, BLACK);
            draw_line(p.x + 5.0, psy + 20.0, p.x + 14.0, psy + 12.0 + climb_motion, 3.0, BLACK);
            draw_line(p.x + 25.0, psy + 20.0, p.x + 16.0, psy + 12.0 - climb_motion, 3.0, BLACK);
        } else {
            let leg_offset = if p.vx.abs() > 10.0 {
                (p.anim_timer * 14.0).sin() * 5.0
            } else {
                0.0
            };
            draw_line(p.x + 10.0, psy + 38.0, p.x + 10.0 + leg_offset, psy + 46.0, 3.0, BLACK);
            draw_line(p.x + 20.0, psy + 38.0, p.x + 20.0 - leg_offset, psy + 46.0, 3.0, BLACK);
        }

        // Name Tag & BOT Badge
        let tag = if p.is_bot {
            format!("{} [BOT]", p.name)
        } else {
            p.name.clone()
        };
        draw_text(&tag, p.x - 10.0, psy - 6.0, 16.0, WHITE);
    }

    // 8. HUD Overlay (Fixed Screen Space)
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, 48.0, Color::new(0.08, 0.06, 0.10, 0.85));
    draw_line(0.0, 48.0, SCREEN_WIDTH, 48.0, 2.0, GOLD_ACCENT);

    let section_title = match current_section {
        1 => "ABSCHNITT 1: DIE PALASTANLAGE (PALACE GROUNDS)",
        2 => "ABSCHNITT 2: DER GROSSE DOMUS (25m x 19m x 19m)",
        _ => "ABSCHNITT 3: DIE SPÄTERE DOPPELKAPELLE (FRIEDRICH II.)",
    };
    draw_text(section_title, 24.0, 32.0, 22.0, GOLD_ACCENT);

    // Player Status in HUD
    for (i, p) in players.iter().enumerate() {
        let hx = SCREEN_WIDTH - 440.0 + (i as f32 * 105.0);
        let status_color = if p.connected {
            GREEN
        } else if p.is_bot {
            YELLOW
        } else {
            GRAY
        };
        draw_circle(hx, 24.0, 6.0, status_color);
        draw_text(&format!("P{}", i + 1), hx + 10.0, 30.0, 18.0, WHITE);
    }
}

pub fn render_checkpoint_modal(quiz: &CheckpointQuiz, time_left: f32, _players: &[Player]) {
    // Dim background
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.75));

    // Parchment Modal Card
    let mw = 840.0;
    let mh = 560.0;
    let mx = (SCREEN_WIDTH - mw) / 2.0;
    let my = (SCREEN_HEIGHT - mh) / 2.0;

    draw_rectangle(mx, my, mw, mh, PARCHMENT);
    draw_rectangle_lines(mx, my, mw, mh, 5.0, RED_SANDSTONE_DARK);

    // Header banner
    draw_rectangle(mx + 20.0, my + 20.0, mw - 40.0, 50.0, RED_SANDSTONE);
    draw_text(quiz.title, mx + 40.0, my + 52.0, 26.0, GOLD_ACCENT);

    // Explanation text
    draw_text(&format!("📜 {}", quiz.subtitle), mx + 40.0, my + 105.0, 22.0, RED_SANDSTONE_DARK);

    // Wrapped narrative explanation
    let mut ey = my + 135.0;
    for line in wrap_string(quiz.historical_explanation, 70) {
        draw_text(&line, mx + 40.0, ey, 18.0, BLACK);
        ey += 24.0;
    }

    // Question Box
    draw_rectangle(mx + 30.0, ey + 10.0, mw - 60.0, 44.0, Color::new(0.90, 0.85, 0.75, 1.0));
    draw_rectangle_lines(mx + 30.0, ey + 10.0, mw - 60.0, 44.0, 2.0, RED_SANDSTONE);
    draw_text(quiz.question, mx + 45.0, ey + 38.0, 20.0, BLACK);

    // 4 Answer Options (A, B, X, Y matching phone controller)
    let btn_colors = [RED, Color::new(0.2, 0.6, 0.9, 1.0), YELLOW, GREEN];
    let btn_labels = ["A", "B", "X", "Y"];

    let mut opt_y = ey + 75.0;
    for i in 0..4 {
        draw_rectangle(mx + 40.0, opt_y, mw - 80.0, 36.0, Color::new(1.0, 1.0, 1.0, 0.9));
        draw_rectangle(mx + 40.0, opt_y, 40.0, 36.0, btn_colors[i]);
        draw_text(btn_labels[i], mx + 52.0, opt_y + 26.0, 22.0, WHITE);
        draw_text(quiz.options[i], mx + 95.0, opt_y + 24.0, 17.0, BLACK);
        opt_y += 44.0;
    }

    // Countdown Progress Bar (15s timer)
    let bar_w = (mw - 80.0) * (time_left / 15.0).clamp(0.0, 1.0);
    draw_rectangle(mx + 40.0, my + mh - 55.0, mw - 80.0, 12.0, GRAY);
    draw_rectangle(mx + 40.0, my + mh - 55.0, bar_w, 12.0, RED_SANDSTONE);
    draw_text(
        &format!("⏳ Verbleibende Zeit: {:.1}s  |  💡 {}", time_left, quiz.did_you_know),
        mx + 40.0,
        my + mh - 64.0,
        15.0,
        BLACK,
    );
}

pub fn render_lobby(players: &[Player], links: &std::collections::HashMap<String, String>) {
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.12, 0.08, 0.14, 1.0));

    // Title Banner
    let title = "🏰 KAISERCLIMB: DIE KAISERPFALZ KAISERSLAUTERN 🏰";
    let tw = measure_text(title, None, 36, 1.0).width;
    draw_text(title, (SCREEN_WIDTH - tw) / 2.0, 80.0, 36.0, GOLD_ACCENT);

    let sub = "Kooperatives 2D-Multiplayer-Kletterabenteuer (1–4 Spieler)";
    let sw = measure_text(sub, None, 22, 1.0).width;
    draw_text(sub, (SCREEN_WIDTH - sw) / 2.0, 120.0, 22.0, PARCHMENT);

    // 4 Player Slots & QR Connection Panels
    for (i, p) in players.iter().enumerate() {
        let px = 60.0 + (i as f32 * 295.0);
        let py = 170.0;
        let pw = 270.0;
        let ph = 420.0;

        draw_rectangle(px, py, pw, ph, Color::new(0.20, 0.16, 0.22, 0.95));
        draw_rectangle_lines(px, py, pw, ph, 2.5, p.color);

        // Header
        draw_rectangle(px, py, pw, 45.0, p.color);
        draw_text(&p.name, px + 15.0, py + 30.0, 22.0, WHITE);

        // Connection Status
        let (status_text, status_col) = if p.connected {
            ("VERBUNDEN ✓", GREEN)
        } else {
            ("WARTE AUF SCAN...", YELLOW)
        };
        draw_text(status_text, px + 20.0, py + 80.0, 18.0, status_col);

        // QR Code / Link URL
        if let Some(link) = links.get(&p.slot) {
            draw_text("Scanne mit Handy:", px + 20.0, py + 120.0, 16.0, PARCHMENT);

            // Draw link string
            let short_link = if link.len() > 32 { &link[..32] } else { link };
            draw_text(short_link, px + 10.0, py + 145.0, 13.0, GOLD_ACCENT);

            // Render pseudo QR grid preview or instruction box
            draw_rectangle(px + 45.0, py + 175.0, 180.0, 180.0, WHITE);
            draw_rectangle_lines(px + 45.0, py + 175.0, 180.0, 180.0, 3.0, BLACK);
            draw_text("📱 CONTROLLER", px + 65.0, py + 265.0, 18.0, BLACK);
            draw_text(&format!("Slot: {}", p.slot), px + 95.0, py + 295.0, 18.0, RED_SANDSTONE);
        } else {
            draw_text("Relay Link bereit", px + 20.0, py + 140.0, 16.0, GRAY);
            draw_text("(Tastatur-Modus aktiv)", px + 20.0, py + 165.0, 14.0, WHITE);
        }

        // Controller hints
        draw_text("Steuerung am Smartphone:", px + 20.0, py + 380.0, 14.0, GRAY);
        draw_text("D-Pad / A / B / X / Menu", px + 20.0, py + 400.0, 14.0, WHITE);
    }

    // Start instructions
    let prompt = "Drücke [ Menu ] am Smartphone oder [ Enter / Leertaste ] am PC zum Starten!";
    let pw = measure_text(prompt, None, 22, 1.0).width;
    draw_rectangle(
        (SCREEN_WIDTH - pw) / 2.0 - 20.0,
        630.0,
        pw + 40.0,
        45.0,
        RED_SANDSTONE,
    );
    draw_text(prompt, (SCREEN_WIDTH - pw) / 2.0, 660.0, 22.0, WHITE);
}

pub fn render_victory_screen() {
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.12, 0.08, 0.14, 0.98));

    let title = "🎉 DIE KAISERPFALZ WURDE ERFOLGREICH ERKUNDET & REKONSTRUIERT! 🎉";
    let tw = measure_text(title, None, 28, 1.0).width;
    draw_text(title, (SCREEN_WIDTH - tw) / 2.0, 80.0, 28.0, GOLD_ACCENT);

    // Architectural Reconstruction Graphic
    let rx = (SCREEN_WIDTH - 640.0) / 2.0;
    let ry = 130.0;
    draw_rectangle(rx, ry, 640.0, 300.0, PARCHMENT);
    draw_rectangle_lines(rx, ry, 640.0, 300.0, 4.0, RED_SANDSTONE);

    // Illustration: The Domus (25m x 19m x 19m) and Chapel
    draw_rectangle(rx + 160.0, ry + 80.0, 320.0, 180.0, RED_SANDSTONE);
    draw_rectangle(rx + 140.0, ry + 60.0, 360.0, 20.0, RED_SANDSTONE_DARK); // Roof cornice
    draw_rectangle(rx + 280.0, ry + 170.0, 80.0, 90.0, WOOD_BROWN); // Great portal

    // Twin Chapel Towers
    draw_rectangle(rx + 80.0, ry + 40.0, 60.0, 220.0, RED_SANDSTONE_LIGHT);
    draw_rectangle(rx + 500.0, ry + 40.0, 60.0, 220.0, RED_SANDSTONE_LIGHT);

    draw_text("HISTORISCHE REKONSTRUKTION (12. & 13. JAHRHUNDERT)", rx + 75.0, ry + 35.0, 18.0, RED_SANDSTONE_DARK);
    draw_text("Domus: ca. 25m Länge • 19m Breite • 19m Höhe", rx + 130.0, ry + 285.0, 16.0, BLACK);

    // Summary Badges
    draw_rectangle(rx, ry + 320.0, 640.0, 150.0, Color::new(0.2, 0.16, 0.22, 0.9));
    draw_text("HISTORISCHE MEILENSTEINE:", rx + 30.0, ry + 355.0, 20.0, GOLD_ACCENT);
    draw_text("✓ 1. Palastanlage: Rahewin's Chronik vom staufischen Wildpark & See", rx + 30.0, ry + 385.0, 17.0, WHITE);
    draw_text("✓ 2. Der Domus: Zentrales kaiserliches Repräsentationsgebäude Barbarossas", rx + 30.0, ry + 415.0, 17.0, WHITE);
    draw_text("✓ 3. Spätere Doppelkapelle: Bauphase unter Kaiser Friedrich II. vollendet", rx + 30.0, ry + 445.0, 17.0, WHITE);

    let return_prompt = "Drücke [ Enter ] oder [ Menu ], um zur Lobby zurückzukehren";
    let rpw = measure_text(return_prompt, None, 20, 1.0).width;
    draw_text(return_prompt, (SCREEN_WIDTH - rpw) / 2.0, 660.0, 20.0, PARCHMENT);
}

fn wrap_string(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if cur.is_empty() {
            cur.push_str(word);
        } else if cur.len() + 1 + word.len() <= max_chars {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(cur);
            cur = word.to_string();
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

use macroquad::color::Color;
use macroquad::math::Rect;

use crate::constants::{
    CLIMB_SPEED, GRAVITY, JUMP_VELOCITY, MOVE_SPEED, PLAYER_COLORS, PLAYER_HEIGHT, PLAYER_NAMES,
    PLAYER_WIDTH,
};
use crate::mechanisms::RopeAnchor;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub slot: String,
    pub name: String,
    pub color: Color,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing_right: bool,
    pub on_ground: bool,
    pub is_climbing: bool,
    pub climbing_rope_id: Option<usize>,
    pub connected: bool,
    /// True when this player is being controlled by a phone via the relay.
    /// Prevents the local keyboard handler from overriding phone input.
    pub phone_controlled: bool,
    pub is_bot: bool,
    pub disconnect_timer: Option<f32>,
    pub last_safe_pos: (f32, f32),
    pub carrying_object: Option<usize>,
    pub holding_switch: Option<usize>,
    pub current_answer: Option<usize>,
    pub anim_timer: f32,
}

impl Player {
    pub fn new(id: usize, slot: &str, start_x: f32, start_y: f32) -> Self {
        Self {
            id,
            slot: slot.to_string(),
            name: PLAYER_NAMES[id % 4].to_string(),
            color: PLAYER_COLORS[id % 4],
            x: start_x,
            y: start_y,
            vx: 0.0,
            vy: 0.0,
            facing_right: true,
            on_ground: false,
            is_climbing: false,
            climbing_rope_id: None,
            connected: false,
            phone_controlled: false,
            is_bot: false,
            disconnect_timer: None,
            last_safe_pos: (start_x, start_y),
            carrying_object: None,
            holding_switch: None,
            current_answer: None,
            anim_timer: 0.0,
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT)
    }

    pub fn move_horizontal(&mut self, dir: f32) {
        if self.is_climbing {
            return;
        }
        self.vx = dir * MOVE_SPEED;
        if dir > 0.0 {
            self.facing_right = true;
        } else if dir < 0.0 {
            self.facing_right = false;
        }
    }

    pub fn stop_horizontal(&mut self) {
        self.vx = 0.0;
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.vy = JUMP_VELOCITY;
            self.on_ground = false;
            self.is_climbing = false;
            self.climbing_rope_id = None;
        } else if self.is_climbing {
            // Jump off rope
            self.vy = JUMP_VELOCITY * 0.8;
            self.is_climbing = false;
            self.climbing_rope_id = None;
        }
    }

    pub fn climb(&mut self, dir: f32) {
        if self.is_climbing {
            self.vy = dir * CLIMB_SPEED;
            self.vx = 0.0;
        }
    }

    pub fn stop_climbing(&mut self) {
        if self.is_climbing {
            self.vy = 0.0;
        }
    }

    pub fn detach_from_rope(&mut self) {
        self.is_climbing = false;
        self.climbing_rope_id = None;
        self.vy = 0.0;
    }

    pub fn try_attach_to_rope(&mut self, ropes: &[RopeAnchor]) {
        if self.is_climbing {
            return;
        }

        if let Some(rope) = ropes
            .iter()
            .find(|rope| rope.is_lowered && self.bounds().overlaps(&rope.rope_bounds()))
        {
            self.x = rope.x - PLAYER_WIDTH / 2.0;
            self.vx = 0.0;
            self.vy = 0.0;
            self.on_ground = false;
            self.is_climbing = true;
            self.climbing_rope_id = Some(rope.id);
        }
    }

    pub fn update_physics(&mut self, dt: f32, platforms: &[Rect], ropes: &[RopeAnchor]) {
        self.anim_timer += dt;

        if self.is_climbing {
            let rope = ropes
                .iter()
                .find(|rope| Some(rope.id) == self.climbing_rope_id && rope.is_lowered);
            if let Some(rope) = rope {
                self.update_rope_physics(dt, platforms, rope);
                return;
            }
            self.detach_from_rope();
        }

        // Apply gravity
        self.vy += GRAVITY * dt;
        if self.vy > 800.0 {
            self.vy = 800.0;
        }

        // Horizontal movement & collision
        self.x += self.vx * dt;
        self.resolve_horizontal_collisions(platforms);

        // Vertical movement & collision
        self.on_ground = false;
        self.y += self.vy * dt;
        self.resolve_vertical_collisions(platforms);

        if self.on_ground {
            self.last_safe_pos = (self.x, self.y);
        }
    }

    fn update_rope_physics(&mut self, dt: f32, platforms: &[Rect], rope: &RopeAnchor) {
        self.on_ground = false;
        let top_y = rope.y - PLAYER_HEIGHT;
        let bottom_y = (rope.y + rope.length - PLAYER_HEIGHT).max(top_y);
        let next_y = (self.y + self.vy * dt).clamp(top_y, bottom_y);

        if self.vy < 0.0 && next_y <= top_y {
            self.y = top_y;
            self.detach_from_rope();
            self.on_ground = true;
            self.last_safe_pos = (self.x, self.y);
            return;
        }

        for platform in platforms {
            let overlaps_horizontally =
                self.x + PLAYER_WIDTH > platform.x && self.x < platform.x + platform.w;
            if !overlaps_horizontally {
                continue;
            }

            if self.vy > 0.0
                && self.y + PLAYER_HEIGHT <= platform.y
                && next_y + PLAYER_HEIGHT >= platform.y
            {
                self.y = platform.y - PLAYER_HEIGHT;
                self.detach_from_rope();
                self.on_ground = true;
                self.last_safe_pos = (self.x, self.y);
                return;
            }

            if self.vy < 0.0
                && self.y >= platform.y + platform.h
                && next_y < platform.y + platform.h
            {
                self.y = platform.y + platform.h;
                self.vy = 0.0;
                return;
            }
        }

        self.y = next_y;
        if self.y >= bottom_y {
            self.vy = 0.0;
        }
    }

    fn resolve_horizontal_collisions(&mut self, platforms: &[Rect]) {
        // Inset vertically by 6.0 pixels from the top and bottom so the player's torso
        // collides with walls, but the floor underneath or ceiling above NEVER registers as a wall!
        let horiz_rect = Rect::new(self.x, self.y + 6.0, PLAYER_WIDTH, PLAYER_HEIGHT - 12.0);
        for plat in platforms {
            if horiz_rect.overlaps(plat) {
                if self.vx > 0.0 {
                    self.x = plat.x - PLAYER_WIDTH;
                } else if self.vx < 0.0 {
                    self.x = plat.x + plat.w;
                }
                self.vx = 0.0;
                break;
            }
        }
    }

    fn resolve_vertical_collisions(&mut self, platforms: &[Rect]) {
        // Inset horizontally by 2.0 pixels on the sides so walls don't register as floor/ceiling
        let vert_rect = Rect::new(self.x + 2.0, self.y, PLAYER_WIDTH - 4.0, PLAYER_HEIGHT);
        for plat in platforms {
            if vert_rect.overlaps(plat) {
                if self.vy > 0.0 {
                    // Falling onto platform
                    self.y = plat.y - PLAYER_HEIGHT;
                    self.vy = 0.0;
                    self.on_ground = true;
                } else if self.vy < 0.0 {
                    // Hitting head on ceiling
                    self.y = plat.y + plat.h;
                    self.vy = 0.0;
                }
            }
        }
    }

    pub fn respawn_to_safe(&mut self) {
        self.x = self.last_safe_pos.0;
        self.y = self.last_safe_pos.1;
        self.vx = 0.0;
        self.vy = 0.0;
        self.is_climbing = false;
        self.climbing_rope_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climbing_down_lands_on_platform_instead_of_passing_through() {
        let mut player = Player::new(0, "p1", 485.0, 120.0);
        let rope = RopeAnchor {
            id: 0,
            x: 500.0,
            y: 100.0,
            length: 200.0,
            is_lowered: true,
            activated_by: None,
        };
        let platform = Rect::new(450.0, 180.0, 100.0, 20.0);
        player.is_climbing = true;
        player.climbing_rope_id = Some(rope.id);
        player.vy = CLIMB_SPEED;

        player.update_physics(0.1, &[platform], &[rope]);

        assert_eq!(player.y, 180.0 - PLAYER_HEIGHT);
        assert!(player.on_ground);
        assert!(!player.is_climbing);
    }

    #[test]
    fn released_climb_input_stops_motion_without_leaving_rope() {
        let mut player = Player::new(0, "p1", 485.0, 120.0);
        player.is_climbing = true;
        player.climbing_rope_id = Some(0);
        player.vy = CLIMB_SPEED;
        player.stop_climbing();

        let rope = RopeAnchor {
            id: 0,
            x: 500.0,
            y: 100.0,
            length: 200.0,
            is_lowered: true,
            activated_by: None,
        };
        player.update_physics(0.1, &[], &[rope]);

        assert_eq!(player.y, 120.0);
        assert!(player.is_climbing);
        assert_eq!(player.vy, 0.0);
    }

    #[test]
    fn climbing_up_stops_at_rope_top() {
        let rope = RopeAnchor {
            id: 0,
            x: 500.0,
            y: 100.0,
            length: 200.0,
            is_lowered: true,
            activated_by: None,
        };
        let mut player = Player::new(0, "p1", 485.0, 60.0);
        player.is_climbing = true;
        player.climbing_rope_id = Some(rope.id);
        player.vy = -CLIMB_SPEED;

        player.update_physics(0.1, &[], &[rope]);

        assert_eq!(player.y, 100.0 - PLAYER_HEIGHT);
        assert!(!player.is_climbing);
    }
}

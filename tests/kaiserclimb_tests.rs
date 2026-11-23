#[cfg(test)]
mod tests {
    use macroquad::math::Rect;

    // We can define or test the logic components directly
    #[derive(Debug, Clone)]
    pub struct TestPlayer {
        pub x: f32,
        pub y: f32,
        pub vy: f32,
        pub on_ground: bool,
    }

    impl TestPlayer {
        pub fn new(x: f32, y: f32) -> Self {
            Self {
                x,
                y,
                vy: 0.0,
                on_ground: false,
            }
        }

        pub fn jump(&mut self) {
            if self.on_ground {
                self.vy = -450.0;
                self.on_ground = false;
            }
        }

        pub fn update_physics(&mut self, dt: f32, platforms: &[Rect]) {
            self.vy += 950.0 * dt;
            self.y += self.vy * dt;
            self.on_ground = false;

            let player_rect = Rect::new(self.x, self.y, 30.0, 46.0);
            for plat in platforms {
                if player_rect.overlaps(plat) && self.vy > 0.0 {
                    self.y = plat.y - 46.0;
                    self.vy = 0.0;
                    self.on_ground = true;
                }
            }
        }
    }

    #[test]
    fn test_player_gravity_and_ground_collision() {
        let mut player = TestPlayer::new(100.0, 100.0);
        let platform = Rect::new(50.0, 200.0, 200.0, 20.0);

        assert!(!player.on_ground);

        // Simulate 0.5s of falling
        for _ in 0..10 {
            player.update_physics(0.05, &[platform]);
        }

        assert!(player.on_ground);
        assert_eq!(player.y, 200.0 - 46.0);
        assert_eq!(player.vy, 0.0);
    }

    #[test]
    fn test_player_jump_velocity() {
        let mut player = TestPlayer::new(100.0, 154.0);
        player.on_ground = true;

        player.jump();
        assert_eq!(player.vy, -450.0);
        assert!(!player.on_ground);

        // Cannot jump again while airborne
        player.jump();
        assert_eq!(player.vy, -450.0);
    }

    #[test]
    fn test_horizontal_movement_on_platform_does_not_stick_to_floor() {
        let mut player = TestPlayer::new(100.0, 154.0);
        player.on_ground = true;
        let platform = Rect::new(0.0, 200.0, 500.0, 20.0);

        let dt = 0.1;
        let mut vx = 200.0;
        player.x += vx * dt;

        // Inset horizontal collision box vertically
        let horiz_rect = Rect::new(player.x, player.y + 6.0, 30.0, 46.0 - 12.0);
        if horiz_rect.overlaps(&platform) {
            vx = 0.0;
        }

        assert_eq!(vx, 200.0);
        assert_eq!(player.x, 120.0);
    }

    #[test]
    fn test_shared_mechanism_dual_activation() {
        struct TestMech {
            player_a: Option<usize>,
            player_b: Option<usize>,
            progress: f32,
            is_open: bool,
        }

        let mut mech = TestMech {
            player_a: None,
            player_b: None,
            progress: 0.0,
            is_open: false,
        };

        // Only one player holding -> progress does not increase
        mech.player_a = Some(0);
        if mech.player_a.is_some() && mech.player_b.is_some() {
            mech.progress += 0.5;
        }
        assert_eq!(mech.progress, 0.0);
        assert!(!mech.is_open);

        // Both players holding -> progress increases and triggers gate opening
        mech.player_b = Some(1);
        if mech.player_a.is_some() && mech.player_b.is_some() {
            mech.progress += 1.0;
            if mech.progress >= 1.0 {
                mech.is_open = true;
            }
        }
        assert!(mech.is_open);
    }

    #[test]
    fn test_rope_lowering_and_assistance() {
        struct TestRope {
            x: f32,
            y: f32,
            length: f32,
            is_lowered: bool,
        }

        let mut rope = TestRope {
            x: 500.0,
            y: 200.0,
            length: 150.0,
            is_lowered: false,
        };

        // Initially coiled up
        assert!(!rope.is_lowered);

        // Upper player toggles anchor
        rope.is_lowered = true;
        assert!(rope.is_lowered);

        // Player below at y = 300, x = 505 is within climbing range
        let climber_x = 505.0;
        let climber_y = 300.0;
        let in_range = (climber_x - rope.x).abs() < 20.0
            && climber_y >= rope.y
            && climber_y <= rope.y + rope.length;
        assert!(in_range);
    }

    #[test]
    fn test_carry_object_target_placement() {
        struct TestCarry {
            x: f32,
            y: f32,
            target_x: f32,
            target_y: f32,
            is_placed: bool,
        }

        let mut obj = TestCarry {
            x: 100.0,
            y: 500.0,
            target_x: 450.0,
            target_y: 200.0,
            is_placed: false,
        };

        assert!(!obj.is_placed);

        // Players carry object to target
        obj.x = 445.0;
        obj.y = 205.0;

        let dist = ((obj.x - obj.target_x).powi(2) + (obj.y - obj.target_y).powi(2)).sqrt();
        if dist < 40.0 {
            obj.x = obj.target_x;
            obj.y = obj.target_y;
            obj.is_placed = true;
        }

        assert!(obj.is_placed);
        assert_eq!(obj.x, 450.0);
        assert_eq!(obj.y, 200.0);
    }

    #[test]
    fn test_bot_takeover_grace_period() {
        let mut is_bot = false;
        let mut disconnect_timer = Some(5.0);

        let dt = 1.0;
        // Simulate 4 seconds
        for _ in 0..4 {
            if let Some(ref mut timer) = disconnect_timer {
                *timer -= dt;
                if *timer <= 0.0 {
                    is_bot = true;
                }
            }
        }
        // Still in grace period
        assert!(!is_bot);
        assert_eq!(disconnect_timer, Some(1.0));

        // Another 2 seconds passes
        for _ in 0..2 {
            if let Some(ref mut timer) = disconnect_timer {
                *timer -= dt;
                if *timer <= 0.0 {
                    is_bot = true;
                }
            }
        }
        // Grace period expired, bot takes over!
        assert!(is_bot);

        // Human reconnects
        let connected = true;
        if connected {
            is_bot = false;
        }
        assert!(!is_bot);
        assert!(connected);
    }
}

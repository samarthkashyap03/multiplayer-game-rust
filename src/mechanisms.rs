use macroquad::math::Rect;

#[derive(Debug, Clone)]
pub struct RopeAnchor {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub length: f32,
    pub is_lowered: bool,
    pub activated_by: Option<usize>,
}

impl RopeAnchor {
    pub fn new(id: usize, x: f32, y: f32, length: f32) -> Self {
        Self {
            id,
            x,
            y,
            length,
            is_lowered: false,
            activated_by: None,
        }
    }

    pub fn anchor_bounds(&self) -> Rect {
        Rect::new(self.x - 24.0, self.y - 20.0, 48.0, 40.0)
    }

    pub fn rope_bounds(&self) -> Rect {
        Rect::new(self.x - 16.0, self.y, 32.0, self.length)
    }
}

#[derive(Debug, Clone)]
pub struct SharedMechanism {
    pub id: usize,
    pub switch_a: (f32, f32),
    pub switch_b: (f32, f32),
    pub player_a: Option<usize>,
    pub player_b: Option<usize>,
    pub activation_progress: f32, // 0.0 to 1.0
    pub is_open: bool,
    pub door_rect: Rect,
    pub door_open_y: f32, // where door slides to
}

impl SharedMechanism {
    pub fn new(
        id: usize,
        switch_a: (f32, f32),
        switch_b: (f32, f32),
        door_rect: Rect,
        door_open_y: f32,
    ) -> Self {
        Self {
            id,
            switch_a,
            switch_b,
            player_a: None,
            player_b: None,
            activation_progress: 0.0,
            is_open: false,
            door_rect,
            door_open_y,
        }
    }

    pub fn switch_a_bounds(&self) -> Rect {
        Rect::new(self.switch_a.0 - 24.0, self.switch_a.1 - 20.0, 48.0, 40.0)
    }

    pub fn switch_b_bounds(&self) -> Rect {
        Rect::new(self.switch_b.0 - 24.0, self.switch_b.1 - 20.0, 48.0, 40.0)
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_open {
            // Smoothly slide door open
            if self.door_rect.y > self.door_open_y {
                self.door_rect.y = (self.door_rect.y - 120.0 * dt).max(self.door_open_y);
            }
            return;
        }

        // Both players holding levers simultaneously?
        if self.player_a.is_some() && self.player_b.is_some() {
            self.activation_progress += dt / 1.2; // takes 1.2s of joint holding
            if self.activation_progress >= 1.0 {
                self.activation_progress = 1.0;
                self.is_open = true;
            }
        } else {
            // Decay progress slowly if let go
            self.activation_progress = (self.activation_progress - dt * 0.8).max(0.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CarryObject {
    pub id: usize,
    #[allow(dead_code)]
    pub name: String,
    pub section: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub carried_by: Vec<usize>,
    pub is_placed: bool,
}

impl CarryObject {
    pub fn new(
        id: usize,
        name: &str,
        section: usize,
        x: f32,
        y: f32,
        target_x: f32,
        target_y: f32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            section,
            x,
            y,
            width: 36.0,
            height: 36.0,
            target_x,
            target_y,
            carried_by: Vec::new(),
            is_placed: false,
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    #[allow(dead_code)]
    pub fn target_bounds(&self) -> Rect {
        Rect::new(self.target_x - 10.0, self.target_y - 10.0, self.width + 20.0, self.height + 20.0)
    }

    pub fn check_target_placement(&mut self) -> bool {
        if self.is_placed {
            return true;
        }
        let dist = ((self.x - self.target_x).powi(2) + (self.y - self.target_y).powi(2)).sqrt();
        if dist < 45.0 {
            self.x = self.target_x;
            self.y = self.target_y;
            self.is_placed = true;
            self.carried_by.clear();
            return true;
        }
        false
    }
}

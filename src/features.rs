use rand::Rng;

pub struct Aimbot {
    enabled: bool,
}

impl Aimbot {
    pub fn new() -> Self {
        Aimbot { enabled: false }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn aim_at(&self, target: (f32, f32)) {
        if self.enabled {
            // Logic to aim at the target
        }
    }
}

pub struct Wallhack {
    enabled: bool,
}

impl Wallhack {
    pub fn new() -> Self {
        Wallhack { enabled: false }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn is_visible(&self, target: (f32, f32)) -> bool {
        if self.enabled {
            // Logic to check visibility
            true
        } else {
            false
        }
    }
}
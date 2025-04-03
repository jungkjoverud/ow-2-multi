pub struct Settings {
    aimbot_enabled: bool,
    wallhack_enabled: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            aimbot_enabled: false,
            wallhack_enabled: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    pub fn toggle_wallhack(&mut self) {
        self.wallhack_enabled = !self.wallhack_enabled;
    }
}
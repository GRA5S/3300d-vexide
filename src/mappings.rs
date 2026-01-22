use vexide::controller::{ButtonState, JoystickState};

// Different drive mods that the driver can switch to
pub enum DriveMode {
    Arcade {
        power: JoystickState,
        turn: JoystickState,
    },
    Tank {
        left: JoystickState,
        right: JoystickState,
    },
}

// TODO: Create ui to allow user to change mappings
// Map all the controller keybinds with their respective subsystem
pub struct ControllerMappings {
    pub drive_mode: DriveMode,

    pub hoard: ButtonState,
    pub outake: ButtonState,
    pub mid: ButtonState,
    pub long: ButtonState,

    pub wing: ButtonState,
    pub matchload: ButtonState,
}

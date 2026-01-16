use vexide::prelude::*;

pub trait BasePneumatic {
    fn up(&mut self) {
        let _ = self.port.set_high();
    }
    fn down(&mut self) {
        let _ = self.port.set_low();
    }
    fn toggle(&mut self) {
        let _ = self.port.toggle();
    }
}


// pub struct Hood {
//     port: AdiPneumatic,
//     port2: AdiPneumatic,
// }
// impl PneumaticActuator for Hood {}

// pub struct Wing {
//     port: AdiPneumatic,
// }
// impl PneumaticActuator for Wing {}

// pub struct Matchload {
//     port: AdiPneumatic,
// }
// impl PneumaticActuator for Matchload {}


macro_rules! pneumatic_struct {
    ($name:ident, $($field:ident: $type:ty),*) => {
        pub struct $name {
            $($field: $type,)*
        }
        impl BasePneumatic for $name {}
    };
}

pneumatic_struct!(Hood, port: AdiPneumatic, port2: AdiPneumatic);
pneumatic_struct!(Wing, port: AdiPneumatic);
pneumatic_struct!(Matchload, port: AdiPneumatic);

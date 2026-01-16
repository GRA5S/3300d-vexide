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

impl Hood {
    pub fn long(&mut self) {
        let _ = self.port.set_high();
        let _ = self.port2.set_high();
    }

    pub fn mid(&mut self) {
        let _ = self.port.set_high();
        let _ = self.port2.set_low();
    }

    pub fn hoard(&mut self) {
        let _ = self.port.set_low();
        let _ = self.port2.set_low();
    }
}









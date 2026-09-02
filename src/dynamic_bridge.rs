// ===================================== //
// 1. THE LOW-LEVEL IMPLEMENTATION TRAIT //
// ===================================== //

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

// ===================================== //
// 2. Concrete Implementations (structs) //
// ===================================== //

// ------------------------- //
// Concrete Implementation A //
// ------------------------- //

pub struct Tv {
    on: bool,
}
impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.on
    }
    fn enable(&mut self) {
        self.on = true;
    }
    fn disable(&mut self) {
        self.on = false;
    }
}

// ------------------------- //
// Concrete Implementation B //
// ------------------------- //

pub struct Radio {
    on: bool,
}
impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.on
    }
    fn enable(&mut self) {
        self.on = true;
    }
    fn disable(&mut self) {
        self.on = false;
    }
}

// =================================== //
// 3. THE HIGH-LEVEL ABSTRACTION TRAIT //
// =================================== //

pub trait Remote {
    fn toggle_power(&mut self);
    fn change_device(&mut self, new_device: Box<dyn Device>); // now part of the trait
}

// =============================== //
// 4. The (Dynamic) Bridge (struct)//
// =============================== //

pub struct BasicRemote {
    // The Bridge: Holds a trait object to decouple from concrete hardware types
    device: Box<dyn Device>,
}

impl BasicRemote {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self { device }
    }
}

// ========================================================= //
// 5. Implement High-level Abstraction for the Bridge Struct //
// ========================================================= //

impl Remote for BasicRemote {
    fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
            println!("Device turned OFF");
        } else {
            self.device.enable();
            println!("Device turned ON");
        }
    }

    fn change_device(&mut self, new_device: Box<dyn Device>) {
        self.device = new_device;
        println!("Swapped to a new device on the bridge.");
    }
}

// ======== //
// 6. Usage //
// ======== //

fn main() {
    // 1. Create low-level the hardware implementations
    let radio = Box::new(Radio { on: false });
    let tv = Box::new(Tv { on: false });

    // 2. Instantiate the High-level abstraction trait object with the radio
    let mut remote: Box<dyn Remote> = Box::new(BasicRemote::new(radio));

    // 3. Control the radio through the bridge abstraction
    remote.toggle_power(); // turns radio on 

    // 4. Hot-swap the implementation under the hood at runtime
    remote.change_device(tv);

    // 5. Control the TV using the exact same remote interface
    remote.toggle_power(); // turns tv on 
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tv_enable_disable() {
        let mut tv = Tv { on: false };
        assert!(!tv.is_enabled());
        tv.enable();
        assert!(tv.is_enabled());
        tv.disable();
        assert!(!tv.is_enabled());
    }

    #[test]
    fn radio_enable_disable() {
        let mut radio = Radio { on: false };
        assert!(!radio.is_enabled());
        radio.enable();
        assert!(radio.is_enabled());
        radio.disable();
        assert!(!radio.is_enabled());
    }

    #[test]
    fn dynamic_remote_toggle() {
        let radio = Box::new(Radio { on: false });
        let mut remote = BasicRemote::new(radio);
        remote.toggle_power();
        assert!(remote.device.is_enabled());
        remote.toggle_power();
        assert!(!remote.device.is_enabled());
    }

    #[test]
    fn dynamic_remote_change_device() {
        let radio = Box::new(Radio { on: false });
        let mut remote = BasicRemote::new(radio);
        remote.toggle_power();
        assert!(remote.device.is_enabled());

        let tv = Box::new(Tv { on: false });
        remote.change_device(tv);
        assert!(!remote.device.is_enabled());
        remote.toggle_power();
        assert!(remote.device.is_enabled());
    }
}

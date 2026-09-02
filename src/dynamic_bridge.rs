// ===================================== //
// 1. THE LOW-LEVEL IMPLEMENTATION TRAIT //
// ===================================== //

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

// Concrete Implementations (structs)

// Concrete Implementation A
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

// Concrete Implementation B
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
// 2. THE HIGH-LEVEL ABSTRACTION TRAIT //
// =================================== //

pub trait Remote {
    fn toggle_power(&mut self);
    fn change_device(&mut self, new_device: Box<dyn Device>);
}

// =============================== //
// 3. The (Dynamic) Bridge (struct)//
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
// 4. Implement High-level Abstraction for the Bridge Struct //
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

// ===================================== //
// 1. THE LOW-LEVEL IMPLEMENTATION TRAIT //
// ===================================== //

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

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
}

// =============================== //
// 3. The (Static) Bridge (struct) //
// =============================== //

// The Bridge: Evaluated at compile time via standard Rust generics
pub struct BasicRemote<D: Device> {
    device: D,
}

impl<D: Device> BasicRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }

    // Static types cannot mutate into different types in place.
    // To switch devices, we consume the remote and return a new typed instance.
    pub fn change_device<ND: Device>(self, new_device: ND) -> BasicRemote<ND> {
        println!("Statically swapped to a new device type.");
        BasicRemote::new(new_device)
    }
}

// ========================================================= //
// 4. Implement High-level Abstraction for the Bridge Struct //
// ========================================================= //

impl<D: Device> Remote for BasicRemote<D> {
    fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
            println!("Device turned OFF");
        } else {
            self.device.enable();
            println!("Device turned ON");
        }
    }
}

// =======================================================
// 3. USAGE
// =======================================================
fn main() {
    // 1. Create the hardware implementations (allocated entirely on the stack)
    let radio = Radio { on: false };
    let tv = Tv { on: false };

    // 2. Instantiate the abstraction. Type infers as BasicRemote<Radio>
    let mut remote = BasicRemote::new(radio);
    remote.toggle_power(); // turn radio on

    // 3. Swap the backend implementation.
    // Ownership transfer creates a brand new BasicRemote<Tv> type structure.
    let mut remote = remote.change_device(tv);
    remote.toggle_power(); // turn tv on
}

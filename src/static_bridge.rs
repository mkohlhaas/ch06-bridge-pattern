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
}

// =============================== //
// 4. The (Static) Bridge (struct) //
// =============================== //

// The Bridge connects low-level to high-level.

// The Bridge: Evaluated at compile time via standard Rust generics
// `Device` is a trait bound.
pub struct BasicRemote<D: Device> {
    device: D, // low-level!!!
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
// 5. Implement High-level Abstraction for the Bridge Struct //
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

// ======== //
// 6. USAGE //
// ======== //

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
    fn static_remote_toggle() {
        let radio = Radio { on: false };
        let mut remote = BasicRemote::new(radio);
        remote.toggle_power();
        assert!(remote.device.is_enabled());
        remote.toggle_power();
        assert!(!remote.device.is_enabled());
    }

    #[test]
    fn static_remote_change_device() {
        let radio = Radio { on: false };
        let mut remote = BasicRemote::new(radio);
        remote.toggle_power();
        assert!(remote.device.is_enabled());

        let tv = Tv { on: false };
        let mut remote = remote.change_device(tv);
        assert!(!remote.device.is_enabled());
        remote.toggle_power();
        assert!(remote.device.is_enabled());
    }
}

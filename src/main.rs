// ========================================== //
// 1. THE LOW-LEVEL IMPLEMENTATION HIERARCHY  //
// ========================================== //

trait RenderEngine {
    fn render_circle(&self, radius: f64);
    fn render_square(&self, side: f64);
}

// ---------------------- //
// Concrete Implementor A //
// ---------------------- //

struct VulkanEngine;
impl RenderEngine for VulkanEngine {
    fn render_circle(&self, radius: f64) {
        println!(
            "Vulkan [GPU Pipeline]: Rasterizing a circle with radius {:.1}.",
            radius
        );
    }

    fn render_square(&self, side: f64) {
        println!(
            "Vulkan [GPU Pipeline]: Rasterizing a square with sides {:.1}.",
            side
        );
    }
}

// ---------------------- //
// Concrete Implementor B //
// ---------------------- //

struct MetalEngine;
impl RenderEngine for MetalEngine {
    fn render_circle(&self, radius: f64) {
        println!(
            "Metal [Apple Silicon]: Drawing vector circle with radius {:.1}.",
            radius
        );
    }

    fn render_square(&self, side: f64) {
        println!(
            "Metal [Apple Silicon]: Drawing vector square with sides {:.1}.",
            side
        );
    }
}

// ========================================== //
// 2. THE HIGH-LEVEL ABSTRACTION HIERARCHY    //
// ========================================== //

trait Shape {
    fn draw(&self);
    fn resize(&mut self, factor: f64);
}

// --------------------- //
// Refined Abstraction A //
// --------------------- //

struct Circle {
    radius: f64,
    // THE BRIDGE: An owned trait object reference to any low-level engine
    engine: Box<dyn RenderEngine>,
}

impl Circle {
    fn new(radius: f64, engine: Box<dyn RenderEngine>) -> Self {
        Self { radius, engine }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        // High-level abstraction delegates work across the bridge
        self.engine.render_circle(self.radius);
    }

    fn resize(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

// --------------------- //
// Refined Abstraction B //
// --------------------- //

struct Square {
    side: f64,
    // THE BRIDGE: Reuses the exact same engine trait object abstraction
    engine: Box<dyn RenderEngine>,
}

impl Square {
    fn new(side: f64, engine: Box<dyn RenderEngine>) -> Self {
        Self { side, engine }
    }
}

impl Shape for Square {
    fn draw(&self) {
        // High-level abstraction delegates work across the bridge
        self.engine.render_square(self.side);
    }

    fn resize(&mut self, factor: f64) {
        self.side *= factor;
    }
}

// ===================== //
// 3. THE CLIENT UTILITY //
// ===================== //

fn main() {
    // We instantiate two separate engines
    let vulkan = Box::new(VulkanEngine);
    let metal = Box::new(MetalEngine);

    // We "cross the bridge" by pairing an abstraction with an implementation at runtime.
    // Notice how we can mix and match any shape with any engine seamlessly.
    let mut circle = Circle::new(5.0, vulkan);
    let square = Square::new(10.0, metal);

    println!("--- Initial Render ---");
    circle.draw();
    square.draw();

    println!("\n--- Modifying Abstraction ---");
    circle.resize(2.0);
    circle.draw();
}

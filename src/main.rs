// ========================================== //
// 1. THE LOW-LEVEL IMPLEMENTATION HIERARCHY  //
// ========================================== //

// The engine knows how to *draw*, but is deliberately agnostic to any specific
// shape. It only consumes a ShapeDescriptor — a shape that describes its own
// geometry to the backend. This keeps the two hierarchies independent: adding
// a new shape never touches an engine, and adding a new engine never touches
// a shape.

// A shape hands the backend a description of its geometry.
// (In a real renderer this might be a mesh, a canvas, shader bindings, etc.)
// NOTE: What would happen if I added a shape that needed more information than just what's in ShapeDescriptor?
struct ShapeDescriptor<'a> {
    kind: &'static str,
    size: f64,
    note: &'a str,
}

trait RenderEngine {
    fn render(&self, descriptor: &ShapeDescriptor);
}

// ---------------------- //
// Concrete Implementor A //
// ---------------------- //

struct VulkanEngine;
impl RenderEngine for VulkanEngine {
    fn render(&self, descriptor: &ShapeDescriptor) {
        println!(
            "Vulkan [GPU Pipeline]: Rasterizing a {} of size {:.1} ({})",
            descriptor.kind, descriptor.size, descriptor.note
        );
    }
}

// ---------------------- //
// Concrete Implementor B //
// ---------------------- //

struct MetalEngine;
impl RenderEngine for MetalEngine {
    fn render(&self, descriptor: &ShapeDescriptor) {
        println!(
            "Metal [Apple Silicon]: Drawing vector {} of size {:.1} ({})",
            descriptor.kind, descriptor.size, descriptor.note
        );
    }
}

// ========================================== //
// 2. THE HIGH-LEVEL ABSTRACTION HIERARCHY    //
// ========================================== //

trait Shape {
    fn draw(&self);
    fn describe(&self) -> ShapeDescriptor<'_>;
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
    fn describe(&self) -> ShapeDescriptor<'_> {
        ShapeDescriptor {
            kind: "circle",
            size: self.radius,
            note: "by radius",
        }
    }

    fn draw(&self) {
        // The shape describes itself, then delegates the actual rasterization
        // to its engine across the bridge.
        let descriptor = self.describe();
        self.engine.render(&descriptor);
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
    fn describe(&self) -> ShapeDescriptor<'_> {
        ShapeDescriptor {
            kind: "square",
            size: self.side,
            note: "side length",
        }
    }

    fn draw(&self) {
        let descriptor = self.describe();
        self.engine.render(&descriptor);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_resize() {
        let engine = Box::new(VulkanEngine);
        let mut circle = Circle::new(5.0, engine);
        circle.resize(2.0);
        assert!((circle.radius - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_square_draw_does_not_panic() {
        let engine = Box::new(MetalEngine);
        let square = Square::new(10.0, engine);
        square.draw();
    }

    #[test]
    fn test_cross_bridge_mix_and_match() {
        let vulkan = Box::new(VulkanEngine);
        let metal = Box::new(MetalEngine);
        let mut circle = Circle::new(3.0, vulkan);
        let mut square = Square::new(4.0, metal);
        circle.resize(1.5);
        square.resize(0.5);
        assert!((circle.radius - 4.5).abs() < f64::EPSILON);
        assert!((square.side - 2.0).abs() < f64::EPSILON);
    }
}

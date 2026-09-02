### The Bridge Design Pattern

The bridge pattern allows separating an abstraction from its
implementation so that both can evolve independently.

The Bridge design pattern in Rust is a structural design pattern that splits a
tightly coupled concept into two separate, independent hierarchies using traits: 
- An Abstraction (the high-level control layer or user-facing interface)
- Implementation (the low-level platform or backend logic)

Because Rust lacks object-oriented inheritance, the Bridge pattern is elegantly
modeled using traits to define the abstract behaviors, and composition via
struct fields to hold references to the interchangeable backends.

### Core Concepts

* *The Abstraction*: The entity interacting with the client (e.g., a RemoteControl or a Window GUI element).
* *The Implementation*: A standard trait interface that all low-level concrete implementations must adhere to (e.g., a Device or a RenderingEngine).
* *The Bridge*: The Abstraction struct holds a reference to a type that implements the Implementation trait (using generics, trait objects, or smart pointers), effectively bridging the two realms.

### Static vs. Dynamic Dispatch

In Rust, you can implement the bridge connection in two distinct flavors based on your architectural requirements:

| Approach | Mechanics | Trade-offs |
|---|---|---|
| Static Dispatch (Generics) | Uses trait bounds like struct RemoteControl<D: Device> directly. | No runtime overhead (monomorphization), but the types are locked at compile time. |
| Dynamic Dispatch (Trait Objects) | Uses heap allocations like struct RemoteControl { device: Box<dyn Device> }. | Allows swapping the device implementation at runtime, at the cost of virtual table pointer lookups. |

### When to Use the Bridge Pattern in Rust

* *Preventing Combinatorial Class Explosion*: Use it when you notice a feature set multiplying across multiple axes (e.g., File Format × Database Provider, or Shape Type × Rendering Pipeline).
* *Decoupling Compile Dependencies*: It lets you develop and maintain front-facing APIs independently from backend platforms or third-party engines.
* *Cross-Platform Abstractions*: Common for cross-platform Rust apps where the high-level code interacts with a single interface, while the backend translates it to Windows, macOS, or Linux APIs.

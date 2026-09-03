### The Bridge Design Pattern

Note: Compare both versions side-by-side in neovim.

The bridge pattern allows separating an abstraction from its
implementation so that both can evolve independently.

The Bridge design pattern in Rust is a structural design pattern that splits a
tightly coupled concept into two separate, independent hierarchies using traits: 
- An Abstraction (the high-level control layer or user-facing interface)
- Implementation (the low-level platform or backend logic)

Because Rust lacks object-oriented inheritance, the Bridge pattern is elegantly
modeled using traits to define the abstract behaviors, and composition via
struct fields to hold references to the interchangeable backends.

### Conceptual Diagram 

For the dynamic case:

```
    ┌──────────────┐
    │    Client    │
    └──────┬───────┘
           │
           │ uses
           ▼
┌──────────────────────────────────────┐          ┌──────────────────────────┐
│   «trait» Abstraction (high-level)   │          │  «trait» / Interface     │
├──────────────────────────────────────┤          │       (low-level)        │
│ - implementation: Box<dyn Impl>      │ ───────► ├──────────────────────────┤
├──────────────────────────────────────┤  has-a   │ + low_level_op1()        │
│ + feature_one()                      │          │ + low_level_op2()        │
│ + feature_two()                      │          └────────────▲─────────────┘
└──────────────────────────────────────┘                       │
                                                               │
                                               ┌───────────────┴───────────────┐
                                               │                               │
                                   ┌───────────┴───────────┐       ┌───────────┴───────────┐
                                   │ ConcreteImplementorA  │       │ ConcreteImplementorB  │
                                   ├───────────────────────┤       ├───────────────────────┤
                                   │                       │       │                       │
                                   ├───────────────────────┤       ├───────────────────────┤
                                   │ + low_level_op1()     │       │ + low_level_op1()     │
                                   │ + low_level_op2()     │       │ + low_level_op2()     │
                                   └───────────────────────┘       └───────────────────────┘
```

#### The Bridge

The "bridge" in this pattern is not a separate component, but rather the
relationship between the Abstraction struct and the Implementor trait
interface.

Specifically, the bridge is the composition link highlighted below:

```
┌──────────────────────────────────────┐               ┌──────────────────────────┐
│             Abstraction              │               │    «trait» / Interface   │
├──────────────────────────────────────┤ ============= │       Implementor        │
│ - implementation: Box<dyn Impl>      │  [THE BRIDGE] ├──────────────────────────┤
├──────────────────────────────────────┤               │ + low_level_op1()        │
│ + feature_one()                      │               │ + low_level_op2()        │
└──────────────────────────────────────┘               └──────────────────────────┘
```

Because of this separation, you can freely add new high-level functions to
Abstraction or add new platforms to Implementor without ever breaking the other
side.

If you inspect the actual code, the bridge manifests in distinct places
(field variables, constructors, structs, etc…).

In the sample code we use a struct:

```
       ┌──────────────┐
       │    Client    │
       └──────┬───────┘
              │
              │ uses
              ▼
  ┌───────────────────────┐
  │   «trait» / Interface │
  │      Abstraction      │
  ├───────────────────────┤
  │ + feature_one()       │
  │ + feature_two()       │
  └───────────▲───────────┘
              │
              │ implemented by
              │
  ┌───────────┴──────────────────────────┐
  │             [ STRUCT ]               │
  │               Bridge                 │
  ├──────────────────────────────────────┤          ┌──────────────────────────┐
  │ - implementor: Box<dyn Implementor>  │ ───────► │   «trait» / Interface    │
  ├──────────────────────────────────────┤  has-a   │       Implementor        │
  │ + feature_one()                      │          ├──────────────────────────┤
  │ + feature_two()                      │          │ + low_level_op1()        │
  └──────────────────────────────────────┘          │ + low_level_op2()        │
                                                    └────────────▲─────────────┘
                                                                 │
                                                                 │ implemented by
                                                 ┌───────────────┴───────────────┐
                                                 │                               │
                                     ┌───────────┴───────────┐       ┌───────────┴───────────┐
                                     │ ConcreteImplementorA  │       │ ConcreteImplementorB  │
                                     ├───────────────────────┤       ├───────────────────────┤
                                     │ + low_level_op1()     │       │ + low_level_op1()     │
                                     │ + low_level_op2()     │       │ + low_level_op2()     │
                                     └───────────────────────┘       └───────────────────────┘
```

### Core Concepts

* **The Abstraction**: The entity interacting with the client (e.g., a RemoteControl or a Window GUI element).
* **The Implementation**: A standard trait interface that all low-level concrete implementations must adhere to (e.g., a Device or a RenderingEngine).
* **The Bridge**: The Abstraction struct holds a reference to a type that implements the Implementation trait (using generics, trait objects, or smart pointers), effectively bridging the two realms.

### Static vs. Dynamic Dispatch

In Rust, you can implement the bridge connection in two distinct flavors based on your architectural requirements:

| Approach | Mechanics | Trade-offs |
|---|---|---|
| Static Dispatch (Generics) | Uses trait bounds like struct RemoteControl<D: Device> directly. | No runtime overhead (monomorphization), but the types are locked at compile time. |
| Dynamic Dispatch (Trait Objects) | Uses heap allocations like struct RemoteControl { device: Box<dyn Device> }. | Allows swapping the device implementation at runtime, at the cost of virtual table pointer lookups. |

### When to Use the Bridge Pattern in Rust

* **Preventing Combinatorial Class Explosion**: Use it when you notice a feature set multiplying across multiple axes (e.g., File Format × Database Provider, or Shape Type × Rendering Pipeline).
* **Decoupling Compile Dependencies**: It lets you develop and maintain front-facing APIs independently from backend platforms or third-party engines.
* **Cross-Platform Abstractions**: Common for cross-platform Rust apps where the high-level code interacts with a single interface, while the backend translates it to Windows, macOS, or Linux APIs.

### What is the Difference to the Adapter Pattern?

While both patterns act as structural wrappers that connect different parts of
a system, they serve entirely different goals and are used at different stages
of the software lifecycle.

The primary difference lies in intent: The Adapter pattern is a tool used to
fix compatibility issues in existing code, while the Bridge pattern is an
architectural choice used up front to prevent code from becoming tangled in the
first place.

Here is a direct comparison overview:

| Feature | 🌉 Bridge Pattern | 🔌 Adapter Pattern |
|---|---|---|
| Primary Intent | Decouple an abstraction from its implementation so the two can vary independently. | Convert the interface of an existing class into another interface that clients expect. |
| When to use | Designed up front during initial system architecture. | Applied after-the-fact when existing modules or 3rd-party libs don't fit together. |
| Relationship | Abstractions and implementations are split into completely independent hierarchies. | Two existing, incompatible interfaces are joined together. |
| Agnosticism | The abstraction layer doesn't care about the low-level details, and the backend doesn't care about the high-level logic. | The adapter forces an existing object (Adaptee) to conform to a specific Target interface. |

### Conceptual Comparison

#### The Adapter Pattern (🔌 "Fixing the Present")
Imagine you have an existing Rust application that expects a data logging
component to implement the Logger trait. You decide to use a third-party
library for cloud logging, but its struct uses a method called
`ship_telemetry_payload()`.

They don't match, and you can't rewrite the third-party library. You write an
Adapter struct that implements Logger and internally translates the calls to
the third-party format.

```
[ Client ] ──► [ «trait» Logger ]
                     ▲
                     │ (implemented by)
               [ LogAdapter ] ───► [ ThirdPartyCloudLib ] (Adaptee)
```

#### The Bridge Pattern (🌉 "Planning the Future")

Imagine you are building a cross-platform graphics application from scratch.
You know you will have multiple shapes (Circle, Square) and multiple rendering
engines (Vulkan, Metal).

Instead of creating 4 distinct combinations (VulkanCircle, MetalCircle,
VulkanSquare, MetalSquare), you use a Bridge. You create a high-level Shape
hierarchy and a low-level RenderEngine hierarchy. They are developed completely
independently, and they "cross bridges" at runtime.

```
[ High-Level Shapes ]          [ Low-Level Engines ]
   ┌──────────┐                   ┌──────────────┐
   │  Circle  │ ─── ( Bridge ) ──►│ VulkanEngine │
   ├──────────┤                   ├──────────────┤
   │  Square  │                   │ MetalEngine  │
   └──────────┘                   └──────────────┘
```

If you add a third shape (Triangle), you don't touch the engines. If you add a
third engine (WebGPU), you don't touch the shapes.

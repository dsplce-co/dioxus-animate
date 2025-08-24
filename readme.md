# dioxus-animate

**CSS Class Animations for Dioxus** — Time-based CSS class manipulation for smooth animations in [Dioxus](https://dioxuslabs.com/) apps. This crate provides ergonomic macros to sequence CSS class additions and removals with precise timing control.

---

## 🖤 Features

✅ Time-based CSS class manipulation<br>
✅ Ergonomic macro-based API<br>
✅ Group multiple operations together<br>
✅ Async-powered with no blocking<br>
✅ Type-safe animation sequences<br>

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-animate = "0.2.0"
```

This crate requires Rust 2024 edition.

⸻

## 🧪 Usage

### 1. Basic Animation Sequence

Use the `use_animate!` macro to create timed CSS class operations:

```rust
use dioxus::prelude::*;
use dioxus_animate::prelude::*;

#[component]
fn App() -> Element {
    let mut element_ref = use_signal(|| None);

    let animation = use_animate!(
        300 => add("opacity-100"),
        500 => remove("opacity-0"),
        1000 => add("scale-110"),
    );

    let start_animation = move |_| {
        animation.start(element_ref.into());
    };

    rsx! {
        div {
            class: "opacity-0 transition-all duration-300",
            onmounted: move |event| element_ref.set(Some(event.data())),
            onclick: start_animation,
            "Click me to animate!"
        }
    }
}
```

### 2. Grouped Operations

Execute multiple class operations simultaneously using parentheses:

```rust
let animation = use_animate!(
    0 => add("animate-pulse"),
    500 => (
        add("bg-blue-500");
        remove("bg-gray-200")
    ),
    1000 => remove("animate-pulse"),
);
```

### 3. Complex Animation Sequences

Chain multiple operations with precise timing:

```rust
let animation = use_animate!(
    0 => add("opacity-100"),
    200 => remove("opacity-0"),
    400 => add("scale-105"),
    600 => (
        add("rotate-3");
        add("shadow-lg")
    ),
    1000 => remove("scale-105 rotate-3"),
    1200 => add("scale-100"),
);
```

### 4. Triggering Animations

Animations can be triggered in two ways:

#### Method 1: Using Element References

Call `start()` with a reference to the mounted element:

```rust
// In your event handler
animation.start(element_ref.into());
```

The element reference is obtained through the `onmounted` event:

```rust
onmounted: move |event| element_ref.set(Some(event.data())),
```

#### Method 2: Using Element ID

Call `start_for_id()` with the element's ID string:

```rust
let animation = use_animate!(
    300 => add("opacity-100"),
    500 => remove("opacity-0"),
);

let trigger_animation = move |_| {
    animation.start_for_id("target");
};

rsx! {
    div {
        id: "target",
        class: "opacity-0 transition-all duration-300",
        onclick: trigger_animation,
        "Click me to animate!"
    }
}
```

This method is useful when you don't need to store element references or when targeting elements by ID is more convenient.

⸻

## 🧠 How It Works

1. **Define**: Use `use_animate!` to define your animation sequence with timestamps and operations
2. **Mount**: Capture element reference with `onmounted`
3. **Trigger**: Call `animation.start(element_ref.into())` to begin the sequence
4. **Execute**: Operations run asynchronously at their specified times

Time values are in milliseconds and represent cumulative time from animation start.

⸻

## 📐 API Reference

### `use_animate!`

Creates an animation sequence with the following syntax:

```rust
use_animate!(
    time_ms => operation,
    time_ms => operation,
    // ...
);
```

**Operations:**
- `add("class-names")` - Adds CSS classes to the element
- `remove("class-names")` - Removes CSS classes from the element
- `(op1; op2; ...)` - Groups multiple operations to execute simultaneously

**Time values:**
- Expressed in milliseconds
- Cumulative from animation start
- Must be in ascending order (think CSS keyframes)

### Animation Methods

#### `start(element_ref)`

Starts the animation sequence on the provided element reference:

```rust
animation.start(element_ref.into());
```

#### `start_for_id(id)`

Starts the animation sequence on an element with the specified ID:

```rust
animation.start_for_id("my-element");
```

This method is convenient when you prefer to target elements by ID rather than maintaining element references.

⸻

## 🔒 License

MIT or Apache-2.0, at your option.

⸻

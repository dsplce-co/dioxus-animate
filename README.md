> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# dioxus-animate

[![Dioxus](https://img.shields.io/badge/Dioxus-000000?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com/)
[![crates.io Downloads](https://img.shields.io/crates/d/dioxus-animate?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/dioxus-animate)
[![crates.io Size](https://img.shields.io/crates/size/dioxus-animate?style=for-the-badge)](https://crates.io/crates/dioxus-animate)
[![License](https://img.shields.io/crates/l/dioxus-animate.svg?style=for-the-badge)](https://crates.io/crates/dioxus-animate)
[![crates.io](https://img.shields.io/crates/v/dioxus-animate?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/dioxus-animate)

✨ Time-based CSS class animations for [Dioxus](https://dioxuslabs.com/) — think CSS keyframes, but driven by your app's logic.

`dioxus-animate` gives you one ergonomic macro to sequence CSS class additions and removals on a timeline. You say "at 300ms add `opacity-100`, at 500ms remove `opacity-0`", it runs the sequence asynchronously against a real DOM element. No animation runtime, no state machine to wire up — you already have the CSS, this just toggles the classes for you at the right moments.

Plays nicely with utility-class frameworks like Tailwind, where the transitions live in the classes and all you need is something to flip them on cue.

## 🖤 Features

- **`use_animate!`** — one declarative macro, your whole sequence reads top-to-bottom like keyframes
- **`add` / `remove`** — the only two verbs you need; classes go on, classes come off
- **Grouped ops** — fire several class changes at the exact same tick with `(...)`
- **Async under the hood** — sequences run on Dioxus' task runtime, nothing blocks
- **Two ways to target** — by mounted element reference, or by plain element `id`

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
- [🧪 Usage](#-usage)
  - [Basic animation sequence](#basic-animation-sequence)
  - [Grouped operations](#grouped-operations)
  - [Complex sequences](#complex-sequences)
  - [Trigger via element reference](#trigger-via-element-reference)
  - [Trigger via element id](#trigger-via-element-id)
- [🧠 How It Works](#-how-it-works)
- [📐 API Reference](#-api-reference)
- [🛠️ Compatibility](#%EF%B8%8F-compatibility)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
dioxus-animate = "0.3"
```

Or let cargo do the editing:

```bash
cargo add dioxus-animate
```

The latest version targets **Dioxus 0.7** and the **web (WASM) renderer** — see the [compatibility table](#%EF%B8%8F-compatibility) for the version mapping. Built on the Rust 2024 edition, so you'll want a recent stable toolchain.

⸻

## 🧪 Usage

### Basic animation sequence

Reach for the `use_animate!` macro to lay out timed CSS class operations, then call `start` on a mounted element:

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

### Grouped operations

Wrap operations in parentheses (separated by `;`) to fire them on the same tick:

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

### Complex sequences

Chain as many steps as you like — single ops and groups mix freely:

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

One thing to keep in mind: timestamps are cumulative from the start and must climb in ascending order (same as you'd write CSS keyframes). The runtime sleeps for the gap between each step, so a step that goes backwards in time isn't a thing.

### Trigger via element reference

Capture the element on `onmounted`, then hand its reference to `start`:

```rust
// grab it when the node mounts
onmounted: move |event| element_ref.set(Some(event.data())),

// fire it from any handler
animation.start(element_ref.into());
```

### Trigger via element id

Don't want to juggle references? Target by `id` with `start_for_id` instead — handy when the element lives somewhere awkward to thread a signal to:

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

Heads up: `start_for_id` expects the element to exist in the DOM at call time — it looks the node up by id and will panic if there's nothing there, so trigger it after the element has mounted.

⸻

## 🧠 How It Works

1. **Define** — `use_animate!` parses your `time => operation` lines into an ordered list of `(ms, Operation)` pairs
2. **Mount** — capture the element reference via `onmounted` (or skip it and target by `id`)
3. **Trigger** — `start(...)` / `start_for_id(...)` spawns an async task on Dioxus' runtime
4. **Execute** — the task sleeps to each timestamp in turn and toggles the classes on the live DOM element

Time values are in milliseconds, cumulative from the start of the sequence.

⸻

## 📐 API Reference

### `use_animate!`

Builds an animation sequence:

```rust
use_animate!(
    time_ms => operation,
    time_ms => operation,
    // ...
);
```

**Operations:**

- `add("class-names")` — adds CSS classes (space-separated string, multiple classes welcome)
- `remove("class-names")` — removes CSS classes (same deal)
- `(op1; op2; ...)` — groups operations to run on the same tick

**Time values:**

- expressed in milliseconds
- cumulative from animation start
- must be in ascending order (think CSS keyframes)

### `UseAnimate::start`

```rust
animation.start(element_ref.into());
```

Runs the sequence against a mounted element. Takes a `ReadSignal<Option<Rc<MountedData>>>` — in practice the `Signal` you filled on `onmounted`, with `.into()`. If the signal is still `None`, the call is a no-op (it just won't animate).

### `UseAnimate::start_for_id`

```rust
animation.start_for_id("my-element");
```

Runs the sequence against the element with the given `id`. Convenient when you'd rather not hold a reference — just make sure the element is in the DOM when you call it (it panics if the id isn't found).

⸻

## 🛠️ Compatibility

| Dioxus version | `dioxus-animate` version |
|:---------------|:-------------------------|
| `0.7`          | `0.3`                    |
| `0.6`          | `0.2`                    |

A couple of things worth knowing:

- **Web / WASM only** — it reaches for `web-sys`, `gloo` and Dioxus' web event APIs, so it runs in the browser renderer (it isn't wired up for desktop/mobile).
- **Rust 2024 edition** — you'll want a recent stable toolchain.

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/dioxus-animate](https://github.com/dsplce-co/dioxus-animate)<br>
📦 **Crate**: [https://crates.io/crates/dioxus-animate](https://crates.io/crates/dioxus-animate)

Contributions, issues, ideas? Hit us up 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.

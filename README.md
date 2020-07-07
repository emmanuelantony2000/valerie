# Valerie

Rust front-end framework for building web apps.

*Valerie is still in a very early phase.
A lot of features are not available at the moment.
A lot of work is left and you are welcome to try it out.*

 - No Virtual DOM.
 - UI can be made in a simple manner,
 by following an MVVM architecture rather an MVC architecture.
 - Use state variables to update the UI where required.
 - Written without any unsafe code.
 - `nightly` Rust required.

## Architecture

 - Every UI element has to implement the `Component` trait.
 - A page is a function which returns a `Node`.
 - Two type of State variables 
   - `StateAtomic` for types implementing `Copy`.
   - `StateMutex` for types implementing `Clone`.

## Setting up

 - Run `cargo new --lib some_name`
 - Add `valerie` to the dependencies
 - Make a `static` directory and make an `index.html` inside it

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Title</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```

 - Also in the `Cargo.toml` enable lto.
 
```toml
[profile.release]
lto = true
opt-level = 3
```

 - Use some server, like [miniserve](https://github.com/svenstaro/miniserve), to host it and try it out.
 
Take a look at `wasm-pack` docs for more options.

## Examples

### Hello world

```rust
use valerie::prelude::components::*;
use valerie::prelude::*;

fn ui() -> Node {
    h1!("Hello World").into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}
```

### Add and Subtract one using a Button

```rust
use valerie::prelude::components::*;
use valerie::prelude::*;

fn ui() -> Node {
    let value = StateAtomic::new(0isize);

    div!(
        h1!("Value ", value.clone()),
        button!("Add 1")
            .on_event("click", value.clone(), move |x, _| {
                *x += 1;
            }),
        button!("Subtract 1")
            .on_event("click", value.clone(), move |x, _| {
                *x -= 1;
            })
    )
    .into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}
```

### Time Counter

```rust
use valerie::prelude::components::*;
use valerie::prelude::*;
use wasm_timer::Delay;

fn ui() -> web_sys::Node {
    let timer = StateAtomic::new(0);

    execute(time(1, timer.clone()));

    p!("Seconds passed: ", timer).into()
}

async fn time(n: u64, mut timer: StateAtomic<usize>) {
    while Delay::new(core::time::Duration::from_secs(n))
        .await
        .is_ok() {
            timer += 1;
        }
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}
```

There are more examples in the examples directory.

## Issues and Contributing

Since this is a new project, there are a lot of issues currently and a lot of features yet to be implemented.
Please do contribute. Contribution guidelines will be up soon.
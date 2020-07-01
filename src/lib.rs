#![no_std]
#![deny(missing_docs)]

//! Rust front-end framework for building web apps.
//!
//! *Valerie is still in a very early phase.
//! A lot of feature are not available at the moment.
//! A lot of work is left and you are welcome to try it out.*
//!
//! - No Virtual DOM.
//! - UI can be made simply, by following an MVVM architecture rather an MVC architecture.
//! - Use state variables to update the UI where required.
//! - Written without any unsafe code and `nightly` rust required.
//!
//! ## Architecture
//!
//! - Every UI element has to implement the `Component` trait.
//! - A page is a function which returns a `web_sys::Node`.
//! - Two type of State variables
//! - `StateAtomic` for types implementing `Copy`.
//! - `StateMutex` for types implementing `Clone`.
//!
//! ## Setting up
//!
//! - Run `cargo new --lib some_name`
//! - Add `valerie` to the dependencies
//! - Make a `static` directory and make an `index.html` inside it
//!
//! ```html
//! <!doctype html>
//! <html lang="en">
//!     <head>
//!         <meta charset="utf-8">
//!         <title>Title</title>
//!         <script type="module">
//!             import init from "./wasm.js"
//!             init()
//!         </script>
//!     </head>
//!     <body></body>
//! </html>
//! ```
//!
//! - Also in the `Cargo.toml` enable lto.
//!
//! ```toml
//! [profile.release]
//! lto = true
//! opt-level = 3
//! ```
//!
//! - Use some server, like [miniserve](https://github.com/svenstaro/miniserve), to host it and try it out.
//!
//! Take a look at `wasm-pack` docs for more options.
//!
//! ## Examples
//!
//! ### Hello world
//!
//! ```rust
//! use valerie::prelude::components::*;
//! use valerie::prelude::*;
//!
//! fn launch_page() -> web_sys::Node {
//!     h1!("Hello World").into()
//! }
//!
//! #[valerie(start)]
//! pub fn run() {
//!     App::new().push("hello_world", launch_page).render();
//! }
//! ```
//!
//! ### Add and Subtract one using a Button
//!
//! ```rust
//! use valerie::prelude::components::*;
//! use valerie::prelude::*;
//!
//! fn launch_page() -> web_sys::Node {
//!     let value = StateAtomic::new(0isize);
//!
//!     div!(
//!         h1!("Value ", value.clone()),
//!         button!("Add 1")
//!             .on_event("click", value.clone(), move |x, _| {
//!                 *x += 1;
//!             }),
//!         button!("Subtract 1")
//!             .on_event("click", value.clone(), move |x, _| {
//!                 *x -= 1;
//!             })
//!     )
//!         .into()
//! }
//!
//! #[valerie(start)]
//! pub fn run() {
//!     App::new()
//!         .push("list_add_remove_items", launch_page)
//!         .render();
//! }
//! ```
//!
//! ### Time Counter
//!
//! ```rust
//! use valerie::prelude::components::*;
//! use valerie::prelude::*;
//! use wasm_timer::Delay;
//!
//! fn launch_page() -> web_sys::Node {
//!     let timer = StateAtomic::new(0);
//!
//!     execute(time(1, timer.clone()));
//!
//!     p!("Seconds passed: ", timer).into()
//! }
//!
//! async fn time(n: u64, mut timer: StateAtomic<usize>) {
//!     while Delay::new(core::time::Duration::from_secs(n))
//!         .await
//!         .is_ok() {
//!         timer += 1;
//!     }
//! }
//!
//! #[valerie(start)]
//! pub fn run() {
//!     App::new().push("time_counter", launch_page).render();
//! }
//! ```

extern crate alloc;

mod component;
mod function;
mod macros;

/// Contains the `App` struct for creating the `App` instance
pub mod app;
/// Contains the `Channel` struct which is the message passed from the `State`
pub mod channel;
/// Contains the structs for defining States
pub mod state;
/// To make HTML Tags
pub mod tag;

pub use component::Component;

/// The `prelude` module
pub mod prelude {
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::wasm_bindgen as valerie;
    pub use wasm_bindgen_futures::spawn_local as execute;
    pub use web_sys;

    pub use app::*;
    pub use component::Component;
    pub use state::{StateAtomic, StateMutex, StateTrait, StateVec};

    use crate::*;

    /// The components module imports all the macros for the Tags
    pub mod components {
        pub use crate::{br, button, div, h1, h2, h3, h4, h5, h6, input, li, ol, p, span, ul};
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

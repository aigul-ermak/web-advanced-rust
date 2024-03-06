# Rust + WebAssembly + Webpack

The current example it to demostrate how use Rust to manipulate the DOM using WebAssembly and Webpack.

## Prerequisites

To start with this project you need to have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)

# Preparation

## Web Assembly

Once the tools are installed we gonna to use npm to install Web Assembly for the system with ***Admin privileges*** in powershell or cmd:

``` npm install -g wasm-pack ```

# Project Creation

Navigate to the folder where you want to create the project and run the following commands:

- Create a new project folder and navigate to it:

``` mkdir <project-name> ```

``` cd <project-name> ```

- Initialize the project with npm:
  
  ``` npm init rust-webpack ```

-Test that the project is working:

``` npm run start ```

# Updating Dependencies and Configuration

Open the file **Cargo.toml** and update the following dependencies to the latest version:

- edition = "2021"
- wasm-bindgen = "0.2.91"
- wee-alloc = "0.4.5"
- [dependencies.web-sys]
  version = "0.3.68"
- console_error_panic_hook = "0.1.7"
- wasm-bindgen-test = "0.3.30"
- futures = "0.3.30"
- js-sys = "0.3.68"
- wasm-bindgen-futures = "0.4.41"

Add the a new cargo package in the main [dependencies] section:

- console_error_panic_hook = "0.1.7"

Delete the following lines from the file **Cargo.toml**:

```rust
[target."cfg(debug_assertions)".dependencies]
console_error_panic_hook = "0.1.7"
```

Add the following features to [dependencies.web-sys]:

```rust
features = features = ["Window", "Document", "HtmlCanvasElement", "CanvasRenderingContext2d"]
```

And in **src/lib.rs** in consequence delete the lines:

```rust
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

```rust
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
```

and

```rust
use web_sys::console;
```

Finally in **test/app.rs** only leave the following code:

```rust
use wasm_bindgen_test::{wasm_bindgen_test_configure, wasm_bindgen_test};

wasm_bindgen_test_configure!(run_in_browser);


// This runs a unit test in native Rust, so it can only use Rust APIs.
#[test]
fn rust_test() {
    assert_eq!(1, 1);
}

// This runs a unit test in the browser, so it can use browser APIs.
#[wasm_bindgen_test]
fn web_test() {
    assert_eq!(1, 1);
}
```

# Creating and Manipuling the Canvas

Open the file **static/index.html** and add create the canvas and a javascript script to manipulate the DOM:

-Body

```javascript
<canvas id="canvas" tabindex="0" height="600" width="600"></canvas>
```

-Script

```javascript
<script>
  canvas = document.getElementById('canvas');
  context = canvas.getContext('2d');

  context.moveTo(300, 0);
  context.beginPath();
  context.lineTo(0, 600);
  context.lineTo(600, 600);
  context.lineTo(300, 0);
  context.closePath();
  context.stroke();
  context.fill();
</script> 
```

# Manipuling the DOM From Rust

Comment the las script create to generate the triangle in the canvas and add the following code to the **src/lib.rs** file:

-Add de library to cast the canvas to a web_sys::HtmlCanvasElement:

```rust
use wasm_bindgen::JsCast;
```

-Emulate the behavior of the script in the **static/index.html** file on rust remplacin the hello world in main function:

```rust
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

    let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

    context.move_to(300.0, 0.0);
    context.begin_path();
    context.line_to(0.0, 600.0);
    context.line_to(600.0, 600.0);
    context.line_to(300.0, 0.0);
    context.close_path();
    context.stroke();
```

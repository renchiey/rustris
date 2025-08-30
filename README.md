# RUSTRIS
Introducing... Tetris but built with Rust!

Rustris is a web based single page application (SPA) built using Rust and the Leptos framework.

## Running Locally
In order to run this web app locally, you will need to have Rust installed. To install **Rust**, follow the installation guide on the official website [here](https://www.rust-lang.org/tools/install).

If Rust has been installed, the next thing to install is **Trunk**, a tool for running Leptos client side rendered applications. Install it by running the following command:
```
cargo install trunk
```

Now we need to add the `wasm32-unknown-unknown` compilation target so that Rust can compile the code into web-assembly. You can do so using the following command:
```
rustup target add wasm32-unknown-unknown
```

Finally, we can run the application using the following command:
```
trunk serve --open
```



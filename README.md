## Yew.rs tailwind template

A minimal template for using yew.rs with tailwindcss!

### Features

-   Routing
-   Example component
-   Custom font
-   404 page

### Using this template

1. Clone the repo:

```
git clone https://github.com/snowstry/yew-tailwind-template
```

2. Get a prebuilt binary of tailwind from their official website. You can also use npm/yarn to globally install tailwindcss.

3. Install `trunk`, `wasm-bindgen-cli` and webassembly target.

```
cargo install trunk
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

4. Run `trunk serve`

yakui-macroquad
-----------------------
[![CI](https://github.com/profan/yakui-macroquad/actions/workflows/rust.yml/badge.svg)](https://github.com/profan/yakui-macroquad/actions/workflows/rust.yml)
[![Docs](https://docs.rs/yakui-macroquad/badge.svg?version=0.3.1)](https://docs.rs/yakui-macroquad/0.3.1/yakui_macroquad/)
[![Crates.io version](https://img.shields.io/crates/v/yakui-macroquad.svg)](https://crates.io/crates/yakui-macroquad)

This is a little macroquad integration for [yakui](https://github.com/SecondHalfGames/yakui), built ontop of [yakui-miniquad](https://github.com/profan/yakui-miniquad).

# Version
This version is for macroquad 0.4.5 and yakui 0.2.0.

# Example
```rust
use macroquad::prelude::*;
use yakui_macroquad::*;

#[macroquad::main("yakui-macroquad-example")]
async fn main() {
    
    loop {

        clear_background(WHITE);

        yakui_macroquad::start();

        yakui::center(|| {
            let mut text_box = yakui::widgets::Text::new(32.0, "hello, world!");
            text_box.style.color = yakui::Color::BLACK;
            text_box.show();
        });

        yakui_macroquad::finish();

        yakui_macroquad::draw();

        next_frame().await;

    }
    
}
```

You can also run the example with `cargo run --example hello-world`.

# License
See LICENSE

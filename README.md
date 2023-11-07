yakui-macroquad
-----------------------
This is a little macroquad integration for [yakui](https://github.com/SecondHalfGames/yakui), built ontop of [yakui-miniquad](https://github.com/profan/yakui-miniquad).

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

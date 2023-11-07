use macroquad::prelude::*;
use yakui_miniquad::yakui;
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

use macroquad::prelude::*;
use yakui_macroquad::*;

#[macroquad::main("yakui-macroquad-example")]
async fn main() {
    loop {
        clear_background(WHITE);

        // Can also use yakui_macroquad::ui(|_| { /* draw stuff here */}); to avoid start and finish

        yakui_macroquad::start();

        yakui::center(|| {
            yakui::colored_box_container(yakui::Color::CORNFLOWER_BLUE, || {
                yakui::pad(yakui::widgets::Pad::all(16.0), || {
                    yakui::text(32.0, "hello, world!");
                });
            });
        });

        yakui_macroquad::finish();

        yakui_macroquad::draw();

        next_frame().await;
    }
}

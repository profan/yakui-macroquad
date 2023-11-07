//!
//! `yakui-macroquad` integrates yakui with macroquad.
//!
//! # Usage
//! To use this library, you call [`start`] when you wish to begin
//! submitting ui draw commands and [`finish`] when you are done.
//!
//! Though, there's also the [`ui`] helper that takes a closure and will call [`start`] before your code and [`finish`] after.
//! But using [`start`] and [`finish`] is closer to how yakui itself does it, so that's probably what you should do.
//!
//! To then render your ui, simply call [`draw`]!
//!
//! ```no_run
//! use macroquad::prelude::*;
//! use yakui_macroquad::*;
//!
//! #[macroquad::main("yakui-macroquad-example")]
//! async fn main() {
//!    
//!     loop {
//!
//!         clear_background(WHITE);
//!
//!         yakui_macroquad::start();
//!
//!         yakui::center(|| {
//!             let mut text_box = yakui::widgets::Text::new(32.0, "hello, world!");
//!             text_box.style.color = yakui::Color::BLACK;
//!             text_box.show();
//!         });
//!
//!         yakui_macroquad::finish();
//!
//!         yakui_macroquad::draw();
//!
//!         next_frame().await;
//!
//!     }
//!    
//! }
//!```

use macroquad::miniquad as mq;
use macroquad::window::get_internal_gl;
use yakui_miniquad::*;

pub use macroquad;
pub use yakui_miniquad::yakui;

struct Yakui(YakuiMiniQuad, usize);

// Global variable and global functions because it's more like macroquad way
static mut YAKUI: Option<Yakui> = None;

fn get_yakui() -> &'static mut Yakui {
    unsafe {
        if let Some(yakui) = &mut YAKUI {
            yakui
        } else {
            YAKUI = Some(Yakui::new());
            YAKUI.as_mut().unwrap()
        }
    }
}

impl Yakui {
    fn new() -> Self {
        Self(
            YakuiMiniQuad::new(unsafe { get_internal_gl() }.quad_context),
            macroquad::input::utils::register_input_subscriber(),
        )
    }

    fn start(&mut self) {
        let ctx = unsafe { get_internal_gl() }.quad_context;
        macroquad::input::utils::repeat_all_miniquad_input(self, self.1);
        self.0.start(ctx);
    }

    fn finish(&mut self) {
        self.0.finish();
    }

    fn ui<F>(&mut self, f: F)
    where
        F: FnOnce(&mut yakui::Yakui) -> (),
    {
        let gl = unsafe { get_internal_gl() };
        macroquad::input::utils::repeat_all_miniquad_input(self, self.1);

        self.0.run(gl.quad_context, f);
    }

    fn draw(&mut self) {
        let mut gl = unsafe { get_internal_gl() };
        // Ensure that macroquad's shapes are not goint to be lost, and draw them now
        gl.flush();
        self.0.draw(&mut gl.quad_context);
    }
}

/// Binds the yakui context to the current thread.
pub fn start() {
    get_yakui().start();
}

/// Finishes the current yakui context and prepares it for rendering.
pub fn finish() {
    get_yakui().finish();
}

/// Allows you to submit commands to the yakui context inside the scope of the closure passed, calls [`start`] and [`finish`] for you.
pub fn ui<F: FnOnce(&mut yakui::Yakui)>(f: F) {
    get_yakui().ui(|ctx| f(ctx))
}

/// Allows you configure the yakui context within the scope of the closure passed, if you need to.
pub fn cfg<F: FnOnce(&mut yakui::Yakui)>(f: F) {
    f(get_yakui().0.ctx());
}

/// Draws the yakui ui. Must be called after `finish`/`ui` and once per frame.
pub fn draw() {
    get_yakui().draw()
}

impl mq::EventHandler for Yakui {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, _ctx: &mut mq::Context) {}

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.0.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.0.mouse_wheel_event(ctx, dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.0.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.0.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        ctx: &mut mq::Context,
        character: char,
        keymods: mq::KeyMods,
        repeat: bool,
    ) {
        self.0.char_event(ctx, character, keymods, repeat);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        repeat: bool,
    ) {
        self.0.key_down_event(ctx, keycode, keymods, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.0.key_up_event(ctx, keycode, keymods);
    }
}

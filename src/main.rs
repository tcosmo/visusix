#![deny(missing_docs)]

//! A visualisation tool for the base 6 Collatz 1DCA

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Windoww;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{Window, WindowSettings};

use clap::{App, Arg};

pub use crate::state::*;
pub use crate::state_controller::CCAStateController;
pub use crate::state_view::{CCAStateView, CCAStateViewSettings};

mod state;
mod state_controller;
mod state_view;

fn main() {
    let matches = App::new("Visusix")
        .version("0.1.0")
        .author("Cosmo <tristan.sterin@gmail.com>")
        .about("Collatz base 6 automaton")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Base 6 input"),
        )
        .get_matches();

    let base6_str = matches.value_of("input").unwrap_or("12345");

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Visusix", [1200, 800])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: Windoww = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut state: CCAState = CCAState::from_str(base6_str).unwrap();
    let mut state_controller = CCAStateController::new(state);
    let state_view_settings = CCAStateViewSettings::new(window.size());
    let mut state_view = CCAStateView::new(state_view_settings);

    while let Some(e) = events.next(&mut window) {
        state_controller.event(&e);
        state_view.event(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |mut c, g| {
                use graphics::{clear, Transformed};

                clear(state_view_settings.background_color, g);
                state_view.draw(&state_controller, &mut c, g);
            });
        }
    }
}

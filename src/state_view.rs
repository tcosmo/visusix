//! Collatz Base 6 CA State view.

use graphics::types::{Color, Radius};
use graphics::{Context, Graphics};
use piston::window::Size;

use crate::state::TritBitDomino;
use crate::state_controller::CCAStateController;

use piston::input::GenericEvent;

/// Stores state view settings.
#[derive(Debug, Clone, Copy)]
pub struct CCAStateViewSettings {
    /// Window size.
    pub window_size: Size,

    /// View origin.
    pub origin: [f64; 2],

    /// Default step when translating view origin
    pub default_trans_step: f64,

    /// Background color.
    pub background_color: Color,

    /// Color of any textural info
    pub text_color: Color,

    /// Height of TritBitDomino in px
    pub domino_height: f64,

    /// Width of TritBitDomino in px
    pub domino_width: f64,

    /// Zoom factor used to display the view
    pub zoom_factor: f64,

    /// Default zoom step when zooming view
    pub default_zoom_step: f64,

    /// Colors to represent trits in base 3' alphabet
    pub trit_colors: [Color; 4],

    /// Colors to represent bits
    pub bit_colors: [Color; 2],

    /// Color to outline cells on tail
    pub tail_border_color: Color,

    /// Thickness of tail outline
    pub tail_border_radius: Radius,
}

impl CCAStateViewSettings {
    /// Creates new gameboard view settings.
    pub fn new(window_size: Size) -> CCAStateViewSettings {
        CCAStateViewSettings {
            window_size: window_size,
            origin: [
                (window_size.width as f64) - 100.0,
                (window_size.height as f64) - 170.0,
            ],
            default_trans_step: 100.,
            background_color: [0.2, 0.2, 0.2, 1.0],
            text_color: [1.0; 4],
            domino_height: 20.0,
            domino_width: 10.0,
            zoom_factor: 7.0,
            default_zoom_step: 1.5,
            trit_colors: [
                [0., 0., 0., 1.],
                [0.6, 0.1, 0.1, 1.],
                [0.1, 0.6, 0.1, 1.],
                [0.6, 0.6, 0.6, 1.],
            ],
            bit_colors: [[0.6, 0.6, 0.6, 1.], [0., 0., 0., 1.]],

            tail_border_color: [0.9, 0.3, 0.3, 0.3],
            tail_border_radius: 0.5,
        }
    }
}

/// Stores visual information about the state.
pub struct CCAStateView {
    /// Stores state view settings.
    pub settings: CCAStateViewSettings,
}

impl CCAStateView {
    /// Creates a new state view.
    pub fn new(settings: CCAStateViewSettings) -> CCAStateView {
        CCAStateView { settings: settings }
    }

    // fn trit_color(&self, tbd: &TritBitDomino) -> Color {
    //     return self.settings.trit_colors[2*(tbd.trit_bit[0] as usize)+(tbd.trit_bit[1] as usize)];
    // }

    fn bit_color(&self, bit_value: bool) -> Color {
        return self.settings.bit_colors[bit_value as usize];
    }

    /// Draw state.
    pub fn draw<G: Graphics>(&self, controller: &CCAStateController, c: &mut Context, g: &mut G) {
        use graphics::{rectangle::Border, Rectangle, Transformed};

        let ref settings = self.settings;

        /* Set the view to correct origin and scale. */
        c.transform = c
            .transform
            .scale(settings.zoom_factor, settings.zoom_factor);
        c.transform = c.transform.trans(
            settings.origin[0] / settings.zoom_factor,
            settings.origin[1] / settings.zoom_factor,
        );

        let mut curr_abstract_pos: [f64; 2] = [0., 0.];

        for tbd in controller.state.cells.iter().rev() {
            if !(tbd.is_tail) {
                curr_abstract_pos[1] -= 0.5;
            }

            let px_pos = [
                (curr_abstract_pos[0] as f64) * settings.domino_width,
                (curr_abstract_pos[1] as f64) * settings.domino_height,
            ];

            let domino_upper_trit_rect = [
                px_pos[0],
                px_pos[1],
                settings.domino_width,
                settings.domino_height / 4.0,
            ];

            let domino_lower_trit_rect = [
                px_pos[0],
                px_pos[1] + settings.domino_height / 4.0,
                settings.domino_width,
                settings.domino_height / 4.0,
            ];

            let domino_bit_rect = [
                px_pos[0],
                px_pos[1] + settings.domino_height / 2.0,
                settings.domino_width,
                settings.domino_height / 2.0,
            ];

            let border_color = if !tbd.is_tail {
                [1.; 4]
            } else {
                self.settings.tail_border_color
            };

            Rectangle::new(self.bit_color(tbd.trit_bit[0]))
                .border(Border {
                    color: border_color,
                    radius: 0.5,
                })
                .draw(domino_upper_trit_rect, &c.draw_state, c.transform, g);

            Rectangle::new(self.bit_color(tbd.trit_bit[1]))
                .border(Border {
                    color: border_color,
                    radius: 0.5,
                })
                .draw(domino_lower_trit_rect, &c.draw_state, c.transform, g);

            Rectangle::new(self.bit_color(tbd.trit_bit[2]))
                .border(Border {
                    color: border_color,
                    radius: 0.5,
                })
                .draw(domino_bit_rect, &c.draw_state, c.transform, g);

            if tbd.is_tail {
                let domino_rect = [
                    px_pos[0],
                    px_pos[1],
                    settings.domino_width,
                    settings.domino_height,
                ];

                Rectangle::new_border(
                    self.settings.tail_border_color,
                    self.settings.tail_border_radius,
                )
                .draw(domino_rect, &c.draw_state, c.transform, g);
            }

            curr_abstract_pos[0] -= 1.5;
        }
    }

    /// Handles view-related events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A => self.settings.zoom_factor /= self.settings.default_zoom_step,
                Key::Z => self.settings.zoom_factor *= self.settings.default_zoom_step,
                Key::Right => self.settings.origin[0] -= self.settings.default_trans_step,
                Key::Left => self.settings.origin[0] += self.settings.default_trans_step,
                Key::Up => self.settings.origin[1] += self.settings.default_trans_step,
                Key::Down => self.settings.origin[1] -= self.settings.default_trans_step,
                _ => {}
            }
        }
    }
}

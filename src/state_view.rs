//! Collatz Base 6 CA State view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use piston::window::Size;

use crate::state_controller::CCAStateController;
use crate::state::TritBitDomino;

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
    pub default_zoom_step: f64
}

impl CCAStateViewSettings {
    /// Creates new gameboard view settings.
    pub fn new(window_size: Size) -> CCAStateViewSettings {
        CCAStateViewSettings {
            window_size: window_size,
            origin: [1.*(window_size.width as f64)/2.0,
                     1.*(window_size.height as f64)/2.0],
            default_trans_step: 100.,
            background_color: [0.2, 0.2, 0.2, 1.0],
            text_color: [1.0; 4],
            domino_height: 20.0,
            domino_width: 10.0,
            zoom_factor: 7.0,
            default_zoom_step: 1.5
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
        CCAStateView {
            settings: settings,
        }
    }

    fn trit_color(tbd: &TritBitDomino) -> Color {
        [0.3;4]
    }

    /// Draw state.
    pub fn draw<G: Graphics>(&self, controller: &CCAStateController, c: &mut Context, g: &mut G) {
        use graphics::{Rectangle,Transformed};

        let ref settings = self.settings;

        /* Set the view to correct origin and scale. */
        c.transform = c.transform.scale(settings.zoom_factor,settings.zoom_factor);
        c.transform = c.transform.trans(settings.origin[0]/settings.zoom_factor,
                                        settings.origin[1]/settings.zoom_factor);

        let mut curr_abstract_pos = [0., 0.];
        for i in 0..controller.state.cells.len() {
            
            let px_pos = [(curr_abstract_pos[0] as f64)*settings.domino_width, 
                          (curr_abstract_pos[1] as f64)*settings.domino_height];

            let domino_trit_rect = [
                px_pos[0], px_pos[1],
                settings.domino_width, settings.domino_height/2.0,
            ];

            let domino_bit_rect = [
                px_pos[0], px_pos[1]+settings.domino_height/2.0,
                settings.domino_width, settings.domino_height/2.0,
            ];

            let tbd: &TritBitDomino = controller.state.cells.get(i).unwrap();

            Rectangle::new(CCAStateView::trit_color(tbd))
                .draw(domino_trit_rect, &c.draw_state, c.transform, g);

            Rectangle::new(settings.text_color)
                .draw(domino_bit_rect, &c.draw_state, c.transform, g);

            curr_abstract_pos[0] -= 1.5;
            curr_abstract_pos[1] -= 0.5;
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
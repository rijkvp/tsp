use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::thread;
use std::time::Duration;

use crate::annealing::{Annealing, Params};

const PADDING: f32 = 26.0;

pub struct Visualizer {
    cities: Vec<(f64, f64)>,
    path: Option<Vec<usize>>,
    annealing: Annealing,
    show_numbers: bool,
    font: Font,
}

impl Visualizer {
    pub fn new(cities: Vec<(f64, f64)>) -> Self {
        let bytes = include_bytes!("../res/RobotoMono-Bold.ttf");
        let font = Font::new(bytes).unwrap();
        let annealing = Annealing::new(cities.clone(), Params::default());
        Self {
            cities,
            path: None,
            annealing,
            show_numbers: false,
            font,
        }
    }

    pub fn run(self) {
        let window = Window::new_centered(
            "Visualizer",
            (
                (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
                (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
            ),
        )
        .unwrap();

        window.run_loop(self);
    }
}

// pub fn visualize(title: &str, cities: Vec<(f64, f64)>, path: Vec<usize>) {
//     let window = Window::new_centered(
//         title,
//         (
//             (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
//             (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
//         ),
//     )
//     .unwrap();

//     let bytes = include_bytes!("../res/RobotoMono-Bold.ttf");
//     let font = Font::new(bytes).unwrap();
//     window.run_loop(Visualizer {
//         cities,
//         path,
//         font,
//         show_numbers: false,
//     });
// }

fn transform_position(i: (f64, f64)) -> (f32, f32) {
    (
        PADDING + ((i.0 + crate::AREA_SIZE) / 2.0) as f32,
        PADDING + ((i.1 + crate::AREA_SIZE) / 2.0) as f32,
    )
}

impl WindowHandler for Visualizer {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.annealing.step();
        let (_, path) = self.annealing.get();
        self.path = Some(path.to_vec());

        graphics.clear_screen(Color::from_gray(0.8));
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (0.0, 0.0),
                (
                    (crate::AREA_SIZE as f32 + 2.0 * PADDING),
                    (crate::AREA_SIZE as f32 + 2.0 * PADDING),
                ),
            ),
            Color::from_gray(0.7),
        );
        if let Some(path) = &self.path {
            for i in 0..path.len() {
                let start = self.cities[path[i]];
                let end = self.cities[path[(i + 1) % path.len()]];
                graphics.draw_line(
                    transform_position(start),
                    transform_position(end),
                    2.0,
                    Color::DARK_GRAY,
                );
            }
        }
        for (n, city) in self.cities.iter().enumerate() {
            let pos = transform_position(*city);
            graphics.draw_circle(pos, 5.0, Color::from_rgb(0.7, 0.2, 0.2));
            if self.show_numbers {
                let block = self
                    .font
                    .layout_text(&n.to_string(), 18.0, TextOptions::new());
                graphics.draw_text((pos.0 - 9.0, pos.1 - 9.0), Color::BLACK, &block);
            }
        }
        thread::sleep(Duration::from_millis(50));
        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper,
        key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if key_code == Some(VirtualKeyCode::N) {
            self.show_numbers = !self.show_numbers;
            helper.request_redraw();
        }
    }
}

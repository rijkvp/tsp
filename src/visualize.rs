use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use crate::algo::TspAlgorithm;

const PADDING: f32 = 32.0;
const FONT_SIZE: f32 = 24.0;
const TEXT_PADDING: f32 = 4.0;

pub struct Visualizer<T: TspAlgorithm> {
    area: f32,
    cities: Vec<(f64, f64)>,
    path: Option<Vec<usize>>,
    state: T,
    running: bool,
    steps_per_frame: usize,
    show_numbers: bool,
    status: String,
    length: f64,
    font: Font,
}

impl<T: TspAlgorithm + 'static> Visualizer<T> {
    pub fn new(cities: Vec<(f64, f64)>, area: f64) -> Self {
        let bytes = include_bytes!("../res/RobotoMono-Bold.ttf");
        let font = Font::new(bytes).unwrap();
        let state = T::init(cities.clone());
        Self {
            area: area as f32,
            cities,
            path: None,
            state,
            running: true,
            steps_per_frame: 10,
            show_numbers: false,
            status: String::new(),
            length: 0.0,
            font,
        }
    }

    pub fn run(self) {
        let window = Window::new_centered(
            "Visualizer",
            (
                (2.0 * PADDING + self.area) as u32,
                (2.0 * PADDING + self.area) as u32,
            ),
        )
        .unwrap();

        window.run_loop(self);
    }
}

fn transform_position(i: (f64, f64), a: f32) -> (f32, f32) {
    (
        PADDING + (i.0 as f32 + a / 2.0),
        PADDING + (i.1 as f32 + a / 2.0),
    )
}

impl<T: TspAlgorithm> WindowHandler for Visualizer<T> {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.running {
            for _ in 0..self.steps_per_frame {
                if self.state.step() {
                    self.running = false;
                    break;
                }
            }
            let (length, path, status) = self.state.state();
            self.path = Some(path.to_vec());
            self.length = length;
            self.status = status;
        }

        graphics.clear_screen(Color::from_gray(0.8));
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (0.0, 0.0),
                ((self.area + 2.0 * PADDING), (self.area + 2.0 * PADDING)),
            ),
            Color::from_gray(0.7),
        );
        if let Some(path) = &self.path {
            for i in 0..path.len() {
                let start = self.cities[path[i]];
                let end = self.cities[path[(i + 1) % path.len()]];
                graphics.draw_line(
                    transform_position(start, self.area),
                    transform_position(end, self.area),
                    2.0,
                    Color::DARK_GRAY,
                );
            }
        }
        for (n, city) in self.cities.iter().enumerate() {
            let pos = transform_position(*city, self.area);
            graphics.draw_circle(pos, 5.0, Color::from_rgb(0.7, 0.2, 0.2));
            if self.show_numbers {
                let block = self
                    .font
                    .layout_text(&n.to_string(), FONT_SIZE, TextOptions::new());
                graphics.draw_text(
                    (pos.0 - block.width() / 2.0, pos.1 - FONT_SIZE),
                    Color::BLACK,
                    &block,
                );
            }
        }

        let block = self
            .font
            .layout_text(&self.status, FONT_SIZE, TextOptions::new());
        graphics.draw_text(
            (PADDING + self.area - block.width(), TEXT_PADDING),
            Color::BLACK,
            &block,
        );
        let block = self.font.layout_text(
            &format!("Length={:.1}", self.length),
            FONT_SIZE,
            TextOptions::new(),
        );
        graphics.draw_text((PADDING, TEXT_PADDING), Color::BLACK, &block);
        let text = {
            if self.running {
                format!("Speed={}/f", self.steps_per_frame)
            } else {
                "Finished :)".to_string()
            }
        };
        let block = self.font.layout_text(&text, FONT_SIZE, TextOptions::new());
        graphics.draw_text(
            (
                PADDING,
                PADDING * 2.0 + self.area - block.height() - TEXT_PADDING,
            ),
            Color::BLACK,
            &block,
        );
        if self.running {
            helper.request_redraw();
        }
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = key_code {
            match key_code {
                VirtualKeyCode::N => {
                    self.show_numbers = !self.show_numbers;
                }
                VirtualKeyCode::F => {
                    self.steps_per_frame *= 2;
                }
                VirtualKeyCode::S => {
                    self.steps_per_frame /= 2;
                }
                _ => {}
            }
        }
    }
}

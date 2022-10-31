use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use crate::algo::{TspAlgorithm, TspState};

const PADDING: f32 = 32.0;
const FONT_SIZE: f32 = 24.0;
const TEXT_PADDING: f32 = 4.0;

pub struct Visualizer<T: TspAlgorithm> {
    area: f32,
    cities: Vec<(f64, f64)>,
    algo: T,
    state: Option<TspState>,
    paused: bool,
    running: bool,
    steps_per_frame: usize,
    show_numbers: bool,
    show_samples: bool,
    font: Font,
}

impl<T: TspAlgorithm + 'static> Visualizer<T> {
    pub fn new(cities: Vec<(f64, f64)>, area: f64) -> Self {
        let bytes = include_bytes!("../res/RobotoMono-Bold.ttf");
        let font = Font::new(bytes).unwrap();
        let algo = T::init(cities.clone());
        Self {
            area: area as f32,
            cities,
            state: None,
            algo,
            paused: true,
            running: true,
            steps_per_frame: 8,
            show_numbers: false,
            show_samples: false,
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
        if self.running && !self.paused {
            for _ in 0..self.steps_per_frame {
                if self.algo.step() {
                    self.running = false;
                    break;
                }
            }
            self.state = Some(self.algo.state());
        }

        graphics.clear_screen(Color::from_gray(0.8));
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (0.0, 0.0),
                ((self.area + 2.0 * PADDING), (self.area + 2.0 * PADDING)),
            ),
            Color::from_gray(0.7),
        );
        if let Some(state) = &self.state {
            if self.show_samples {
                for i in 0..state.sample.len() {
                    let start = self.cities[state.sample[i]];
                    let end = self.cities[state.sample[(i + 1) % state.sample.len()]];
                    graphics.draw_line(
                        transform_position(start, self.area),
                        transform_position(end, self.area),
                        2.0,
                        Color::from_rgba(0.25, 0.25, 0.25, 200.0),
                    );
                }
            }
            for i in 0..state.path.len() {
                let start = self.cities[state.path[i]];
                let end = self.cities[state.path[(i + 1) % state.path.len()]];
                graphics.draw_line(
                    transform_position(start, self.area),
                    transform_position(end, self.area),
                    3.0,
                    Color::DARK_GRAY,
                );
            }
            let block = self
                .font
                .layout_text(&state.status, FONT_SIZE, TextOptions::new());
            graphics.draw_text(
                (PADDING + self.area - block.width(), TEXT_PADDING),
                Color::BLACK,
                &block,
            );
            let block = self.font.layout_text(
                &format!("Length={:.1}", state.length),
                FONT_SIZE,
                TextOptions::new(),
            );
            graphics.draw_text((PADDING, TEXT_PADDING), Color::BLACK, &block);
        }
        for (n, city) in self.cities.iter().enumerate() {
            let pos = transform_position(*city, self.area);
            graphics.draw_circle(pos, 6.0, Color::from_rgb(0.7, 0.2, 0.2));
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

        let text = {
            if !self.running {
                "Finished :)".to_string()
            } else if !self.paused {
                format!("Speed={}/f", self.steps_per_frame)
            } else {
                "Paused (press space to start)".to_string()
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
        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = key_code {
            match key_code {
                VirtualKeyCode::Space => self.paused = !self.paused,
                VirtualKeyCode::N => self.show_numbers = !self.show_numbers,
                VirtualKeyCode::F => self.steps_per_frame = (2*self.steps_per_frame).min(16777216),
                VirtualKeyCode::S => self.steps_per_frame = (self.steps_per_frame/2).max(1),
                VirtualKeyCode::P => self.show_samples = !self.show_samples,
                _ => {}
            }
        }
    }
}

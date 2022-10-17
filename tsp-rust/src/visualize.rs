use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const PADDING: f32 = 26.0;

struct Visualizor {
    cities: Vec<(f64, f64)>,
    path: Vec<usize>,
    show_numbers: bool,
    font: Font,
}

pub fn visualize(title: &str, cities: Vec<(f64, f64)>, path: Vec<usize>) {
    let window = Window::new_centered(
        title,
        (
            (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
            (2.0 * PADDING as f64 + crate::AREA_SIZE) as u32,
        ),
    )
    .unwrap();

    let bytes = include_bytes!("../res/RobotoMono-Bold.ttf");
    let font = Font::new(bytes).unwrap();
    window.run_loop(Visualizor {
        cities,
        path,
        font,
        show_numbers: false,
    });
}

fn transform_position(i: (f64, f64)) -> (f32, f32) {
    (
        PADDING + ((i.0 + crate::AREA_SIZE) / 2.0) as f32,
        PADDING + ((i.1 + crate::AREA_SIZE) / 2.0) as f32,
    )
}

impl WindowHandler for Visualizor {
    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
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
        for i in 0..self.path.len() {
            let start = self.cities[self.path[i]];
            let end = self.cities[self.path[(i + 1) % self.path.len()]];
            graphics.draw_line(
                transform_position(start),
                transform_position(end),
                2.0,
                Color::DARK_GRAY,
            );
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

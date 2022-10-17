use speedy2d::color::Color;
use speedy2d::font::Font;
use speedy2d::font::TextOptions;
use speedy2d::font::TextLayout;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

pub fn visualize(title: &str, cities: Vec<(f64, f64)>, path: Vec<usize>) {
    let window = Window::new_centered(title, (500, 500)).unwrap();

    
    let bytes = include_bytes!("/usr/share/fonts/liberation/LiberationMono-Regular.ttf");
    let font = Font::new(bytes).unwrap();
    window.run_loop(Visualizor { cities, path, font });
}

struct Visualizor {
    cities: Vec<(f64, f64)>,
    path: Vec<usize>,
    font: Font,
}

fn transform_position(i: (f64, f64)) -> (f32, f32) {
    (((i.0 + crate::AREA_SIZE)/2.0) as f32, ((i.1 + crate::AREA_SIZE)/2.0) as f32)
}

impl WindowHandler for Visualizor {
    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        for i in 0..self.path.len() {
            let start = self.cities[self.path[i]];
            let end = self.cities[self.path[(i+1) % self.path.len()]];
            graphics.draw_line(
                transform_position(start),
                transform_position(end),
                3.0,
                Color::WHITE,
            );
        }
        for (n, city) in self.cities.iter().enumerate() {
            let pos = transform_position(*city);
            graphics.draw_circle(pos, 10.0, Color::YELLOW);
            let block = self.font.layout_text(&format!("#{}", n), 24.0, TextOptions::new());
            graphics.draw_text(pos, Color::BLUE, &block);
            
        }
    }
}

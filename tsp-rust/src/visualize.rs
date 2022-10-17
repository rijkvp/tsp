use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

pub fn visualize(cities: Vec<(f64, f64)>, path: Vec<usize>) {
    let window = Window::new_centered("Path Visualization", (600, 400)).unwrap();

    window.run_loop(Visualizor { cities, path });
}

struct Visualizor {
    cities: Vec<(f64, f64)>,
    path: Vec<usize>,
}

impl WindowHandler for Visualizor {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);

        for city in self.cities.iter() {
            graphics.draw_circle((city.0 as f32, city.1 as f32), 10.0, Color::YELLOW);
        }
        for i in 0..self.path.len() {
            let start = self.cities[self.path[i]];
            let end = self.cities[self.path[(i + 1) % (self.path.len() - 1)]];
            graphics.draw_line(
                (start.0 as f32, start.1 as f32),
                (end.0 as f32, end.1 as f32),
                5.0,
                Color::WHITE,
            );
        }

        helper.request_redraw();
    }
}

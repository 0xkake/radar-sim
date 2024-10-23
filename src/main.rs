use eframe::egui::{self, Color32, Rect, Pos2, ViewportBuilder};

const X_SIZE: usize = 500;
const Y_SIZE: usize = 300;

const DT: f32 = 0.1;
const C: f32 = 1.0;

struct Matrix {
    x: usize,
    y: usize,
    data: Vec<f32>,
}

impl Matrix {
    fn new(x: usize, y: usize) -> Self {
        Matrix { x, y, data: vec![0.0; x * y] }
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, (x, y): (usize, usize)) -> &f32 {
        &self.data[x + y * self.x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut f32 {
        &mut self.data[x + y * self.x]
    }
}


struct EMField {
    ez: Matrix,
    hx: Matrix,
    hy: Matrix,
    time: f32
}

impl EMField {
    fn new() -> Self {
        let ez = Matrix::new(X_SIZE, Y_SIZE);
        let hx = Matrix::new(X_SIZE, Y_SIZE);
        let hy = Matrix::new(X_SIZE, Y_SIZE);
        
        EMField { ez, hx, hy, time: 0.0 }
    }

    fn update(&mut self) {
        for y in 0..Y_SIZE-1 {
            for x in 0..X_SIZE-1 {
                self.hx[(x, y)] -= DT * (self.ez[(x, y+1)] - self.ez[(x, y)]);
                self.hy[(x, y)] += DT * (self.ez[(x+1, y)] - self.ez[(x, y)]);
            }
        }

        for y in 1..Y_SIZE-1 {
            for x in 1..X_SIZE-1 {
                self.ez[(x, y)] += C * DT * (
                    (self.hy[(x, y)] - self.hy[(x-1, y)]) -
                    (self.hx[(x, y)] - self.hx[(x, y-1)])
                );
            }
        }

        self.ez[(1, 100)] = (self.time * 0.25).sin();
        self.ez[(1, 125)] = (self.time * 0.25).sin();
        self.ez[(1, 150)] = (self.time * 0.25).sin();
        self.ez[(1, 175)] = (self.time * 0.25).sin();
        self.ez[(1, 200)] = (self.time * 0.25).sin();

        self.time += DT;
    }
}

struct EMWaveApp {
    field: EMField,
}

impl Default for EMWaveApp {
    fn default() -> Self {
        Self {
            field: EMField::new(),
        }
    }
}

impl eframe::App for EMWaveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t0 = std::time::Instant::now();

        for _ in 0..15 { self.field.update(); }

        println!("{:?}", t0.elapsed());

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_width = (ui.available_size().x / X_SIZE as f32).floor();
            let cell_height = (ui.available_size().y / Y_SIZE as f32).floor();

            let (response, painter) = ui.allocate_painter(
                egui::vec2(cell_width * X_SIZE as f32, cell_height * Y_SIZE as f32),
                egui::Sense::hover(),
            );
            let rect = response.rect;

            // Draw each cell
            for x in 0..X_SIZE {
                for y in 0..Y_SIZE {
                    let value = self.field.ez[(x, y)] as f32;
                    
                    let alpha = (value * 2.0).min(1.0).max(-1.0);
                    let red = (alpha.max(0.0) * 255.0) as u8;
                    let blue = (-alpha.min(0.0) * 255.0) as u8;

                    let color = Color32::from_rgb(red, 0, blue);
                    
                    let cell_rect = Rect::from_min_size(
                        Pos2::new(
                            rect.min.x + x as f32 * cell_width,
                            rect.min.y + y as f32 * cell_height,
                        ),
                        egui::vec2(cell_width, cell_height),
                    );
                    
                    painter.rect_filled(cell_rect, 0.0, color);
                }
            }
        });

        // Request continuous repaint
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size((1020.0, 620.0)),
        ..Default::default()
    };


    
    eframe::run_native(
        "Radar simulation",
        options,
        Box::new(|_cc| Ok(Box::new(EMWaveApp::default()))),
    )
}
use eframe::egui::{self, Color32, Rect, Pos2};

const SIMULATION_SIZE: usize = 200;
const DT: f32 = 0.5;
const C: f32 = 1.0;
const SOURCE_FREQUENCY: f32 = 0.5;


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

    fn index(&self, (i, j): (usize, usize)) -> &f32 {
        &self.data[i * self.y + j]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
        &mut self.data[i * self.y + j]
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
        let ez = Matrix::new(SIMULATION_SIZE, SIMULATION_SIZE);
        let hx = Matrix::new(SIMULATION_SIZE, SIMULATION_SIZE);
        let hy = Matrix::new(SIMULATION_SIZE, SIMULATION_SIZE);
        
        EMField { ez, hx, hy, time: 0.0 }
    }

    fn update(&mut self) {
        let t0 = std::time::Instant::now();

        // Update magnetic field components
        for i in 0..SIMULATION_SIZE-1 {
            for j in 0..SIMULATION_SIZE-1 {
                self.hx[(i, j)] -= DT * (self.ez[(i, j+1)] - self.ez[(i, j)]);
                self.hy[(i, j)] += DT * (self.ez[(i+1, j)] - self.ez[(i, j)]);
            }
        }

        // Update electric field component
        for i in 1..SIMULATION_SIZE-1 {
            for j in 1..SIMULATION_SIZE-1 {
                self.ez[(i, j)] += C * DT * (
                    (self.hy[(i, j)] - self.hy[(i-1, j)]) -
                    (self.hx[(i, j)] - self.hx[(i, j-1)])
                );
            }
        }

        // Add source
        self.ez[(1, 20)] = (self.time * SOURCE_FREQUENCY).sin();
        self.ez[(1, 40)] = (self.time * SOURCE_FREQUENCY).sin();
        self.ez[(1, 60)] = (self.time * SOURCE_FREQUENCY).sin();
        self.ez[(1, 80)] = (self.time * SOURCE_FREQUENCY).sin();
        self.time += DT;

        println!("{:?}", t0.elapsed());
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
        self.field.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let cell_size = (available_size.x.min(available_size.y) / SIMULATION_SIZE as f32).floor();
            
            // Create a custom painting canvas
            let (response, painter) = ui.allocate_painter(
                egui::vec2(cell_size * SIMULATION_SIZE as f32, cell_size * SIMULATION_SIZE as f32),
                egui::Sense::hover(),
            );
            
            let rect = response.rect;
            
            // Draw each cell
            for i in 0..SIMULATION_SIZE {
                for j in 0..SIMULATION_SIZE {
                    let value = self.field.ez[(i, j)] as f32;
                    
                    let color = if value > 0.0 {
                        let intensity = 1.0 - value.min(1.0);
                        Color32::from_rgb(255, (intensity * 255.0) as u8, (intensity * 255.0) as u8)
                    } else {
                        let intensity = 1.0 + value.max(-1.0);
                        Color32::from_rgb((intensity * 255.0) as u8, (intensity * 255.0) as u8, 255)
                    };
                    
                    let cell_rect = Rect::from_min_size(
                        Pos2::new(
                            rect.min.x + i as f32 * cell_size,
                            rect.min.y + j as f32 * cell_size,
                        ),
                        egui::vec2(cell_size, cell_size),
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
        ..Default::default()
    };
    
    eframe::run_native(
        "Radar simulation",
        options,
        Box::new(|_cc| Ok(Box::new(EMWaveApp::default()))),
    )
}
use ggez::graphics::{self, Canvas, Color, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct ConnectingComponent {
    points: Vec<Point2<f32>>,
    is_drawing: bool,
}

impl ConnectingComponent {
    pub fn new() -> Self {
        ConnectingComponent {
            points: Vec::new(),
            is_drawing: false,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        
        for window in self.points.windows(2) {
            if let [start, end] = window {
                let line = graphics::Mesh::new_line(
                    ctx, 
                    &[*start, *end], 
                    2.0, 
                    Color::BLACK)?;
                canvas.draw(&line, graphics::DrawParam::default());
            }
        }
        // let mesh = mesh_builder.build();
        // let wire_piece = graphics::Mesh::new_line(
            // ctx, 
            // &[Point2{ x: 0.0, y: 0.0}, Point2{ x:0.0, y:board_height}], 
            // 2.0, 
            // Color::BLACK)?;
        
        Ok(())
    }

    pub fn handle_mouse_down(&mut self, x: f32, y: f32, button: ggez::event::MouseButton) {
        match button {
            ggez::event::MouseButton::Left => {
                self.points.push(Point2{x, y});
                self.is_drawing = true;
            }
            ggez::event::MouseButton::Right => {
                self.is_drawing = false;
            }
            _ => {}
        }
    }
}

use ggez::glam::vec2;
use ggez::graphics::{self, Canvas, Color, Image};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use crate::gates::logic_gates::LogicGate;

pub struct DraggableComponent {
    pos: Point2<f32>,
    size: (f32, f32),
    img:Image,
    is_dragged: bool,
    drag_offset: Point2<f32>,
}

impl DraggableComponent {
    pub fn new(ctx: &mut Context,pos: Point2<f32>, width:u32, height:u32, img_path: &str) -> Self {
        let img = match graphics::Image::from_path(ctx, img_path){
            Ok(i) => i,
            Err(_) => { println!("error in opeing asset path: {}", img_path);
                        graphics::Image::from_color(ctx, width, height, Some(Color::GREEN)) }
        };
        DraggableComponent {
            pos,
            size: ((height as f32), (width as f32)),
            img,
            is_dragged: false,
            drag_offset: Point2{x:0.0, y:0.0},
        }
    }

    pub fn from_type(t: LogicGate, ctx: &mut Context, pos: Point2<f32>, width:u32, height:u32) -> Self{
        match t {
            LogicGate::AndGate => DraggableComponent::new(ctx, pos, width, height, "/images/and_gate.png"),
            LogicGate::OrGate => DraggableComponent::new(ctx, pos, width, height, "/images/or_gate.png"),
            LogicGate::NotGate => DraggableComponent::new(ctx, pos, width, height, "/images/not_gate.png"), 
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let img_param = graphics::DrawParam::default().dest(vec2(self.pos.x, self.pos.y));
        canvas.draw(&self.img, img_param);
        Ok(())
    }

    pub fn handle_mouse_down(&mut self, x: f32, y: f32) {
        if x >= self.pos.x && x <= self.pos.x + self.size.0 && y >= self.pos.y && y <= self.pos.y + self.size.1 {
            self.is_dragged = true;
            self.drag_offset = Point2{x: x - self.pos.x, y: y - self.pos.y};
        }
    }

    pub fn handle_mouse_up(&mut self) {
        self.is_dragged = false;
    }

    pub fn handle_mouse_motion(&mut self, x: f32, y: f32) {
        if self.is_dragged {
            self.pos.x = x - self.drag_offset.x;
            self.pos.y = y - self.drag_offset.y;
        }
    }
}
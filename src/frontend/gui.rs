use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Canvas, Color, Rect, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{Context, GameError, GameResult};
use ggez::glam::*;

use crate::gates::circuit;
use crate::LogicGate;
use crate::Circuit;
use crate::frontend::dragable::DraggableComponent;

const X_PAD:f32=10.0;
const Y_PAD:f32=10.0;

pub struct GameState {
    circuit: Circuit,
    dragables: Vec<DraggableComponent>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut s= GameState{
            circuit: Circuit::new(),
            dragables: vec![
                DraggableComponent::new(Point2{x: 100.0, y: 100.0}, (50.0, 50.0)),
                DraggableComponent::new(Point2{x: 200.0, y: 100.0}, (50.0, 50.0)),
                DraggableComponent::new(Point2{x: 300.0, y: 100.0}, (50.0, 50.0)),
            ],
        };
        s.reset(ctx)?;
        Ok(s)
    }
    pub fn reset(&mut self, ctx: &mut Context)-> GameResult<()>{
        Ok(())
    }

    pub fn draw_picker_box(&mut self, ctx: &mut Context,canvas: &mut Canvas, x_offset:f32, y_offset:f32) -> GameResult<()>{
        let picker_height = 100.0;
        let picker_width = 1500.0;
        let picker_area= graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(5.0), 
            Rect::new(0.0, 0.0, picker_width, picker_height), 
            Color::BLACK)?;
        
        let picker_border = graphics::Mesh::new_line(
            ctx, 
            &[Point2{ x: 0.0, y: 0.0}, Point2{ x:0.0, y:picker_height}], 
            5.0, 
            Color::BLACK)?;
        
        canvas.draw(&picker_area, Vec2::new(X_PAD+x_offset, Y_PAD+y_offset));
        for i in 1..15{
            canvas.draw(&picker_border, Vec2::new(X_PAD+x_offset+(i as f32*picker_height), Y_PAD+y_offset));
        }
        Ok(())
    }
    
    pub fn draw_title_text(&mut self, ctx: &mut Context, canvas: &mut Canvas, text_size:f32) -> GameResult<()> {
        let title = TextFragment::new("Logic Gate Game").color(Color::BLACK).scale(text_size);
        let title_text = Text::new(title);

        canvas.draw(&title_text, Vec2::new(X_PAD, Y_PAD));
        Ok(())
    }

    pub fn draw_breadboard(&mut self, ctx: &mut Context, canvas: &mut Canvas, x_offset:f32, y_offset:f32)->GameResult<()> {
        
        let board_height = 700.0;
        let board_width =1500.0;
        let slot_size=100.0;

        let board_area= graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(2.0), 
            Rect::new(0.0, 0.0, board_width, board_height), 
            Color::BLACK)?;
        let board_border_y = graphics::Mesh::new_line(
            ctx, 
            &[Point2{ x: 0.0, y: 0.0}, Point2{ x:0.0, y:board_height}], 
            2.0, 
            Color::BLACK)?;
        let board_border_x = graphics::Mesh::new_line(
            ctx, 
            &[Point2{ x: 0.0, y: 0.0}, Point2{ x:board_width, y:0.0}], 
            2.0, 
            Color::BLACK)?;
        
        canvas.draw(&board_area, Vec2::new(X_PAD+x_offset, Y_PAD+y_offset));
        for i in 1..15{
            canvas.draw(&board_border_y, Vec2::new(X_PAD+x_offset+(i as f32*slot_size), Y_PAD+y_offset));
        }

        for i in 1..9{
            canvas.draw(&board_border_x, Vec2::new(X_PAD+x_offset, Y_PAD+y_offset+(i as f32*slot_size)));
        }
        
        Ok(())
    }

    pub fn draw_level_info(&mut self, ctx: &mut Context, canvas: &mut Canvas, x_offset:f32, y_offset:f32)->GameResult<()>{
        let level_info_height=100.0+20.0+700.0;
        let level_info_width= 300.0;

        let level_area= graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(5.0), 
            Rect::new(0.0, 0.0, level_info_width, level_info_height), 
            Color::BLACK)?;
        
        canvas.draw(&level_area, Vec2::new(X_PAD+x_offset, Y_PAD+y_offset));

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::WHITE,
        );
        let title_size = 100.0;
        self.draw_title_text(ctx, &mut canvas, title_size)?;
        
        self.draw_picker_box(ctx, &mut canvas, 0.0, title_size+5.0)?;

        self.draw_breadboard(ctx, &mut canvas, 0.0, title_size+100.0+20.0)?;

        self.draw_level_info(ctx, &mut canvas, 1500.0+10.0, 100.0+5.0)?;

        for dragable in &self.dragables{
            dragable.draw(ctx, &mut canvas)?
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        if button == MouseButton::Left {
            for component in &mut self.dragables {
                component.handle_mouse_down(x, y);
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        if button == MouseButton::Left {
            for component in &mut self.dragables {
                component.handle_mouse_up();
            }
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y:f32,  dx: f32, dy: f32)-> Result<(), GameError> {
        for component in &mut self.dragables {
            component.handle_mouse_motion(x, y);
        }
        Ok(())
    }
}
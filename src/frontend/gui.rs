use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;

use crate::gates::circuit;
use crate::LogicGate;
use crate::Circuit;


pub struct GameState {
    circuit: Circuit,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut s= GameState{
            circuit: Circuit::new(),
        };
        s.reset(ctx)?;
        Ok(s)
    }
    pub fn reset(&mut self, ctx: &mut Context)-> GameResult<()>{
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

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::BLACK,
        )?;
        canvas.draw(&circle, Vec2::new(100.0, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}
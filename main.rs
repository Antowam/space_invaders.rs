use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{graphics, nalgebra as na, timer};
use ggez::input::keyboard;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
    pos_y: f32,
    speed: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 0.0, speed: 5.0 };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
        if keyboard::is_key_pressed(ctx, KeyCode::Right)    { self.pos_x += self.speed; }
        if keyboard::is_key_pressed(ctx, KeyCode::Left)     { self.pos_x -= self.speed; }
        if keyboard::is_key_pressed(ctx, KeyCode::Down)     { self.pos_y += self.speed; }
        if keyboard::is_key_pressed(ctx, KeyCode::Up)       { self.pos_y -= self.speed; }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        // Create a circle at `position_x` and draw
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            na::Point2::new(self.pos_x, self.pos_y),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Q => {
                ggez::event::quit(ctx);
            }
            _ => (),
        }
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    ggez::event::run(ctx, event_loop, state)
}

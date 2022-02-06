use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct MainState {
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        MainState {
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Rong", "lguist")
        .build()
        .expect("Could not create ggez context!");

    graphics::set_window_title(&ctx, "Rong");
    let state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, state);
}

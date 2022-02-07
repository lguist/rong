use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};
use nalgebra::{Point2};

const RACKET_HEIGHT: f32 = 150.0;
const RACKET_WIDTH: f32 = 30.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

struct MainState {
    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (_screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);
        MainState {
            player_1_pos : Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos : Point2::new(screen_w-RACKET_WIDTH_HALF, screen_h_half),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let racket = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket, Color::WHITE)?;

        let mut draw_param = DrawParam::default();

        draw_param = draw_param.dest(self.player_1_pos);
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param = draw_param.dest(self.player_2_pos);
        graphics::draw(ctx, &racket_mesh, draw_param)?;

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

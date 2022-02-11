use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{self, KeyCode};
use nalgebra::{Point2};

const RACKET_HEIGHT: f32 = 150.0;
const RACKET_WIDTH: f32 = 30.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
	if *value < low {
		*value = low;
	} else if *value > high {
		*value = high;
	}
} 

fn move_racket(pos: &mut Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
	let dt = ggez::timer::delta(ctx).as_secs_f32();
	let screen_h = graphics::drawable_size(ctx).1;
	if keyboard::is_key_pressed(ctx, keycode) {
		pos.y += y_dir * PLAYER_SPEED * dt;
	}
	clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h-RACKET_HEIGHT_HALF);
}

struct MainState {
    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,
	ball_pos: Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);
        MainState {
            player_1_pos : Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos : Point2::new(screen_w-RACKET_WIDTH_HALF, screen_h_half),
			ball_pos: Point2::new(screen_w_half, screen_h_half),
		}
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
		move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
		move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
		move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, Color::WHITE)?;
		let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
		let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, Color::WHITE)?;
		
        let mut draw_param = DrawParam::default();

        draw_param = draw_param.dest(self.player_1_pos);
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param = draw_param.dest(self.player_2_pos);
        graphics::draw(ctx, &racket_mesh, draw_param)?;

		draw_param = draw_param.dest(self.ball_pos);
		graphics::draw(ctx, &ball_mesh, draw_param)?;

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

use tetra::graphics::{self, Color, Texture, Rectangle, DrawParams};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::window;

const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player1.draw(ctx);
        self.player2.draw(ctx);
        self.ball.draw(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        self.ball.position += self.ball.velocity;

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x < 0.0 {
            window::quit(ctx);
            println!("Player 2 wins!");
        } else if self.ball.position.x > WINDOW_WIDTH {
            window::quit(ctx);
            println!("Player 1 wins!");
        }

        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_texture_height = player1_texture.height() as f32;
        let player1 = Entity::new(
            player1_texture,
            Vec2::new(32.0, (WINDOW_HEIGHT - player1_texture_height) / 2.0)
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_texture_height = player2_texture.height() as f32;
        let player2 = Entity::new(
            player2_texture,
            Vec2::new(WINDOW_WIDTH - 32.0, (WINDOW_HEIGHT - player2_texture_height) / 2.0)
        );

        let ball_texture = Texture::new(ctx, "./resources/ballGrey.png")?;
        let ball_width = ball_texture.width() as f32;
        let ball_height = ball_texture.height() as f32;
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball = Entity::with_velocity(
            ball_texture,
            Vec2::new((WINDOW_WIDTH - ball_width) / 2.0, (WINDOW_HEIGHT - ball_height) / 2.0),
            ball_velocity,
        );

        Ok(GameState { player1, player2, ball })
    }
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity: Vec2::zero(),
        }
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.texture,
            self.position,
        );
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

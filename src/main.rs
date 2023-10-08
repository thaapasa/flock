extern crate ggez;
use ggez::{event, graphics, Context, GameResult, graphics::Canvas};

const NUM_BOIDS: usize = 100;

#[derive(Clone, Copy)]
struct Boid {
    position: MyVector2,
    velocity: MyVector2,
}

// Wrapper for the Vector2 type
#[derive(Clone, Copy)]
pub struct MyVector2(pub ggez::mint::Vector2<f32>);

// Implement addition and assignment operations for the wrapper type
use std::ops::{Add, AddAssign};
impl Add for MyVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MyVector2(ggez::mint::Vector2 {
            x: self.0.x + rhs.0.x,
            y: self.0.y + rhs.0.y,
        })
    }
}

impl AddAssign for MyVector2 {
    fn add_assign(&mut self, rhs: MyVector2) {
        self.0.x += rhs.0.x;
        self.0.y += rhs.0.y;
    }
}

struct MainState {
    boids: Vec<Boid>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut boids = Vec::with_capacity(NUM_BOIDS);
        for _ in 0..NUM_BOIDS {
            boids.push(Boid {
                position: MyVector2(ggez::mint::Vector2 {
                    x: rand::random::<f32>() * 800.0,
                    y: rand::random::<f32>() * 600.0,
                }),
                velocity: MyVector2(ggez::mint::Vector2 {
                    x: (rand::random::<f32>() - 0.5) * 2.0,
                    y: (rand::random::<f32>() - 0.5) * 2.0,
                }),
            });
        }

        let s = MainState { boids };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for boid in &mut self.boids {
            boid.position += boid.velocity;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK); // Create canvas to draw on the screen

        // canvas.clear(graphics::Color::BLACK)?; // Clear the canvas with black color

        for boid in &self.boids {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(boid.position.0.x, boid.position.0.y, 2.0, 10.0),
                graphics::Color::WHITE,
            )?;
            canvas.draw(&rect, graphics::DrawParam::default()); // Queue draw calls on the canvas
        }

        canvas.finish(ctx)?; // Submit the draw queue

        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("flocking", "ggez").build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}
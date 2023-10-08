use ggez::{event, graphics, Context, GameResult};
use ggez::mint as na;

const NUM_BOIDS: usize = 100;
const BOID_RADIUS: f32 = 2.0;
const NEIGHBOR_RADIUS: f32 = 50.0;
const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.05;

#[derive(Clone, Copy)]
struct Boid {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

use std::ops::AddAssign;

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        let velocity = na::Vector2 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,  
        };
        Boid {
            position: na::Point2::from([x, y]),
            velocity,
        }
    }

    fn align(&self, boids: &[Boid]) -> na::Vector2<f32> {
        let mut steering = na::Vector2::from([0.0, 0.0]);
        let mut total = 0;

        for boid in boids {
            let distance = na::distance(&self.position, &boid.position);
            if distance > 0.0 && distance < NEIGHBOR_RADIUS {
                steering += boid.velocity;
                total += 1;
            }
        }

        if total > 0 {
            steering /= total as f32;
            steering = steering.normalize() * MAX_SPEED;
            steering -= self.velocity;
            steering = na::Vector2::new(
                steering.x.max(-MAX_FORCE).min(MAX_FORCE),
                steering.y.max(-MAX_FORCE).min(MAX_FORCE),
            );
        }

        steering
    }

    // You'll need to implement `cohesion` and `separation` functions similar to `align`.

    fn update(&mut self, boids: &[Boid]) {
        let alignment = self.align(boids);
        // let cohesion = self.cohesion(boids);
        // let separation = self.separation(boids);
        // Add the above functions to the velocity
        self.velocity += alignment; // + cohesion + separation;
        self.velocity = self.velocity.normalize() * MAX_SPEED;
        self.position += self.velocity;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            BOID_RADIUS,
            0.1,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))
    }
}

struct MainState {
    boids: Vec<Boid>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut boids = Vec::with_capacity(NUM_BOIDS);
        for _ in 0..NUM_BOIDS {
            boids.push(Boid::new(rand::random::<f32>() * 800.0, rand::random::<f32>() * 600.0));
        }

        Ok(MainState { boids })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for boid in &mut self.boids {
            boid.update(&self.boids);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::BLACK);
        for boid in &self.boids {
            boid.draw(ctx)?;
        }
        graphics::present(ctx)
    }
}

fn main() -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("boid_flocking", "author_name");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
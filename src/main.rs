use ggez::{Context, GameResult, graphics, event::EventHandler};
use ggez::event;
use nalgebra as na;
use ggez::graphics::Canvas;

const BOID_COUNT: usize = 150;

struct Boid {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

impl Boid {
    fn new(_x: f32, _y: f32) -> Self {
        Boid {
            position: na::Point2::new(
                rand::random::<f32>() * 1600.0,
                rand::random::<f32>() * 1200.0,
            ),
            velocity: na::Vector2::new(
                (rand::random::<f32>() - 0.5) * 2.0,
                (rand::random::<f32>() - 0.5) * 2.0,
            ),
        }
    }
}

// Define the extension trait
trait NalgebraMintConversions {
    fn to_mint(&self) -> mint::Point2<f32>;
    fn from_mint(point: mint::Point2<f32>) -> Self;
}

// Implement the extension trait for nalgebra::Vector2<f32>
impl NalgebraMintConversions for na::Vector2<f32> {
    fn to_mint(&self) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x,
            y: self.y,
        }
    }

    fn from_mint(point: mint::Point2<f32>) -> Self {
        na::Vector2::new(point.x, point.y)
    }
}

struct MainState {
    boids: Vec<Boid>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut boids = Vec::with_capacity(BOID_COUNT);
        for _ in 0..BOID_COUNT {
            boids.push(Boid::new(400.0, 300.0)); // Start all boids at the center
        }

        Ok(MainState { boids })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for i in 0..self.boids.len() {
            let mut separation = na::Vector2::new(0.0, 0.0 );
            let mut alignment = na::Vector2::new(0.0, 0.0);
            let mut cohesion = na::Vector2::new(0.0, 0.0);
            
            let mut neighbor_count = 0;
            
            for j in 0..self.boids.len() {
                if i != j {
                    let distance = na::distance(&self.boids[i].position, &self.boids[j].position);
                    
                    // Separation
                    if distance < SEPARATION_DISTANCE {
                        separation += self.boids[i].position - self.boids[j].position;
                    }
                    
                    // Alignment and Cohesion
                    if distance < NEIGHBOR_DISTANCE {
                        alignment += self.boids[j].velocity;
                        cohesion += na::Vector2::new(self.boids[j].position.x, self.boids[j].position.y);
                        neighbor_count += 1;
                    }
                }
            }
    
            // Calculate average for alignment and cohesion
            if neighbor_count > 0 {
                alignment /= neighbor_count as f32;
                alignment = alignment.normalize() * MAX_SPEED - self.boids[i].velocity;
    
                cohesion /= neighbor_count as f32;
                cohesion -= na::Vector2::new(self.boids[i].position.x, self.boids[i].position.y);
            }
    
            // Limit the magnitude of the vectors
            separation = limit_magnitude(separation, MAX_FORCE);
            alignment = limit_magnitude(alignment, MAX_FORCE);
            cohesion = limit_magnitude(cohesion, MAX_FORCE);
    
            // Apply the rules to the boid's velocity
            self.boids[i].velocity += separation * SEPARATION_WEIGHT;
            self.boids[i].velocity += alignment * ALIGNMENT_WEIGHT;
            self.boids[i].velocity += cohesion * COHESION_WEIGHT;
            self.boids[i].velocity = limit_magnitude(self.boids[i].velocity, MAX_SPEED);
    
            // Update position
            let vel =na::Vector2::new(self.boids[i].velocity.x, self.boids[i].velocity.y);
            self.boids[i].position += vel;
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
                graphics::Rect::new(boid.position.x, boid.position.y, 2.0, 10.0),
                graphics::Color::WHITE,
            )?;
            canvas.draw(&rect, graphics::DrawParam::default()); // Queue draw calls on the canvas
        }

        canvas.finish(ctx)?; // Submit the draw queue

        Ok(())
    }
}

fn limit_magnitude(vec: na::Vector2<f32>, max: f32) -> na::Vector2<f32> {
    if vec.magnitude() > max {
        vec.normalize() * max
    } else {
        vec
    }
}

const SEPARATION_DISTANCE: f32 = 25.0;
const NEIGHBOR_DISTANCE: f32 = 50.0;
const MAX_SPEED: f32 = 2.0;
const MAX_FORCE: f32 = 0.1;
const SEPARATION_WEIGHT: f32 = 1.5;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.0;

pub fn main() -> GameResult {
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("flocking", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Flock Simulation")) // Window title
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 1200.0)) // Window dimensions
        .build()
        .expect("Failed to build the game context");
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}
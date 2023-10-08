# Flocking Simulator

A simple flocking simulator implemented using Rust and the ggez game framework.

## Description

This project is a simulation of the flocking behavior of birds. Each bird, or "boid", operates under three basic principles:

- Alignment: Steer towards the average heading of local flockmates.
- Cohesion: Steer to move towards the average position of local flockmates.
- Separation: Steer to avoid crowding local flockmates.

## Setup & Running

### Prerequisites

Rust

### Running the Simulator

Clone the repository:

```bash
git clone https://github.com/thaapasa/flock.git
```

Navigate to the repository directory:

```bash
cd flock
```

Run the simulator:

```bash
cargo run
```

## Features

Boids move around the window simulating flocking behavior.

Uses nalgebra for vector mathematics.

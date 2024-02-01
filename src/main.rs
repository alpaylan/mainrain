use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct V2 {
    x: f64,
    y: f64,
}

impl Display for V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

fn rainToString(rain: &Vec<V2>) -> String {
    let mut result = String::new();
    for i in rain {
        result.push_str(&format!("({:.2}, {:.2}) ", i.x, i.y));
    }
    result
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }
}

impl Default for V2 {
    fn default() -> V2 {
        V2 { x: 0.0, y: 0.0 }
    }
}

struct SimulationParameters {
    pub object_size: V2,
    pub object_speed: f64,
    pub scene_size: V2,
    pub rain_speed: V2,
    pub rain_density: f64,
}

struct Simulation {
    pub parameters: SimulationParameters,
    pub object_position: V2,
    pub rain: Vec<V2>,
    pub score: u32,
}

impl Simulation {
    pub fn new(parameters: SimulationParameters) -> Simulation {
        Simulation {
            parameters,
            object_position: V2::default(),
            rain: Vec::new(),
            score: 0,
        }
    }

    pub fn update(&mut self, dt: f64) -> bool {
        // Update all raindrops
        for raindrop in &mut self.rain {
            raindrop.x += self.parameters.rain_speed.x * dt;
            raindrop.y += self.parameters.rain_speed.y * dt;
        }
        // Remove raindrops that are outside the scene
        self.rain.retain(|&raindrop| {
            raindrop.x < self.parameters.scene_size.x && raindrop.x > 0_f64 && raindrop.y > 0_f64
        });
        // Add new raindrops
        let new_raindrops =
            (self.parameters.rain_density * dt * self.parameters.scene_size.x) as usize;

        for _ in 0..new_raindrops {
            let position = V2::new(
                rand::random::<f64>() * self.parameters.scene_size.x,
                self.parameters.scene_size.y,
            );
            self.rain.push(position);
        }

        // Update object position
        self.object_position.x = self.object_position.x + self.parameters.object_speed * dt;

        // Check for collisions
        let mut toRemove = Vec::new();
        for (i, raindrop) in self.rain.iter().enumerate() {
            if self.object_position.x < raindrop.x
                && raindrop.x < self.object_position.x + self.parameters.object_size.x
                && self.object_position.y < raindrop.y
                && raindrop.y < self.object_position.y + self.parameters.object_size.y
            {
                println!("Collision with raindrop at {}", raindrop);
                self.score += 1;
                toRemove.push(i);
            }
        }

        // Remove collided raindrops
        for i in toRemove.iter().rev() {
            self.rain.remove(*i);
        }

        // Check if object is outside the scene
        self.object_position.x + self.parameters.object_size.x > self.parameters.scene_size.x
    }
}

fn main() {
    let parameters = SimulationParameters {
        object_size: V2::new(1.0, 1.0),
        object_speed: 0.5,
        scene_size: V2::new(100.0, 10.0),
        rain_speed: V2::new(0.0, -1.0),
        rain_density: 1.0,
    };

    let mut simulation = Simulation::new(parameters);

    let mut time = 0.0;
    while !simulation.update(0.1) {
        time += 0.1;
        println!(
            "Time: {:.1}, Number of collisions: {}",
            time, simulation.score
        );
        println!(
            "Object position: ({}, {})",
            simulation.object_position,
            V2::new(
                simulation.object_position.x + simulation.parameters.object_size.x,
                simulation.object_position.y + simulation.parameters.object_size.y
            )
        );
    }

    println!("Game over! Score: {}", simulation.score);
}

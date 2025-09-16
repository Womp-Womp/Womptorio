use crate::logistics;
use crate::save::WorldSnapshot;
use crate::ships;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{ExecutorKind, Schedule};
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};

/// Simulation configuration shared between the server and clients.
#[derive(Debug, Clone)]
pub struct SimConfig {
    pub seed: u64,
    pub fixed_time_step: u32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            seed: 0xDEADBEEF,
            fixed_time_step: 1,
        }
    }
}

/// Resource tracking the current simulation tick.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TimeState {
    pub tick: u64,
    pub fixed_time_step: u32,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            tick: 0,
            fixed_time_step: 1,
        }
    }
}

impl TimeState {
    pub fn new(fixed_time_step: u32) -> Self {
        Self {
            fixed_time_step,
            ..Default::default()
        }
    }
}

/// Deterministic random number generator resource.
#[derive(Resource, Debug, Clone)]
pub struct RandomState {
    seed: u64,
    rng: Pcg32,
}

impl RandomState {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: Pcg32::seed_from_u64(seed),
        }
    }

    pub fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Pcg32::seed_from_u64(seed);
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    pub fn rng_mut(&mut self) -> &mut Pcg32 {
        &mut self.rng
    }
}

/// Entry point for driving the simulation world.
pub struct SimWorld {
    config: SimConfig,
    world: World,
    schedule: Schedule,
}

impl SimWorld {
    pub fn new(config: SimConfig) -> Self {
        let mut world = World::default();
        world.insert_resource(TimeState::new(config.fixed_time_step));
        world.insert_resource(RandomState::new(config.seed));

        logistics::init_resources(&mut world);
        ships::init_resources(&mut world);

        logistics::seed_world(&mut world);
        ships::seed_world(&mut world);

        let mut schedule = Schedule::default();
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        logistics::configure_schedule(&mut schedule);
        ships::configure_schedule(&mut schedule);

        Self {
            config,
            world,
            schedule,
        }
    }

    pub fn config(&self) -> &SimConfig {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn step(&mut self) {
        {
            let mut time = self
                .world
                .get_resource_mut::<TimeState>()
                .expect("time resource should exist");
            time.tick += time.fixed_time_step as u64;
        }
        self.schedule.run(&mut self.world);
    }

    pub fn run_for_ticks(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.step();
        }
    }

    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot::capture(&self.world)
    }
}

use bevy::{prelude::*, time::FixedTimestep};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

pub fn system_set() -> SystemSet {
    SystemSet::new().with_run_criteria(FixedTimeStep::step(TIME_STEP as f64))
}

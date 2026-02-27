use std::time::Duration;
use evian::{
    motion::{Basic, Seeking},
    control::loops::{Pid, AngularPid},
    prelude::*,
};
use vexide::prelude::*;
use crate::Robot;

pub async fn run(robot: &mut Robot, basic: &mut Basic<Pid, AngularPid>, seeking: &mut Seeking<Pid, Pid>) {
    // Set starting position
    robot.drivetrain.tracking.set_position((1.0, 1.0));
    robot.drivetrain.tracking.set_heading(0.0_f64.deg());

    let start_pos = robot.drivetrain.tracking.position();
    println!("START POS: ({:.2}, {:.2})", start_pos.x, start_pos.y);

    // Reset distance using the robot's method
    robot.weighted_dist_reset();

    // Get position after reset
    let after_reset = robot.drivetrain.tracking.position();
    println!("AFTER RESET: ({:.2}, {:.2})", after_reset.x, after_reset.y);
}

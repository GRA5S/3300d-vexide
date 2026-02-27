use std::time::Duration;
use evian::{
    motion::{Basic, Seeking},
    control::loops::{Pid, AngularPid},
    prelude::*,
};
use vexide::prelude::*;
use crate::Robot;

pub async fn run(robot: &mut Robot, basic: &mut Basic<Pid, AngularPid>, seeking: &mut Seeking<Pid, Pid>) {
    let dt = &mut robot.drivetrain;

// Evian PID
// Starting point: (46.58 in, 0.39 in)
// Point 2
dt.tracking.set_heading(180.0.deg());
// Point 2
basic
    .drive_distance(dt, 46.50)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 3
basic
    .turn_to_heading(dt, 92.27.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 19.55)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 4
basic
    .turn_to_heading(dt, 91.29.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -41.40)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 5
basic
    .turn_to_heading(dt, 90.56.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 15.81)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 6
basic
    .turn_to_heading(dt, 180.29.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 92.54)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 7
basic
    .turn_to_heading(dt, 91.02.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 26.05)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 8
basic
    .turn_to_heading(dt, 89.35.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -40.92)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 9
basic
    .turn_to_heading(dt, 89.08.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 28.83)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 10
basic
    .turn_to_heading(dt, 129.56.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -55.49)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
}

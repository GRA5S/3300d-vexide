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
// Starting point: (43.32 in, 0.08 in)
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_high();
_ = robot.midgoal.set_low(); //hoard
// Point 2
dt.tracking.set_heading(270.00.deg());
// Point 2
basic
    .drive_distance(dt, 13.49)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 3
basic
    .turn_to_heading(dt, 340.71.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 9.85)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 4
basic
    .turn_to_heading(dt, 340.35.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 6.91)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = robot.matchload.toggle(); //matchload
// Point 5
basic
    .turn_to_heading(dt, 337.01.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 16.67)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 6
basic
    .turn_to_heading(dt, 61.98.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 32.66)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 7
basic
    .turn_to_heading(dt, 91.33.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 20.00)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 8
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
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_low();
_ = robot.midgoal.set_low(); //score
sleep(Duration::from_millis(4000)).await;
}

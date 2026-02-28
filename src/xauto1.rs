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

    // evian seeking shit
    dt.tracking.set_position((60.53, -14.96));
dt.tracking.set_heading(180.0.deg());

// Starting point: (60.53 in, -14.96 in)
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_high();
_ = robot.midgoal.set_low(); //hoard
// seeking
//   .move_to_point(dt, (60.53, -14.96))
//   .with_timeout(Duration::from_millis(2000))
//   .with_linear_output_limit(1.0)
//   .await;

// seeking
//   .move_to_point(dt, (13.64, -23.49))
//   .with_timeout(Duration::from_millis(2000))
//   .with_linear_output_limit(0.6)
//   .await;
// seeking
//   .move_to_point(dt, (53.05, -45.47))
//   .with_timeout(Duration::from_millis(2000))
//   .with_linear_output_limit(1.0)
//   .await;
// _ = robot.matchload.set_low();

// basic
//     .turn_to_heading(dt, 0.0.deg()).await;
// basic
//     .drive_distance(dt, -45.12)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.hood.set_low();
// _ = robot.midgoal.set_low(); //score
// _ = robot.matchload.set_high();
// sleep(Duration::from_millis(1000)).await;
// _ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.hood.set_high();
// _ = robot.midgoal.set_low(); //hoard
// basic
//     .drive_distance(dt, 45.12)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(0.7)
//     .await;
// basic
//     .drive_distance(dt, -1.0)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1)
//     .await;
// basic
//     .drive_distance(dt, 1.0)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1)
//     .await;
// basic
//     .drive_distance(dt, -45.12)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(0.7)
//     .await;
// _ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.hood.set_low();
// _ = robot.midgoal.set_low(); //score
// sleep(Duration::from_millis(5000)).await;





seeking
  .move_to_point(dt, (60.53, -14.96))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0)
  .await;

seeking
  .move_to_point(dt, (40.93, -25.01))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0)
  .await;
seeking
  .move_to_point(dt, (49.05, -39.07))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0)
  .await;
// _ = robot.matchload.set_low();

basic
    .turn_to_heading(dt, 0.0.deg()).await;
// basic
//     .drive_distance(dt, -45.12)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = robot.hood.set_low();
// _ = robot.midgoal.set_low(); //score
// _ = robot.matchload.set_high();
// sleep(Duration::from_millis(1000)).await;
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_high();
_ = robot.midgoal.set_low(); //hoard
// basic
//     .drive_distance(dt, -5.12)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(0.7)
//     .await;
_ = robot.matchload.set_high(); 
sleep(Duration::from_millis(200)).await;


basic
    .drive_distance(dt, 45.12)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -1.0)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 1.0)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// basic.turn_to_heading(dt, 5.0.deg()).await;

basic
    .drive_distance(dt, -45.12)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(0.7)
    .await;
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_low();
_ = robot.midgoal.set_low(); //score
sleep(Duration::from_millis(5000)).await;
}

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
dt.tracking.set_position((43.32, -0.08));
// Starting point: (43.32 in, -0.08 in)
basic
    .turn_to_point(dt, (43.32, -0.08))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (43.32, -0.08))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (29.84, -0.08))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (29.84, -0.08))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (26.58, -9.38))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (26.58, -9.38))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (23.79, -15.89))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (23.79, -15.89))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (17.75, -31.23))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (17.75, -31.23))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (51.69, -46.58))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (51.69, -46.58))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (66.57, -46.11))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (66.57, -46.11))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;
basic
    .turn_to_point(dt, (25.19, -47.04))
    .with_timeout(Duration::from_millis(2000))
    .await;
seeking
  .move_to_point(dt, (25.19, -47.04))
  .with_timeout(Duration::from_millis(2000))
  .with_linear_output_limit(1.0).await;

    // TODO: Add auto6 routine here
}

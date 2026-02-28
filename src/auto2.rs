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
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_high();
_ = self.midgoal.set_low(); //hoard
// Point 2
dt.tracking.set_heading(180.00.deg());
// Point 2
basic
    .drive_distance(dt, 46.97)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.matchload.toggle(); //matchload
// Point 3
basic
    .turn_to_heading(dt, 88.73.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 20.93)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 4
basic
    .drive_distance(dt, -42.33)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_low();
_ = self.midgoal.set_low(); //score
sleep(Duration::from_millis(2000)).await;
// Point 5
basic
    .turn_to_heading(dt, 90.00.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 10.23)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.intake1.set_voltage(0.0);
_ = self.intake2.set_voltage(0.0); //stop intake
// Point 6
basic
    .turn_to_heading(dt, 125.31.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 13.68)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 7
basic
    .turn_to_heading(dt, 90.00.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -37.67)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;

}

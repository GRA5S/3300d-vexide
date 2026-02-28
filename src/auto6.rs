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

Configuration
Robot Visualization
Visible
Field
Grid Density
Conversion Factor

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
    .with_linear_output_limit(NaN)
    .await;
// Point 4
basic
    .turn_to_heading(dt, 88.74.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
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
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_high();
_ = self.midgoal.set_low(); //hoard
_ = self.matchload.toggle(); //matchload
// Point 6
basic
    .turn_to_heading(dt, 0.29.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 91.14)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.matchload.toggle(); //matchload
// Point 7
basic
    .turn_to_heading(dt, 86.58.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 31.21)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(NaN)
    .await;
// Point 8
basic
    .turn_to_heading(dt, 90.64.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -41.39)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_low();
_ = self.midgoal.set_low(); //score
sleep(Duration::from_millis(2000)).await;
// Point 9
basic
    .turn_to_heading(dt, 91.06.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 25.12)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 10
basic
    .turn_to_heading(dt, 144.87.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 15.35)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(NaN)
    .await;
// Point 11
basic
    .turn_to_heading(dt, 180.00.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 35.34)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(NaN)
    .await;


}

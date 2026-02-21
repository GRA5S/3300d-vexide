use std::time::{
    Duration,
    Instant,
};
use evian::{
    motion::{Basic, Seeking},
    control::loops::{AngularPid, Pid},
    drivetrain::model::Differential,
    prelude::*,
    tracking::{
        shared_motors,
        wheeled::{TrackingWheel, WheeledTracking},
    },
};
use vexide::prelude::*;
use vexide::adi::digital::LogicLevel;

const LEFT_DISTANCE_FROM_CENTER: f64 = 0.0;
const RIGHT_DISTANCE_FROM_CENTER: f64 = 0.0;
const FRONT_DISTANCE_FROM_CENTER: f64 = 0.0;

struct Robot {
    controller: Controller,
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    intake1: Motor,
    intake2: Motor,
    wing: AdiDigitalOut,
    matchload: AdiDigitalOut,
    hood: AdiDigitalOut,
    midgoal: AdiDigitalOut,
    intake2_overcurrent_disabled: bool,
    intake2_overcurrent_time: Option<Instant>,
}
pub const TRACK_WIDTH: f64 = 10.0;
pub const WHEEL_DIAMETER: f64 = 3.25;
pub const GEARING: f64 = 48.0/72.0;
impl Robot {
    const LINEAR_PID: Pid = Pid::new(7.1, 0.0, 0.6, None);
    const LATERAL_PID: Pid = Pid::new(6.5, 0.0, 1.29, Some(2.0));   
    const ANGULAR_PID: AngularPid = AngularPid::new(6.5, 0.0, 0.59, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(9.0)
        .velocity(4.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(4.0))
        .velocity(4.0)
        .duration(Duration::from_millis(150));
}

impl Robot {
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        let dt = &mut self.drivetrain;
        let mut seeking = Seeking {
            linear_controller: Self::LINEAR_PID,
            lateral_controller: Self::LATERAL_PID,
            tolerances: Self::LINEAR_TOLERANCES,
            timeout: Some(Duration::from_secs(10)),
        };
        let mut basic = Basic {
            linear_controller: Self::LINEAR_PID,
            angular_controller: Self::ANGULAR_PID,
            linear_tolerances: Self::LINEAR_TOLERANCES,
            angular_tolerances: Self::ANGULAR_TOLERANCES,
            timeout: Some(Duration::from_secs(10)),
        };
        // _ = dt.model.drive_arcade(-3.0, 0.0);
        // sleep(Duration::from_millis(500)).await;
        // _ = dt.model.drive_arcade(0.0, 0.0);
        // basic.turn_to_heading(dt, 0.0.deg()).await;
        // basic.turn_to_heading(dt, 90.0.deg()).await;
        // basic.drive_distance_at_heading(dt, 24.0, 90.0.deg()).await;
        // basic.drive_distance_at_heading(dt, -12.0, 90.0.deg()).await;
        // basic.drive_distance_at_heading(dt, -12.0, 90.0.deg()).await;


        // dt.tracking.set_heading(270.0.deg());
        // // Path

        // basic.drive_distance(dt, 24.448 as f64).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 0.7 as f64).await;
        // basic.turn_to_heading(dt, (234.686 as f64).deg()).await;
        // basic.drive_distance(dt, 31.883 as f64).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 0.7 as f64).await;
        // basic.turn_to_heading(dt, (270 as f64).deg()).await;
        // basic.drive_distance(dt, -(49.916 as f64)).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 0.7 as f64).await;
        // basic.turn_to_heading(dt, (67.824 as f64).deg()).await;
        // basic.drive_distance(dt, -(60.847 as f64)).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 0.7 as f64).await;
        // basic.turn_to_heading(dt, (64.272 as f64).deg()).await;

        // // Path

        // basic.drive_distance(dt, 31.9 as f64).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 1 as f64).await;
        // basic.turn_to_heading(dt, (90 as f64).deg()).await;
        // basic.drive_distance(dt, -(31.9 as f64)).with_linear_output_limit(Motor::V5_MAX_VOLTAGE * 1 as f64).await;
        // basic.turn_to_heading(dt, (90.302 as f64).deg()).await;
// // Evian PID
// // Starting point: (59.37 in, -14.91 in)
// // Point 2
// dt.tracking.set_heading(270.00.deg());
// // Point 2
// basic
//     .drive_distance(dt, 22.23)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.hood.set_high();
// _ = self.midgoal.set_low();
// // Point 3
// basic
//     .turn_to_heading(dt, 250.35.deg())
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// basic
//     .drive_distance(dt, 30.05)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(0.33)
//     .await;
// _ = self.intake1.set_voltage(0.0);
// _ = self.intake2.set_voltage(0.0);
// // Point 4
// basic
//     .turn_to_heading(dt, 118.81.deg())
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// basic
//     .drive_distance(dt, 46.13)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// // Point 5
// basic
//     .turn_to_heading(dt, 90.00.deg())
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// basic
//     .drive_distance(dt, -66.69)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = self.matchload.toggle();
// _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.hood.set_low();
// _ = self.midgoal.set_low();
// // Point 6
// basic
//     .turn_to_heading(dt, 90.00.deg())
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// basic
//     .drive_distance(dt, 46.48)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.hood.set_high();
// _ = self.midgoal.set_low();
// // Point 7
// basic
//     .turn_to_heading(dt, 270.00.deg())
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// basic
//     .drive_distance(dt, 46.48)
//     .with_timeout(Duration::from_millis(2000))
//     .with_linear_output_limit(1.0)
//     .await;
// _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
// _ = self.hood.set_low();
// _ = self.midgoal.set_low();
// Evian PID
// Starting point: (59.37 in, -14.91 in)
// Point 2
dt.tracking.set_heading(269.37.deg());
// Point 2
basic
    .drive_distance(dt, 22.99)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(0.99)
    .await;
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_high();
_ = self.midgoal.set_low();
// Point 3
basic
    .turn_to_heading(dt, 239.97.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 31.81/2.0)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(0.33)
    .await;
_ = self.matchload.toggle();
basic
    .drive_distance(dt, 31.81/2.0)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(0.33)
    .await;
_ = self.matchload.toggle();

_ = self.intake1.set_voltage(0.0);
_ = self.intake2.set_voltage(0.0);
// Point 4
basic
    .turn_to_heading(dt, 98.97.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 38.88)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 5
basic
    .turn_to_heading(dt, 180.00.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 10.11)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 6
basic
    .turn_to_heading(dt, 90.00.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -50.53)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_low();
_ = self.midgoal.set_low();
sleep(Duration::from_millis(4000)).await;




// Evian PID
// Starting point: (9.85 in, -45.47 in)
// Point 2
dt.tracking.set_heading(91.19.deg());
// Point 2
_ = self.matchload.set_high();
basic
    .drive_distance(dt, 89.77)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(0.67)
    .await;
_ = self.hood.set_high();
_ = self.midgoal.set_low();
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);

sleep(Duration::from_millis(4000)).await;
basic
    .turn_to_heading(dt, 89.84.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;     
basic
    .drive_distance(dt, -91.96)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = self.hood.set_low();
_ = self.midgoal.set_low();
sleep(Duration::from_millis(4000)).await;















        
    }

    async fn driver(&mut self) {
        
        loop {
            let state = self.controller.state().unwrap_or_default();

            if state.button_left.is_pressed() && state.button_right.is_pressed() {
                self.autonomous().await;
                continue;
            }

            _ = self
                .drivetrain
                .model
                .drive_arcade(state.right_stick.y(), state.left_stick.x());


            if state.button_l1.is_pressed() {
                _ = self.intake1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake2.set_voltage(-Motor::V5_MAX_VOLTAGE);
            } else if state.button_r2.is_pressed() {
                _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.hood.set_low();
                _ = self.midgoal.set_low();
            } else if state.button_r1.is_now_pressed() {
               self.intake2_overcurrent_disabled = false;
               self.intake2_overcurrent_time = None;
            } else if state.button_r1.is_pressed() {
              let current = self.intake2.current().unwrap_or(0.0);
              if current > 2.0 {
                  if self.intake2_overcurrent_time.is_none() {
                      self.intake2_overcurrent_time = Some(Instant::now());
                  } else if let Some(start) = self.intake2_overcurrent_time {
                      if start.elapsed() >= Duration::from_secs(1) {
                          _ = self.intake2.set_voltage(0.0);
                          self.intake2_overcurrent_disabled = true;
                      }
                  }
              } else {
                  self.intake2_overcurrent_time = None;
                  if !self.intake2_overcurrent_disabled {
                       _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
                       _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
                       _ = self.hood.set_high();
                       _ = self.midgoal.set_low();
                  }
              }
             } else if state.button_l2.is_pressed() {
                _ = self.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.hood.set_high();
                _ = self.midgoal.set_high();
            } else {
                _ = self.intake1.set_voltage(0.0);
                _ = self.intake2.set_voltage(0.0);
            }


            if state.button_down.is_now_pressed() {
                _ = self.matchload.toggle();
            }

            if state.button_up.is_now_pressed() {
                _ = self.wing.toggle();
            }

            // let current = self.intake2.current().unwrap_or(0.0);
            // _ = self.controller.set_text(&format!("I2: {:.2}A", current), 2, 1).await;

            sleep(Motor::WRITE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {

    let controller = peripherals.primary_controller;

    let mut imu = InertialSensor::new(peripherals.port_17);
    imu.calibrate().await.unwrap();

    let distance_left = DistanceSensor::new(peripherals.port_9);
    let distance_right = DistanceSensor::new(peripherals.port_1);
    let distance_front = DistanceSensor::new(peripherals.port_8);

    let left_motors = shared_motors![
        Motor::new(peripherals.port_14, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_16, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_13, Gearset::Blue, Direction::Reverse),
    ];
    let right_motors = shared_motors![
        Motor::new(peripherals.port_11, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_15, Gearset::Blue, Direction::Forward),
    ];

    let robot = Robot {
        controller,
        drivetrain: Drivetrain::new(
            Differential::from_shared(left_motors.clone(), right_motors.clone()),
            WheeledTracking::forward_only(
                (0.0, 0.0),
                90.0.deg(),
                [
                    TrackingWheel::new(left_motors, WHEEL_DIAMETER, TRACK_WIDTH/2.0, Some(GEARING)),
                    TrackingWheel::new(right_motors, WHEEL_DIAMETER, TRACK_WIDTH/2.0, Some(GEARING)),
                ],
                Some(imu),
            ),
        ),
        intake1: Motor::new(peripherals.port_5, Gearset::Blue, Direction::Reverse),
        intake2: Motor::new(peripherals.port_6, Gearset::Blue, Direction::Reverse),
        matchload: AdiDigitalOut::with_initial_level(peripherals.adi_b, LogicLevel::Low),
        wing: AdiDigitalOut::with_initial_level(peripherals.adi_d, LogicLevel::Low),
        hood: AdiDigitalOut::with_initial_level(peripherals.adi_e, LogicLevel::High),
        midgoal: AdiDigitalOut::with_initial_level(peripherals.adi_f, LogicLevel::Low),
        intake2_overcurrent_disabled: false,
        intake2_overcurrent_time: None,
    };

    robot.compete().await;
}
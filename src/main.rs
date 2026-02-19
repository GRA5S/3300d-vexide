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
// pros::Distance distance_left(9);
// pros::Distance distance_right(1);
// pros::Distance distance_front(9);

// float left_distance_from_center = 0;
// float right_distance_from_center = 0;
// float front_distance_from_center = 0;
// float dist_to_center = 0;

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
    const LINEAR_PID: Pid = Pid::new(6.5, 0.0, 1.29, None);
    const LATERAL_PID: Pid = Pid::new(0.09, 0.001, 0.004, Some(2.0));   
    const ANGULAR_PID: AngularPid = AngularPid::new(6.5, 0.0, 0.59, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(4.0)
        .velocity(0.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(4.0))
        .velocity(4.0)
        .duration(Duration::from_millis(150));
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        let dt = &mut self.drivetrain;
        let mut seeking = Seeking {
            linear_controller: Pid::new(0.0, 0.0, 0.0, None),
            lateral_controller: Pid::new(0.0, 0.0, 0.0, None),
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
        basic.drive_distance_at_heading(dt, 24.0, 90.0.deg()).await;
        basic.drive_distance_at_heading(dt, -12.0, 90.0.deg()).await;
        basic.drive_distance_at_heading(dt, -12.0, 90.0.deg()).await;


        
    }

    async fn driver(&mut self) {
        
        loop {
            let state = self.controller.state().unwrap_or_default();

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

            let current = self.intake2.current().unwrap_or(0.0);
            _ = self.controller.set_text(&format!("I2: {:.2}A", current), 2, 1).await;

            sleep(Motor::WRITE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {

    let controller = peripherals.primary_controller;

    let mut imu = InertialSensor::new(peripherals.port_17);
    imu.calibrate().await.unwrap();

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
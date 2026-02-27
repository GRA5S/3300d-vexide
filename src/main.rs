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

mod auto1;
mod auto2;
mod auto3;
mod auto4;
mod auto5;
mod auto6;
mod auto7;
mod auto8;
mod auto_selector;

static mut SELECTED_AUTO: u8 = 2;
const LEFT_DISTANCE_FROM_CENTER: f64 = 5.163;
const RIGHT_DISTANCE_FROM_CENTER: f64 = 5.163;
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
    front_sensor: DistanceSensor,
    left_sensor: DistanceSensor,
    right_sensor: DistanceSensor,
}
pub const TRACK_WIDTH: f64 = 10.0;
pub const WHEEL_DIAMETER: f64 = 3.25;
pub const GEARING: f64 = 48.0/72.0;
impl Robot {
    const LINEAR_PID: Pid = Pid::new(7.1, 0.0, 0.6, None);
    // const LINEAR_PID: Pid = Pid::new(0.0, 0.0, 0.0, None);
    const LATERAL_PID: Pid = Pid::new(0.09, 0.01, 0.01, None);     
    const ANGULAR_PID: AngularPid = AngularPid::new(6.5, 0.0, 0.59, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(9.0)
        .velocity(4.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(4.0))
        .velocity(4.0)
        .duration(Duration::from_millis(150));

    async fn driveto(
        &mut self,
        target_x: f64,
        target_y: f64,
        timeout: u64,
        speed: f64,
        basic: &mut Basic<Pid, AngularPid>,
    ) {
        let dt = &mut self.drivetrain;
        let pos = dt.tracking.position();
        let current_x = pos.x;
        let current_y = pos.y;

        let dx = target_x - current_x;
        let dy = target_y - current_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();

        basic
            .turn_to_heading(dt, angle.deg())
            .with_timeout(Duration::from_millis(timeout))
            .await;
        basic
            .drive_distance(dt, distance)
            .with_timeout(Duration::from_millis(timeout))
            .with_linear_output_limit(speed)
            .await;
    }

    async fn reversedriveto(
        &mut self,
        target_x: f64,
        target_y: f64,
        timeout: u64,
        speed: f64,
        basic: &mut Basic<Pid, AngularPid>,
    ) {
        let dt = &mut self.drivetrain;
        let pos = dt.tracking.position();
        let current_x = pos.x;
        let current_y = pos.y;

        let dx = target_x - current_x;
        let dy = target_y - current_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();

        basic
            .turn_to_heading(dt, (angle + 180.0).deg())
            .with_timeout(Duration::from_millis(timeout))
            .await;
        basic
            .drive_distance(dt, -distance)
            .with_timeout(Duration::from_millis(timeout))
            .with_linear_output_limit(speed)
            .await;
    }

    async fn turnto(
        &mut self,
        heading: f64,
        timeout: u64,
        speed: f64,
        basic: &mut Basic<Pid, AngularPid>,
    ) {
        let dt = &mut self.drivetrain;
        basic
            .turn_to_heading(dt, heading.deg())
            .with_timeout(Duration::from_millis(timeout))
            .with_linear_output_limit(speed)
            .await;
    }

    fn weighted_distance(&self, sensor_index: u8) -> Option<f64> {
        let mut distances = [0.0; 5];
        let mut confidences = [0.0; 5];
        let mut total_confidence = 0.0;

        for i in 0..5 {
            match sensor_index {
                1 => {
                    if let Ok(obj) = self.left_sensor.object() {
                        if let Some(obj) = obj {
                            distances[i] = obj.distance as f64 * 0.0394; // mm to inches
                            confidences[i] = obj.confidence as f64 / 63.0;
                        }
                    }
                }
                2 => {
                    if let Ok(obj) = self.right_sensor.object() {
                        if let Some(obj) = obj {
                            distances[i] = obj.distance as f64 * 0.0394; // mm to inches
                            confidences[i] = obj.confidence as f64 / 63.0;
                        }
                    }
                }
                _ => return None,
            }
            total_confidence += confidences[i];
        }

        if total_confidence == 0.0 {
            return None;
        }

        let mut weighted_distance = 0.0;
        for i in 0..5 {
            let weighted_confidence = confidences[i] / total_confidence;
            weighted_distance += distances[i] * weighted_confidence;
        }

        Some(weighted_distance)
    }

    fn simple_dist_reset(&mut self) {
        let pose = self.drivetrain.tracking.position();

        if pose.x > 0.0 && pose.y > 0.0 {
            if let Ok(obj) = self.left_sensor.object() {
                if let Some(obj) = obj {
                    let dist_to_center = obj.distance as f64 * 0.0394 + LEFT_DISTANCE_FROM_CENTER;
                    self.drivetrain.tracking.set_position((pose.x, 72.0 - dist_to_center));
                }
            }
        } else if pose.x < 0.0 && pose.y > 0.0 {
            if let Ok(obj) = self.right_sensor.object() {
                if let Some(obj) = obj {
                    let dist_to_center = obj.distance as f64 * 0.0394 + RIGHT_DISTANCE_FROM_CENTER;
                    self.drivetrain.tracking.set_position((pose.x, 72.0 - dist_to_center));
                }
            }
        } else if pose.x < 0.0 && pose.y < 0.0 {
            if let Ok(obj) = self.left_sensor.object() {
                if let Some(obj) = obj {
                    let dist_to_center = obj.distance as f64 * 0.0394 + LEFT_DISTANCE_FROM_CENTER;
                    self.drivetrain.tracking.set_position((pose.x, -72.0 + dist_to_center));
                }
            }
        } else if pose.x > 0.0 && pose.y < 0.0 {
            if let Ok(obj) = self.right_sensor.object() {
                if let Some(obj) = obj {
                    let dist_to_center = obj.distance as f64 * 0.0394 + RIGHT_DISTANCE_FROM_CENTER;
                    self.drivetrain.tracking.set_position((pose.x, -72.0 + dist_to_center));
                }
            }
        }
    }

    fn weighted_dist_reset(&mut self) {
        let pose = self.drivetrain.tracking.position();

        if pose.x > 0.0 && pose.y > 0.0 {
            if let Some(weighted_dist) = self.weighted_distance(1) {
                let dist_to_center = weighted_dist + LEFT_DISTANCE_FROM_CENTER;
                self.drivetrain.tracking.set_position((pose.x, 72.0 - dist_to_center));
            }
        } else if pose.x < 0.0 && pose.y > 0.0 {
            if let Some(weighted_dist) = self.weighted_distance(2) {
                let dist_to_center = weighted_dist + RIGHT_DISTANCE_FROM_CENTER;
                self.drivetrain.tracking.set_position((pose.x, 72.0 - dist_to_center));
            }
        } else if pose.x < 0.0 && pose.y < 0.0 {
            if let Some(weighted_dist) = self.weighted_distance(1) {
                let dist_to_center = weighted_dist + LEFT_DISTANCE_FROM_CENTER;
                self.drivetrain.tracking.set_position((pose.x, -72.0 + dist_to_center));
            }
        } else if pose.x > 0.0 && pose.y < 0.0 {
            if let Some(weighted_dist) = self.weighted_distance(2) {
                let dist_to_center = weighted_dist + RIGHT_DISTANCE_FROM_CENTER;
                self.drivetrain.tracking.set_position((pose.x, -72.0 + dist_to_center));
            }
        }
    }
}

impl Compete for Robot {
     async fn autonomous(&mut self) {
         let mut basic = Basic {
             linear_controller: Self::LINEAR_PID,
             angular_controller: Self::ANGULAR_PID,
             linear_tolerances: Self::LINEAR_TOLERANCES,
             angular_tolerances: Self::ANGULAR_TOLERANCES,
             timeout: Some(Duration::from_secs(10)),
         };
         let mut seeking = Seeking {
             linear_controller: Self::LINEAR_PID,
             lateral_controller: Self::LATERAL_PID,
             tolerances: Self::LINEAR_TOLERANCES,
             timeout: Some(Duration::from_secs(10)),
         };

         match unsafe { SELECTED_AUTO } {
             1 => auto1::run(self, &mut basic, &mut seeking).await,
             2 => auto2::run(self, &mut basic, &mut seeking).await,
             3 => auto3::run(self, &mut basic, &mut seeking).await,
             4 => auto4::run(self, &mut basic, &mut seeking).await,
             5 => auto5::run(self, &mut basic, &mut seeking).await,
             6 => auto6::run(self, &mut basic, &mut seeking).await,
             7 => auto7::run(self, &mut basic, &mut seeking).await,
             8 => auto8::run(self, &mut basic, &mut seeking).await,
             _ => {}
         }
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

    let mut display = peripherals.display;
    let controller = peripherals.primary_controller;

    // Show auto selector
    unsafe {
        SELECTED_AUTO = auto_selector::select_auto(&mut display, &controller).await;
    }

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
        front_sensor: DistanceSensor::new(peripherals.port_2),
        left_sensor: DistanceSensor::new(peripherals.port_1),
        right_sensor: DistanceSensor::new(peripherals.port_3),
    };

    robot.compete().await;
}
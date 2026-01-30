use std::time::{
    Duration,
    // Instant
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

struct Robot {
    controller: Controller,
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    intake: (Motor, Motor, Motor),
    wing: AdiDigitalOut,
    matchload: AdiDigitalOut,
    triple_state: (AdiDigitalOut, AdiDigitalOut),
}
pub const TRACK_WIDTH: f64 = 10.0;
pub const WHEEL_DIAMETER: f64 = 3.25;
pub const GEARING: f64 = 48.0/72.0;
impl Robot {
    const LINEAR_PID: Pid = Pid::new(1.0, 0.0, 0.125, None);
    const LATERAL_PID: Pid = Pid::new(0.09, 0.001, 0.004, Some(2.0));   
    const ANGULAR_PID: AngularPid = AngularPid::new(10.0, 0.0, 1.29, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(4.0)
        .velocity(0.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(0.9))
        .velocity(0.09)
        .duration(Duration::from_millis(150));
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

        // Turn to 0 degrees heading.
        basic.turn_to_heading(dt, 0.0.deg()).await;
        basic.turn_to_heading(dt, 90.0.deg()).await;
        // basic.turn_to_heading(dt, 0.0.deg()).await;
        // basic.turn_to_heading(dt, 90.0.deg()).await;
        // basic.turn_to_heading(dt, 0.0.deg()).await;
        // basic.drive_distance(dt, 24.0).await;
        // basic.turn_to_heading(dt, 180.0.deg()).await;
        // basic.drive_distance(dt, 24.0).await;
        
        // // Move to point (24, 24) on the field.
        // seeking.move_to_point(dt, (24.0, 24.0)).await;
        

    }

    async fn driver(&mut self) {
        
        loop {
            let state = self.controller.state().unwrap_or_default();

            _ = self
                .drivetrain
                .model
                .drive_arcade(state.right_stick.y(), state.left_stick.x());


            if state.button_r1.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
                // _ = self.triple_state.0.set_high();
                // _ = self.triple_state.1.set_high();
            } else if state.button_l2.is_pressed() {
                _ = self.intake.0.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
            } else if state.button_l1.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.triple_state.0.set_low();
                _ = self.triple_state.1.set_low();
             } else if state.button_r2.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.triple_state.0.set_high();
                _ = self.triple_state.1.set_low();
            } else {
                _ = self.intake.0.set_voltage(0.0);
                _ = self.intake.1.set_voltage(0.0);
                _ = self.intake.2.set_voltage(0.0);
            }


            if state.button_down.is_now_pressed() {
                _ = self.matchload.toggle();
            }

            if state.button_up.is_now_pressed() {
                _ = self.wing.toggle();
            }

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
        intake: (
            Motor::new(peripherals.port_6, Gearset::Blue, Direction::Reverse),
            Motor::new(peripherals.port_7, Gearset::Blue, Direction::Reverse),
            Motor::new(peripherals.port_5, Gearset::Blue, Direction::Reverse),
        ),
        matchload: AdiDigitalOut::with_initial_level(peripherals.adi_b, LogicLevel::Low),
        wing: AdiDigitalOut::with_initial_level(peripherals.adi_d, LogicLevel::Low),
        triple_state: (
            AdiDigitalOut::with_initial_level(peripherals.adi_e, LogicLevel::Low),
            AdiDigitalOut::with_initial_level(peripherals.adi_f, LogicLevel::Low),
        ),
    };

    robot.compete().await;
}
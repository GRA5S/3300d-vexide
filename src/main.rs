#![no_main]
#![no_std]

extern crate alloc;

// mod autonomous;
mod mappings;
// mod pose;
// mod subsystems;

use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, time::Duration};
use alloc::vec;
use vexide_motorgroup::*;

use vexide::prelude::*;
// use autonomous::{
//     command::Command,
//     execute::{
//         ANGULAR_CONTROLLER, LINEAR_CONTROLLER,
//         TOLERANCES, /*, ANGULAR_CONTROLLER, LINEAR_CONTROLLER, TOLERANCES*/
//         execute_command,
//     },
// };
use evian::{drivetrain::Drivetrain, math::Vec2, motion::Basic, prelude::*};
use mappings::{ControllerMappings, DriveMode};
// use subsystems::{
//     drivetrain::differential_drive,
//     intake::{Intake, IntakeCommand},
//     lady_brown::LadyBrown,
// };
use vexide::{
    adi::digital::LogicLevel, prelude::*, startup::banner::themes::THEME_MURICA,
};



const TRACK_WIDTH: f64 = 10;
const GEARING: f64 = 48.0 / 72.0;
const WHEEL_DIAMETER: f64 = 3.25;


struct Robot {
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    intake: (Motor, Motor, Motor),
    wing: AdiDigitalOut,
    matchload: AdiDigitalOut,
    triple_state: (AdiDigitalOut, AdiDigitalOut),

    controller: Controller,
}



impl Compete for Robot {
    async fn autonomous(&mut self) {
        // todo: make auto ;;
                // self.triple_state.1.set_high()?;
                // self.triple_state.1.set_high()?;
    }

    async fn driver(&mut self) {

        // let mut basic = Basic {
        //     linear_controller: LINEAR_CONTROLLER,
        //     angular_controller: ANGULAR_CONTROLLER,
        //     linear_tolerances: TOLERANCES,
        //     angular_tolerances: TOLERANCES,
        //     timeout: Some(Duration::from_millis(2000)),
        // };

        loop {
            let delay = Instant::now() + Controller::UPDATE_INTERVAL;

            let state = self.controller.state().unwrap_or_default();

            let mappings = ControllerMappings {
                drive_mode: DriveMode::Arcade {
                    power: state.right_stick,
                    turn: state.left_stick,
                },

                hoard: state.button_r1,
                outake: state.button_l2,
                mid: state.button_l1,
                long: state.button_r2,

                
                wing: state.button_up,
                matchload: state.button_down,
            };

            let power = differential_drive(&mappings.drive_mode);
            _ = self.drivetrain.motors.set_voltages(power);

            // neaten with refactor
            if mappings.hoard.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
                // self.triple_state.0.set_high()?;
                // self.triple_state.1.set_high()?;
            } else if mappings.outtake.is_pressed() {
                _ = self.intake.0.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
            } else if mappings.mid.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
                self.triple_state.0.set_low()?;
                self.triple_state.1.set_low()?;
            } else if mappings.long.is_pressed() {
                _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
                _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
                self.triple_state.0.set_high()?;
                self.triple_state.1.set_low()?;
            } else {
                _ = self.intake.0.set_voltage(0);
                _ = self.intake.1.set_voltage(0);
                _ = self.intake.2.set_voltage(0);
            }


            if mappings.matchload.is_now_pressed() {
                _ = self.matchload.toggle();
            }

            if mappings.wing.is_now_pressed() {
                _ = self.wing.toggle();
            }

            sleep_until(delay).await;
        }
    }
}
#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut imu = InertialSensor::new(peripherals.port_8);

    match imu.calibrate().await {
    }
    let left_motors = [
        Motor::new(peripherals.port_14, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_16, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_13, Gearset::Blue, Direction::Forward),
    ];
    
    let right_motors = [
        Motor::new(peripherals.port_11, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_15, Gearset::Blue, Direction::Forward),
    ];
    let robot = Robot {
        drivetrain: Drivetrain::new(
            Differential::from_shared(left_motors.clone(), right_motors.clone()),
            WheeledTracking::new(
                Vec2::default(),
                Angle::default(),
                [
                    TrackingWheel::new(
                        left_motors.clone(),
                        WHEEL_DIAMETER,
                        TRACK_WIDTH,
                        Some(GEARING),
                    ),
                    TrackingWheel::new(
                        right_motors.clone(),
                        WHEEL_DIAMETER,
                        TRACK_WIDTH,
                        Some(GEARING),
                    ),
                ],
                [
                    // Fake tracking wheel because i can't figure out how to work without sideways
                    TrackingWheel::new(
                        RotationSensor::new(peripherals.port_20, Direction::Forward),
                        WHEEL_DIAMETER,
                        TRACK_WIDTH,
                        Some(GEARING),
                    ),
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
            AdiDigitalOut::with_initial_level(peripherals.adi_f, LogicLevel::Low5),
            AdiDigitalOut::with_initial_level(peripherals.adi_g, LogicLevel::Low),
        ),
        controller: peripherals.primary_controller,
    };
}
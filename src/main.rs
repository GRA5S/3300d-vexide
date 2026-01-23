
// extern crate alloc;

// // mod autonomous;
// mod mappings;
// // mod pose;
// // mod subsystems;

// use alloc::{
//     rc::Rc,
//     // vec::Vec
// };
// use core::{
//     cell::RefCell,
//     // time::Duration
// };
// // use alloc::vec;
// use vexide::prelude::*;
// // use autonomous::{
// //     command::Command,
// //     execute::{
// //         ANGULAR_CONTROLLER, LINEAR_CONTROLLER,
// //         TOLERANCES, /*, ANGULAR_CONTROLLER, LINEAR_CONTROLLER, TOLERANCES*/
// //         execute_command,
// //     },
// // };
// use evian::{
//     drivetrain::Drivetrain,
//     math::{Angle, Vec2},
//     tracking::wheeled::{TrackingWheel, WheeledTracking},
//     drivetrain::model::{Arcade, Differential},

//     // prelude::*
// };
// use mappings::{ControllerMappings, DriveMode};
// // use subsystems::{
//     // drivetrain::differential_drive,
//     // intake::{Intake, IntakeCommand},
//     // lady_brown::LadyBrown,
// // };


// use vexide::{
//     adi::digital::LogicLevel,
//     // predlude::*,
// };



// const TRACK_WIDTH: f64 = 10.0;
// const GEARING: f64 = 48.0 / 72.0;
// const WHEEL_DIAMETER: f64 = 3.25;


// struct Robot {
//     drivetrain: Drivetrain<Differential, WheeledTracking>,
//     intake: (Motor, Motor, Motor),
//     wing: AdiDigitalOut,
//     matchload: AdiDigitalOut,
//     triple_state: (AdiDigitalOut, AdiDigitalOut),

//     controller: Controller,
// }


// impl Robot {
//     const LINEAR_PID: Pid = Pid::new(1.0, 0.0, 0.125, None);
//     const ANGULAR_PID: AngularPid = AngularPid::new(16.0, 0.0, 1.0, None);
//     const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
//         .error(4.0)
//         .velocity(0.25)
//         .duration(Duration::from_millis(15));
//     const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
//         .error(f64::to_radians(8.0))
//         .velocity(0.09)
//         .duration(Duration::from_millis(15));
// }

// impl Compete for Robot {
//     async fn autonomous(&mut self) {
//         // todo: make auto ;;
//                 // self.triple_state.1.set_high()?;
//                 // self.triple_state.1.set_high()?;
//     }

//     async fn driver(&mut self) {

//         // let mut basic = Basic {
//         //     linear_controller: LINEAR_CONTROLLER,
//         //     angular_controller: ANGULAR_CONTROLLER,
//         //     linear_tolerances: TOLERANCES,
//         //     angular_tolerances: TOLERANCES,
//         //     timeout: Some(Duration::from_millis(2000)),
//         // };

//         loop {
//             loop {
//                 let state = self.controller.state().unwrap_or_default();

//                 _ = self
//                     .drivetrain
//                     .model
//                     .drive_arcade(state.left_stick.y(), state.left_stick.x());
//                 println!("{}", self.drivetrain.tracking.position());

//                 sleep(Motor::WRITE_INTERVAL).await;
//             }
//             // let state = self.controller.state().unwrap_or_default();

//             // let mappings = ControllerMappings {
//             //     drive_mode: DriveMode::Arcade {
//             //         power: state.right_stick,
//             //         turn: state.left_stick,
//             //     },

//             //     hoard: state.button_r1,
//             //     outake: state.button_l2,
//             //     mid: state.button_l1,
//             //     long: state.button_r2,

                
//             //     wing: state.button_up,
//             //     matchload: state.button_down,
//             // };

//             // // let power = differential_drive(&mappings.drive_mode);
//             // // _ = self.drivetrain.model.apply_power(power);
//             // _ = self
//             // .drivetrain
//             // .model
//             // .drive_arcade(state.left_stick.y(), state.left_stick.x());

//             // // neaten with refactor
//             // if mappings.hoard.is_pressed() {
//             //     _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
//             //     // _ = self.triple_state.0.set_high();
//             //     // _ = self.triple_state.1.set_high();
//             //     } else if mappings.outake.is_pressed() {
//             //     _ = self.intake.0.set_voltage(Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.1.set_voltage(Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.2.set_voltage(Motor::V5_MAX_VOLTAGE);
//             // } else if mappings.mid.is_pressed() {
//             //     _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.triple_state.0.set_low();
//             //     _ = self.triple_state.1.set_low();
//             //  } else if mappings.long.is_pressed() {
//             //     _ = self.intake.0.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.1.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.intake.2.set_voltage(-Motor::V5_MAX_VOLTAGE);
//             //     _ = self.triple_state.0.set_high();
//             //     _ = self.triple_state.1.set_low();
//             // } else {
//             //     _ = self.intake.0.set_voltage(0.0);
//             //     _ = self.intake.1.set_voltage(0.0);
//             //     _ = self.intake.2.set_voltage(0.0);
//             // }


//             // if mappings.matchload.is_now_pressed() {
//             //     _ = self.matchload.toggle();
//             // }

//             // if mappings.wing.is_now_pressed() {
//             //     _ = self.wing.toggle();
//             // }

//             // sleep(Controller::UPDATE_INTERVAL);
//         }
//     }
// }
// #[vexide::main]
// async fn main(peripherals: Peripherals) {
//     let mut imu = InertialSensor::new(peripherals.port_8);

//     imu.calibrate().await.unwrap();
    
//     let left_motors = [
//         Motor::new(perzipherals.port_14, Gearset::Blue, Direction::Forward),
//         Motor::new(peripherals.port_16, Gearset::Blue, Direction::Reverse),
//         Motor::new(peripherals.port_13, Gearset::Blue, Direction::Forward),
//     ];
    
//     let right_motors = [
//         Motor::new(peripherals.port_11, Gearset::Blue, Direction::Forward),
//         Motor::new(peripherals.port_12, Gearset::Blue, Direction::Reverse),
//         Motor::new(peripherals.port_15, Gearset::Blue, Direction::Forward),
//     ];
//     // let robot = Robot {
//     //     drivetrain: Drivetrain::new(
//     //         Differential::from_shared(left_motors.clone(), right_motors.clone()),
//     //         WheeledTracking::new(
//     //             Vec2::default(),
//     //             Angle::default(),
//     //             [
//     //                 TrackingWheel::new(
//     //                     left_motors.clone(),
//     //                     WHEEL_DIAMETER,
//     //                     TRACK_WIDTH,
//     //                     Some(GEARING),
//     //                 ),
//     //                 TrackingWheel::new(
//     //                     right_motors.clone(),
//     //                     WHEEL_DIAMETER,
//     //                     TRACK_WIDTH,
//     //                     Some(GEARING),
//     //                 ),
//     //             ],
//     //             [
//     //                 // Fake tracking wheel because i can't figure out how to work without sideways
//     //                 TrackingWheel::new(
//     //                     RotationSensor::new(peripherals.port_20, Direction::Forward),
//     //                     WHEEL_DIAMETER,
//     //                     TRACK_WIDTH,
//     //                     Some(GEARING),
//     //                 ),
//     //             ],
//     //             Some(imu),
//     //         ),
//     //     ),
//     //     intake: (
//     //         Motor::new(peripherals.port_6, Gearset::Blue, Direction::Reverse),
//     //         Motor::new(peripherals.port_7, Gearset::Blue, Direction::Reverse),
//     //         Motor::new(peripherals.port_5, Gearset::Blue, Direction::Reverse),
//     //     ),
//     //     matchload: AdiDigitalOut::with_initial_level(peripherals.adi_b, LogicLevel::Low),
//     //     wing: AdiDigitalOut::with_initial_level(peripherals.adi_d, LogicLevel::Low),
//     //     triple_state: (
//     //         AdiDigitalOut::with_initial_level(peripherals.adi_e, LogicLevel::Low),
//     //         AdiDigitalOut::with_initial_level(peripherals.adi_f, LogicLevel::Low),
//     //     ),
//     //     controller: peripherals.primary_controller,
//     // };
//     // robot.compete().await;
//     Robot {
//         drivetrain: Drivetrain::new(
//             Differential::new(left_motors, right_motors),
//             WheeledTracking::new(
//                 (0.0, 0.0),
//                 90.0.deg(),
//                 [TrackingWheel::new(forwards_enc, 2.0, 0.0, None)],
//                 [TrackingWheel::new(sideways_enc, 2.0, 0.0, None)],
//                 Some(imu),
//             ),
//         ),
//         controller: peripherals.primary_controller,
//     }
//     .compete()
//     .await;
// }


use std::time::Duration;

use evian::prelude::*;
use vexide::prelude::*;

use evian::{
    control::loops::{AngularPid, Pid},
    drivetrain::model::{Arcade, Differential},
    math::{Angle, Vec2},
    motion::{Basic, Seeking},
    tracking::wheeled::{TrackingWheel, WheeledTracking},
};

const TRACK_WIDTH: f64 = 10.0;
const GEARING: f64 = 48.0 / 72.0;
const WHEEL_DIAMETER: f64 = 3.25;

struct Robot {
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    controller: Controller,
}

impl Robot {
    const LINEAR_PID: Pid = Pid::new(1.0, 0.0, 0.125, None);
    const ANGULAR_PID: AngularPid = AngularPid::new(16.0, 0.0, 1.0, None);
    const LINEAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(4.0)
        .velocity(0.25)
        .duration(Duration::from_millis(15));
    const ANGULAR_TOLERANCES: Tolerances = Tolerances::new()
        .error(f64::to_radians(8.0))
        .velocity(0.09)
        .duration(Duration::from_millis(15));
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

        // Turn to 0 degrees heading.
        basic.turn_to_heading(dt, 0.0.deg()).await;
        basic.turn_to_heading(dt, 90.0.deg()).await;

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
            println!("{}", self.drivetrain.tracking.position());

            sleep(Motor::WRITE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let left_motors = [
        Motor::new(peripherals.port_14, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_16, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_13, Gearset::Blue, Direction::Reverse),
    ];
    
    let right_motors = [
        Motor::new(peripherals.port_11, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_15, Gearset::Blue, Direction::Forward),
    ];

    let mut imu = InertialSensor::new(peripherals.port_17);
    imu.calibrate().await.unwrap();

    Robot {
        // drivetrain: Drivetrain::new(
        //     Differential::new(left_motors, right_motors),
        //     WheeledTracking::new(
        //         Vec2::default(),
        //         Angle::default(),
        //         [
        //             TrackingWheel::new(
        //                 AdiOpticalEncoder::new(peripherals.adi_a, peripherals.adi_b),
        //                 WHEEL_DIAMETER,
        //                 TRACK_WIDTH,
        //                 Some(GEARING),
        //             ),
        //             TrackingWheel::new(
        //                 AdiOpticalEncoder::new(peripherals.adi_c, peripherals.adi_d),
        //                 WHEEL_DIAMETER,
        //                 TRACK_WIDTH,
        //                 Some(GEARING),
        //             ),
        //         ],
        //         [
        //             TrackingWheel::new(
        //                 RotationSensor::new(peripherals.port_20, Direction::Forward),
        //                 WHEEL_DIAMETER,
        //                 TRACK_WIDTH,
        //                 Some(GEARING),
        //             ),
        //         ],
        //         Some(imu),
        //     ),
        // ),
        drivetrain: Drivetrain::new(
            Differential::new(left_motors, right_motors),
            WheeledTracking::new(
                Vec2::default(),
                Angle::default(),
                [
                    TrackingWheel::new(
                        &left_motors[0],
                        WHEEL_DIAMETER,
                        TRACK_WIDTH,
                        Some(GEARING),
                    ),
                    TrackingWheel::new(
                        &right_motors[0],
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
        controller: peripherals.primary_controller,
    }
    .compete()
    .await;
}
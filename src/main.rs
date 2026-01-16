mod subsystems;
use subsystems::{
    drivetrain::differential_drive,
    intake::{Intake, IntakeCommand},
    pneumatics::Pneumatics,
};

use std::time::Duration;

use evian::prelude::*;
use vexide::prelude::*;

use evian::{
    control::loops::{AngularPid, Pid},
    drivetrain::model::{Arcade, Differential},
    motion::{Basic, Seeking},
    tracking::wheeled::{TrackingWheel, WheeledTracking},
};

struct Robot {
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    intake: Intake,
    pneumatics: Pneumatics,
    controller: Controller,
}

#[vexide::main]
async fn main(peripherals: Peripherals) {

    let left_motors = [
        Motor::new(peripherals.port_16, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_13, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_14, Gearset::Blue, Direction::Forward),
    ];
    let right_motors = [
        Motor::new(peripherals.port_11, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_15, Gearset::Blue, Direction::Forward),
    ];

    let mut imu = InertialSensor::new(peripherals.port_10);
    imu.calibrate().await.unwrap();

    Robot {
        drivetrain: Drivetrain::new(
            Differential::new(left_motors, right_motors),
            WheeledTracking::new(
                (0.0, 0.0),
                90.0.deg(),
                [TrackingWheel::new(forwards_enc, 2.0, 0.0, None)],
                [TrackingWheel::new(sideways_enc, 2.0, 0.0, None)],
                Some(imu),
            ),
        ),
        controller: peripherals.primary_controller,
    }
    .compete()
    .await;
}
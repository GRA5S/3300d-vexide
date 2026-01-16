mod subsystems;
use subsystems::{
    drivetrain::differential_drive,
    intake::{Intake, IntakeCommand},
    pneumatics::Pneumatics,
};

use evian::prelude::*;
use vexide::prelude::*;

struct Robot {
    drivetrain: Drivetrain<Differential, WheeledTracking>,
    intake: Intake,
    pneumatics: Pneumatics,
    controller: Controller,
}
/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: main.rs                                 */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Dec 12th 2025 10:30AM          */
/*    Description: Example Main file                */
/*                                                  */
/* ------------------------------------------------ */

// To build the file run command
// cargo v5 build
// to upload the file run the command
// cargo v5 upload --slot # --release
#![no_main]
#![no_std]

use autons::{
    prelude::*,
    simple::{route, SimpleSelect},
};
use vexide::prelude::*;
mod eclipselib;
mod robot;

extern crate alloc;
pub use alloc::vec;

struct Robot {
    controller: Controller,
    swerve: swervelib::swervedrive::SwerveDrive,
    smartmtr: eclipselib::motors::AdvMotor,
}

impl Robot {
    async fn test_auto(&mut self) {}
}

impl SelectCompete for Robot {
    async fn connected(&mut self) {
        println!("Pre-Initiated");
    }
    async fn disconnected(&mut self) {
        println!("Controller Disconnected")
    }
    async fn disabled(&mut self) {
        println!("Awaiting Competition")
    }
    async fn driver(&mut self) {
        println!("Driver!");

        loop {
            let controller_state = self.controller.state().unwrap_or_default();
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        controller: peripherals.primary_controller,
        swerve: swervelib::swervedrive::SwerveDrive::new(
            swervelib::swervemod::SwerveModule::new(
                swervelib::swervemotors::SwerveMotor::new(
                    peripherals.port_1,
                    Gearset::Blue,
                    Direction::Forward,
                ),
                swervelib::swervemotors::SwerveMotor::new(
                    peripherals.port_2,
                    Gearset::Blue,
                    Direction::Forward,
                ),
                RotationSensor::new(peripherals.port_3, Direction::Forward),
            ),
            swervelib::swervemod::SwerveModule::new(
                swervelib::swervemotors::SwerveMotor::new(
                    peripherals.port_4,
                    Gearset::Blue,
                    Direction::Forward,
                ),
                swervelib::swervemotors::SwerveMotor::new(
                    peripherals.port_5,
                    Gearset::Blue,
                    Direction::Forward,
                ),
                RotationSensor::new(peripherals.port_6, Direction::Forward),
            ),
            InertialSensor::new(peripherals.port_7),
        ),
        smartmtr: eclipselib::motors::AdvMotor::new(
            peripherals.port_8,
            Gearset::Blue,
            Direction::Forward,
        ),
    };

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [route!("Test Auto", Robot::test_auto)],
        ))
        .await;
}

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

use autons::{
    prelude::*,
    simple::{SimpleSelect, route},
};
use vexide::prelude::*;
mod eclipselib;
mod robot;

struct Robot {
    controller: Controller,
    swerve: eclipselib::swerve::swervedrive::DualSwerveDrive,
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
        swerve: eclipselib::swerve::swervedrive::DualSwerveDrive::new(
            eclipselib::swerve::swervemod::SwerveModule::new22w(
                Motor::new(peripherals.port_1, Gearset::Blue, Direction::Forward),
                Motor::new(peripherals.port_2, Gearset::Blue, Direction::Forward),
                RotationSensor::new(peripherals.port_3, Direction::Forward),
            ),
            eclipselib::swerve::swervemod::SwerveModule::new22w(
                Motor::new(peripherals.port_4, Gearset::Blue, Direction::Forward),
                Motor::new(peripherals.port_5, Gearset::Blue, Direction::Forward),
                RotationSensor::new(peripherals.port_6, Direction::Forward),
            ),
            InertialSensor::new(peripherals.port_7),
            eclipselib::pid::PIDController::set_gains(0.0, 0.0, 0.0),
        ),
    };

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [route!("Test Auto", Robot::test_auto)],
        ))
        .await;
}

/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: main.rs                                 */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Nov 05th 2025 02:42PM          */
/*    Team: BBR1                                    */
/*    Description: Example Main file                */
/*                                                  */
/* ------------------------------------------------ */

#![no_main]
#![no_std]

use autons::{
    prelude::*,
    simple::{route, SimpleSelect},
};
use vexide::prelude::*;
mod eclipselib; // Use eclipselib
extern crate alloc;
pub use alloc::vec;

struct Robot { 
    controller: Controller,
    left_drive: eclipselib::motors::MotorGroup,
    right_drive: eclipselib::motors::MotorGroup,
    odometry: eclipselib::odometry::Dual_Track_Odometry, 
    smartmtr: eclipselib::motors::AdvMotor

}

impl Robot{
    async fn match_auto(&mut self) {

    }
    async fn elims_auto(&mut self) {

    }
    async fn skills_auto(&mut self) {

    }
}

impl SelectCompete for Robot {
    async fn connected(&mut self) {
        println!("Pre-Initiated");
        self.odometry.reset();

    }
    async fn disconnected(&mut self) {
        println!("Controller Disconnected")
    }
    async fn disabled(&mut self) {
        println!("Awaiting Competition")
    }
    async fn driver(&mut self) {
        println!("Driver!");
        // Define booleans for motor controllers 
        let mut drive_bool: bool = true;
        let mut mtr_toggle_bool: bool = false;

        loop{
            let controller_state = self.controller.state().unwrap_or_default();
            let forward_speed  = controller_state.right_stick.y(); 
            let turn_speed = controller_state.left_stick.x(); //note to set turn speed on a log scale fopr precision
            if forward_speed.abs() < 5.0 && turn_speed.abs() < 5.0{
                if drive_bool {
                    self.left_drive.stop();
                    self.right_drive.stop();
                    drive_bool = false
                } else {
                    drive_bool = true
                }
            }
            if drive_bool {
                self.left_drive.set_voltage((forward_speed + turn_speed)*Motor::V5_MAX_VOLTAGE);
                self.right_drive.set_voltage((forward_speed - turn_speed)*Motor::V5_MAX_VOLTAGE);
            }
            if controller_state.button_l2.is_pressed(){
                self.smartmtr.toggle(12.0);
                mtr_toggle_bool = true;
            } else if mtr_toggle_bool {
                self.smartmtr.stop();
                mtr_toggle_bool = false;
            }
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {

    let robot = Robot {
        controller: peripherals.primary_controller,
        
        left_drive: eclipselib::motors::MotorGroup::new(vec![
            Motor::new(peripherals.port_1, Gearset::Blue, Direction::Forward),
            Motor::new(peripherals.port_2, Gearset::Blue, Direction::Forward),
            Motor::new(peripherals.port_3, Gearset::Blue, Direction::Forward), 
        ]),
        right_drive: eclipselib::motors::MotorGroup::new(vec![
            Motor::new(peripherals.port_4, Gearset::Blue, Direction::Forward), 
            Motor::new(peripherals.port_5, Gearset::Blue, Direction::Forward), 
            Motor::new(peripherals.port_6, Gearset::Blue, Direction::Forward), 
        ]),
        odometry: eclipselib::odometry::Dual_Track_Odometry::new(
            RotationSensor::new(peripherals.port_7, Direction::Forward),
            RotationSensor::new(peripherals.port_8, Direction::Forward),
            InertialSensor::new(peripherals.port_9),
        ),
        smartmtr: eclipselib::motors::AdvMotor::new(
            peripherals.port_10, Gearset::Blue, Direction::Forward,
        )

    };

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [
                route!("Match Auto", Robot::match_auto),
                route!("Elims Auto", Robot::elims_auto),
                route!("Skills", Robot::skills_auto),
            ],
        ))
        .await;
}

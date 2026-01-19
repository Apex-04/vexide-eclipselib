/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib / swerve           */
/*    File: swervemotor.rs                          */
/*    Author: Andrew Bobay                          */
/*    Date Created: Dec 12th 2025 10:30AM           */
/*    Date Modified: Jan 18th 2025 08:40PM          */
/*    Description: swerve motors modules            */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec::Vec;

use vexide::prelude::*;

use crate::eclipselib;
pub struct SwerveMotor {
    motor: eclipselib::motors::AdvMotor,
}
#[allow(unused)]
impl SwerveMotor {
    pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self {
        Self {
            motor: eclipselib::motors::AdvMotor::new(port, gearset, direction),
        }
    }
}

// Used for 88W implemenation
pub struct SwerveMotorGroup {
    motors: Vec<SwerveMotor>,
}
#[allow(unused)]
impl SwerveMotorGroup {
    pub fn new(motors: Vec<SwerveMotor>) -> Self {
        Self { motors }
    }
}

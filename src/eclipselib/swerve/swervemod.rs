/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib / swerve           */
/*    File: swervemod.rs                            */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: swerve modules                   */
/*                                                  */
/* ------------------------------------------------ */

use vexide::{devices::PortError, prelude::*};

const WHEEL_SIZE: f64 = 3.25;

pub struct SwerveModule {
    smartmtr_top: Vec<Motor>,
    smartmtr_bottom: Vec<Motor>,
    azimuth: RotationSensor,
}

#[allow(unused)]
impl SwerveModule {
    /// Create a standard 2-motor swerve module (one top, one bottom)
    pub fn new22w(smartmtr_top: Motor, smartmtr_bottom: Motor, azimuth: RotationSensor) -> Self {
        Self {
            smartmtr_top: vec![smartmtr_top],
            smartmtr_bottom: vec![smartmtr_bottom],
            azimuth,
        }
    }

    /// Create a 4-motor swerve module (two top, two bottom)
    pub fn new44w(
        smartmtr_top1: Motor,
        smartmtr_top2: Motor,
        smartmtr_bottom1: Motor,
        smartmtr_bottom2: Motor,
        azimuth: RotationSensor,
    ) -> Self {
        Self {
            smartmtr_top: vec![smartmtr_top1, smartmtr_top2],
            smartmtr_bottom: vec![smartmtr_bottom1, smartmtr_bottom2],
            azimuth,
        }
    }

    pub fn get_azimuth(&mut self) -> Result<Position, PortError> {
        self.azimuth.position()
    }
}

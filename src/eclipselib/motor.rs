/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: motors.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: Eclipselib advanced motor        */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::{
    math::Angle,
    prelude::*,
    smart::{PortError, motor::BrakeMode},
};

extern crate alloc;
use alloc::vec::Vec;

pub struct MotorGroup {
    motors: Vec<Motor>,
}

#[allow(unused)]
impl MotorGroup {
    pub fn new(motors: Vec<Motor>) -> Self {
        Self { motors }
    }

    /// Set voltage for all motors in the group (in millivolts, -12000 to 12000)
    pub fn set_voltage(&mut self, millivolts: f64) -> Result<(), PortError> {
        for motor in self.motors.iter_mut() {
            motor.set_voltage(millivolts)?;
        }
        Ok(())
    }

    /// Stop all motors by setting voltage to 0
    pub fn stop(&mut self) -> Result<(), PortError> {
        self.set_voltage(0.0)
    }

    //  Brake all motors with the specified brake mode
    pub fn brake(&mut self, mode: BrakeMode) -> Result<(), PortError> {
        for motor in self.motors.iter_mut() {
            motor.brake(mode);
        }
        Ok(())
    }

    /// Get the average position of all motors in the group (in degrees)
    pub fn position(&self) -> Result<f64, PortError> {
        let mut total = 0.0;
        for motor in self.motors.iter() {
            total += motor.position()?.as_degrees();
        }
        Ok(total / self.motors.len() as f64)
    }

    /// Reset position for all motors to 0
    pub fn reset_position(&mut self) -> Result<(), PortError> {
        for motor in self.motors.iter_mut() {
            motor.reset_position()?;
        }
        Ok(())
    }

    /// Spin motors to a target position using position control
    pub fn spin_to(&mut self, target_degrees: f64, rpm: i32) -> Result<(), PortError> {
        for motor in self.motors.iter_mut() {
            motor.set_position_target(Angle::from_degrees(target_degrees), rpm)?;
        }
        Ok(())
    }

    /// Get number of motors in the group
    pub fn len(&self) -> usize {
        self.motors.len()
    }

    /// Check if the motor group is empty
    pub fn is_empty(&self) -> bool {
        self.motors.is_empty()
    }
}

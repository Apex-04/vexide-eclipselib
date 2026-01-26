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

use vexide::prelude::*;

pub struct MotorGroup {
    motors: Vec<Motor>,
}

#[allow(unused)]
impl MotorGroup {
    pub fn new(motors: Vec<Motor>) -> Self {
        Self { motors }
    }

    /// Set voltage for all motors in the group (in millivolts, -12000 to 12000)
    pub fn set_voltage(&mut self, millivolts: i32) -> Result<(), MotorError> {
        for motor in self.motors.iter_mut() {
            motor.set_voltage(millivolts)?;
        }
        Ok(())
    }

    /// Stop all motors by setting voltage to 0
    pub fn stop(&mut self) -> Result<(), MotorError> {
        self.set_voltage(0)
    }

    /// Brake all motors with the specified brake mode
    pub fn brake(&mut self, mode: BrakeMode) -> Result<(), MotorError> {
        for motor in self.motors.iter_mut() {
            motor.set_brake_mode(mode)?;
        }
        self.set_voltage(0)
    }

    /// Get the average position of all motors in the group (in degrees)
    pub fn position(&self) -> Result<f64, MotorError> {
        let mut total = 0.0;
        for motor in self.motors.iter() {
            total += motor.position()?.as_degrees();
        }
        Ok(total / self.motors.len() as f64)
    }

    /// Get the average velocity of all motors in the group (in RPM)
    pub fn velocity(&self) -> Result<f64, MotorError> {
        let mut total = 0.0;
        for motor in self.motors.iter() {
            total += motor.velocity()?.as_rpm();
        }
        Ok(total / self.motors.len() as f64)
    }

    /// Reset position for all motors to 0
    pub fn reset_position(&mut self) -> Result<(), MotorError> {
        for motor in self.motors.iter_mut() {
            motor.reset_position()?;
        }
        Ok(())
    }

    /// Get the average temperature of all motors in the group (in Celsius)
    pub fn temperature(&self) -> Result<f64, MotorError> {
        let mut total = 0.0;
        for motor in self.motors.iter() {
            total += motor.temperature()?.as_celsius();
        }
        Ok(total / self.motors.len() as f64)
    }

    /// Spin motors to a target position using position control
    pub fn spin_to(&mut self, target_degrees: f64, max_voltage: i32) -> Result<(), MotorError> {
        for motor in self.motors.iter_mut() {
            // Set target position for built-in position control
            motor.set_target(target_degrees, max_voltage)?;
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

/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: motors.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Dec 12th 2025 10:30AM          */
/*    Description: Eclipselib advanced motor        */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec::Vec;

use vexide::prelude::*;
extern crate alloc;
use core::time::Duration;

pub struct AdvMotor {
    motor: Motor,
}

#[allow(unused)]
impl AdvMotor {
    // Creates a new AdvMotor Object
    pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self {
        Self {
            motor: Motor::new(port, gearset, direction),
        }
    }
    pub fn toggle(&mut self, volts: f64) {
        let _ = self.motor.set_voltage(volts);
    }
    pub fn stop(&mut self) {
        let _ = self.motor.set_voltage(0.0);
    }

    pub fn spin(&mut self, volts: f64) {
        let _ = self.motor.set_voltage(volts);
    }

    pub fn spin_to(&mut self, volts: f64){

    }

}

pub struct MotorGroup {
    motors: Vec<Motor>,
}

#[allow(unused)]
impl MotorGroup {
    pub fn new(motors: Vec<Motor>) -> Self {
        Self { motors }
    }

    pub fn toggle(&mut self, volts: f64) {
        for motor in self.motors.iter_mut() {
            let _ = motor.set_voltage(volts);
        }
    }

    pub fn stop(&mut self) {
        for motor in self.motors.iter_mut() {
            let _ = motor.set_voltage(0.0);
        }
    }

    pub fn spin_to(&mut self, distance: f64, volts: f64) {
        for motor in self.motors.iter_mut() {
            // Implement P loop here later
        }
    }

    pub fn spin(&mut self, volts: f64) {
        for motor in self.motors.iter_mut() {
            let _ = motor.set_voltage(volts);
        }
    }
}

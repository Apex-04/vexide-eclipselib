/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: motors.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Nov 17th 2025 3:00PM           */
/*    Team: BBR1                                    */
/*    Description: Eclipselib advanced motor        */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */


use alloc::vec::Vec;
use vexide::devices::smart::imu::InertialError;
use vexide::devices::{PortError, peripherals}; 
use vexide::prelude::*;
extern crate alloc;
pub use alloc::vec;
use core::time::{self, Duration};


pub struct AdvMotor{
    motor: Motor
}

#[allow(unused)]
impl AdvMotor{
// Creates a new AdvMotor Object 
pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self{
        Self{motor: Motor::new(port, gearset, direction)}
}
pub fn toggle(&mut self, volts: f64){
    let _ = self.motor.set_voltage(volts);

}   
pub fn stop(&mut self){
    let _ = self.motor.set_voltage(0.0);
}

pub fn spin(&mut self, volts: f64){
    let _ = self.motor.set_voltage(volts);
}

pub async fn pid_spin_for(&mut self, target: f64, kp: f64, ki: f64, kd: f64) {
    let _ = self.motor.set_position(Position::from_degrees(0.0));

    let mut last_error = 0.0;
    let mut integral = 0.0;

    while let Ok(current_position) = self.motor.position() {
        let current_degrees = current_position.as_degrees();
        let error = target - current_degrees;

        if error.abs() < 0.5 {
            break;
        }

        integral += error;                       // I-term (scaled by loop frequency)
        let derivative = error - last_error;     // D-term (change per loop)
        last_error = error;

        // PID control output
        let output = (kp * error) + (ki * integral) + (kd * derivative);

        // Clamp and apply to motor
        let max_voltage = self.motor.max_voltage();
        let _ = self.motor.set_voltage(output.clamp(-max_voltage, max_voltage));

        // small wait to keep dt constant
        vexide::time::sleep(Duration::from_millis(10)).await;
    }

    let _ = self.motor.brake(BrakeMode::Hold);
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

    pub fn spin_for(&mut self, distance: f64, volts: f64) {
        for motor in self.motors.iter_mut() {
            // Implement P loop here later
        }
    }

    pub fn set_voltage(&mut self, volts: f64) {
        for motor in self.motors.iter_mut() {
            let _ = motor.set_voltage(volts);
        }
    }
}

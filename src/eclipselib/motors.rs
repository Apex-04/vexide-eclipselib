/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: motors.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Nov 10th 2025 10:00AM          */
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

pub struct AdvMotor{
    motor: Motor
}

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

pub async fn spin_for(&mut self, target: f64, p_value: f64) {
    let _ = self.motor.set_position(Position::from_degrees(0.0));


    // Loop until the motor is close to the target
    while let Ok(current_position) = self.motor.position() {
        if current_position.as_degrees() > target.abs() {
               break;
           }
   
           // Calculate the error, the difference between the target and the current position.
           let pct_error = 100.0 - (current_position.as_degrees()/target);
   
           // Calculate the voltage to send to the motor.
           // This is proportional to the error, which is why it's called a P-loop.
           let voltage = pct_error * p_value;
   
           // Clamp the voltage to the motor's maximum voltage range.
           let max_voltage = self.motor.max_voltage();
           let clamped_voltage = voltage.clamp(-max_voltage, max_voltage);
   
           // Set the motor's voltage.
           let _ = self.motor.set_voltage(clamped_voltage);
       }  
   
       // Stop the motor once the target is reached
       let _ = self.motor.brake(BrakeMode::Hold);
   }
}
pub struct MotorGroup {
    motors: Vec<Motor>,
}

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

/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: motors.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Oct 21st 2025 11:20AM          */
/*    Team: BBR1                                    */
/*    Description: Eclipselib advanced motor        */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */


use alloc::vec; 
use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;

pub struct AdvMotor{
    motor: Motor
}

impl AdvMotor{
pub fn new(motor:Motor) -> Self{
        Self{motor}
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

pub struct MotorGroup{
    motors: vec::Vec<Motor>
}

impl MotorGroup {
    pub fn new2_mtr_group(motor1:Motor, motor2:Motor) -> Self {
        Self{
            motors: vec![motor1, motor2]
        }
    }
  pub fn new3_mtr_group(motor1:Motor, motor2:Motor, motor3:Motor) -> Self {
        Self{
            motors: vec![motor1, motor2, motor3]
        }
    }

    pub fn toggle(&mut self, volts: f64){
        for motor in self.motors.iter_mut(){
            // spin all motors in given direction
            let _ = motor.set_voltage(volts);
        }
    }
    pub fn stop(&mut self){
        for motor in self.motors.iter_mut(){
            // spin all motors in given direction
           let _ = motor.set_voltage(0.0);
        }
    }
    // uses a p-loop to move a motors to their target
    pub fn spin_for(&mut self, distance: f64, volts:f64){
        for motor in self.motors.iter_mut(){
            // Implement P Loop
        }
    }
    pub fn set_voltage(&mut self, volts: f64){
        for motor in self.motors.iter_mut(){
            let _ = motor.set_voltage(volts);
        }
    }
}
/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: eclipselib.rs                           */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Oct 20th 2025 12:30PM          */
/*    Team: BBR1                                    */
/*    Description: eclipselib file                  */
/*                                                  */
/* ------------------------------------------------ */


use alloc::vec; 
use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;

// Advanced Implementations of other constructs, when defining a stand alone object it is best to use these
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


pub struct Solonoid{
    solonoid: AdiDigitalOut
}
impl Solonoid{
    pub fn new(port: AdiPort) -> Self{
        Self{
            solonoid: AdiDigitalOut::new(port)
        }

    }
    pub fn set_high(&mut self){
        let _ = self.solonoid.set_high();
    }
    pub fn set_low(&mut self){
        let _ = self.solonoid.set_low();
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


// Using ADI devices instead of just taking the ports to make it easier on the backend
pub struct AdiPortGroup{
    adi_devices: vec::Vec<AdiDigitalOut>
}
impl AdiPortGroup{
    pub fn new2_adi_group(device1: AdiPort, device2: AdiPort) -> Self{
        Self{
            adi_devices: vec![AdiDigitalOut::new(device1), AdiDigitalOut::new(device2)]
        }
    }

    // Toggle is not very verbose as ut doesnt tell you exactly which state it's in, best practice only use set_high and set_low
    pub fn set_high(&mut self){
        for adi_device in self.adi_devices.iter_mut(){
            let _ = adi_device.set_high();
        }
    }
    pub fn set_low(&mut self){
        for adi_device in self.adi_devices.iter_mut(){
            let _ = adi_device.set_low();
        }
    }
}


pub struct Odometry{
    rotation_back: RotationSensor,
    rotation_front: RotationSensor,
    inertial: InertialSensor
}
impl Odometry{
    pub fn new2_rot_odom(rotation_back:RotationSensor, rotation_front:RotationSensor, inertial:InertialSensor) -> Self{
        Self{rotation_back, rotation_front, inertial}
    }

    pub fn get_heading(&mut self) -> Result<f64, InertialError>{
        self.inertial.heading()
    }
    pub fn get_back_position(&mut self) -> Result<Position, PortError>{
        self.rotation_back.position()
    }
    pub fn get_front_position(&mut self) -> Result<Position, PortError>{
        self.rotation_front.position()
    }
    pub fn reset(&mut self) -> i8{
        let _ = self.rotation_back.set_position(Position::from_degrees(0.0));
        let _ = self.rotation_front.set_position(Position::from_degrees(0.0));
        let _ = self.inertial.set_heading(0.0);
        0 
    }


}

pub struct AdvDrive{
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    gear_ratio: f64,
    wheel_szie: f64,
    gear_set: Gearset,
    odometry: Odometry,    
}

impl AdvDrive{


}


pub struct SimpleDrive{
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    inertial: InertialSensor,
    gear_ratio: f64,
    wheel_szie: f64,
    gear_set: Gearset,
}

impl SimpleDrive{


}

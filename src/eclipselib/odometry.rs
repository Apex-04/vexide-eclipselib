/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: odometry.rs                             */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Oct 21st 2025 11:20AM          */
/*    Team: BBR1                                    */
/*    Description: Eclipselib Odometry              */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec; 
use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;


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
/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: odometry.rs                             */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Nov 17th 2025 3:00PM           */
/*    Team: BBR1                                    */
/*    Description: Eclipselib Odometry              */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::devices::smart::rotation;
use vexide::prelude::*;


pub struct DualTrackOdometry{
    rotation_back: RotationSensor,
    rotation_front: RotationSensor,
    inertial: InertialSensor
}
#[allow(unused)]
impl DualTrackOdometry{
    pub fn new(rotation_back:RotationSensor, rotation_front:RotationSensor, inertial:InertialSensor) -> Self{
        Self{rotation_back, rotation_front, inertial}
    }

    pub fn update_position(&mut self){
        // Function Body
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
    pub fn reset(&mut self){
        let _ = self.rotation_back.set_position(Position::from_degrees(0.0));
        let _ = self.rotation_front.set_position(Position::from_degrees(0.0));
        let _ = self.inertial.set_heading(0.0);
    }
}

pub struct TriTrackOdometry{
    rotation_left: RotationSensor,
    rotation_right: RotationSensor,
    rotation_back: RotationSensor
}

#[allow(unused)]
impl TriTrackOdometry{
    pub fn new(rotation_left:RotationSensor, rotation_right:RotationSensor, rotation_back:RotationSensor) -> Self{
        Self{rotation_left, rotation_right, rotation_back}
    }

    pub fn update_position(&mut self){
        // Function Body
    }
    
    pub fn get_back_position(&mut self) -> Result<Position, PortError>{
        self.rotation_back.position()
    }
    pub fn get_right_position(&mut self) -> Result<Position, PortError>{
        self.rotation_right.position()
    }
    pub fn get_left_position(&mut self) -> Result<Position, PortError>{
        self.rotation_left.position()
    }
    pub fn reset(&mut self){
        let _ = self.rotation_back.set_position(Position::from_degrees(0.0));
        let _ = self.rotation_left.set_position(Position::from_degrees(0.0));
        let _ = self.rotation_right.set_position(Position::from_degrees(0.0));

    }
}
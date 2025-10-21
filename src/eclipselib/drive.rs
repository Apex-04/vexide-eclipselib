/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: drive.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Oct 21st 2025 11:20AM          */
/*    Team: BBR1                                    */
/*    Description: Eclipselib smart drivetrain      */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec; 
use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;
use crate::eclipselib::motors::*;
use crate::eclipselib::odometry::*;



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

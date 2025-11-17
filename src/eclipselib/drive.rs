/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: drive.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Nov 17th 2025 3:00PM           */
/*    Team: BBR1                                    */
/*    Description: Eclipselib smart drivetrain      */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;
use crate::eclipselib::motors::*;
use crate::eclipselib::odometry::*;


// An Advanced drive that uses eclipselib:odomotry for advanced movement capabilitys
pub struct Drivetrain{
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    gear_ratio: f64,
    wheel_szie: f64,
    gear_set: Gearset,
    inertial: Option<InertialSensor>,
    dual_odom: Option<DualTrackOdometry>,
    tri_odom: Option<TriTrackOdometry>,

}

#[allow(unused)]
impl Drivetrain{


}


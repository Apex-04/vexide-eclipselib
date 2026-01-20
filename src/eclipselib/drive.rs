/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: drive.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Dec 12th 2025 10:30AM          */
/*    Description: Eclipselib smart drivetrain      */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::{devices::controller::ControllerState, prelude::*};

use crate::eclipselib::{motors::*, odometry::*};

// An Advanced drive that uses eclipselib:odomotry for advanced movement capabilitys
pub struct Drivetrain {
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
impl TankDrive {
    pub fn new_simpledrive (
        left_drive: MotorGroup,
        right_drive: MotorGroup,
        gear_ratio: f64,
        wheel_size: f64,
        gear_set: Gearset,
        inertial: InertialSensor,
    ) -> Self {
        Self {
            left_drive,
            right_drive,
            gear_ratio,
            wheel_szie: wheel_size,
            gear_set,
            inertial: Some(inertial),
            dual_odom: None,
            tri_odom: None,
        }
    }

    pub fn new_odomdrive(
        left_drive: MotorGroup,
        right_drive: MotorGroup,
        gear_ratio: f64,
        wheel_size: f64,
        gear_set: Gearset,
        dual: DualTrackOdometry,
    ) -> Self {
        Self {
            left_drive,
            right_drive,
            gear_ratio,
            wheel_szie: wheel_size,
            gear_set,
            inertial: None,
            dual_odom: Some(dual),
            tri_odom: None,
        }
        }
    }

    pub fn opc_drive(controller_state: ControllerState) {
        /* */
    }
}

pub struct XDrive {
    east_drive: MotorGroup, // Front Left && Back Right
    west_drive: MotorGroup, // Front Right && Back Left
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
}

#[allow(unused)]
impl XDrive {
    fn new(
        east_drive: MotorGroup,
        west_drive: MotorGroup,
        gear_ratio: f64,
        wheel_size: f64,
        gear_set: Gearset,
    ) -> Self {
        Self {
            east_drive,
            west_drive,
            gear_ratio,
            wheel_size,
            gear_set,
        }
    }

    fn opc_drive(&mut self, controller_state: ControllerState) {}

    fn drive_to_coordinates(&mut self, controller_state: ControllerState) {}
}

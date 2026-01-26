/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: drive.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: Eclipselib smart drivetrain      */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::{devices::controller::ControllerState, prelude::*};

use crate::eclipselib::{motors::*, odometry::*};

/// Trait defining the interface for tank drive systems
pub trait TankDrive {
    /// Operator control drive using controller input
    fn opc_drive(&mut self, controller_state: ControllerState);

    /// Drive to a specific distance using PID control
    fn drive_to(&mut self, distance: f64);

    /// Get the left drive motor group
    fn left_drive(&mut self) -> &mut MotorGroup;

    /// Get the right drive motor group
    fn right_drive(&mut self) -> &mut MotorGroup;
}

/// Simple tank drive with basic motor control
pub struct SimpleDrive {
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    inertial: InertialSensor,
}

/// Advanced tank drive with odometry capabilities
pub struct OdomDrive {
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    odom: DualTrackOdometry,
}

#[allow(unused)]
impl SimpleDrive {
    pub fn new(
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
            wheel_size,
            gear_set,
            inertial,
        }
    }
}

#[allow(unused)]
impl OdomDrive {
    pub fn new(
        left_drive: MotorGroup,
        right_drive: MotorGroup,
        gear_ratio: f64,
        wheel_size: f64,
        gear_set: Gearset,
        odom: DualTrackOdometry,
    ) -> Self {
        Self {
            left_drive,
            right_drive,
            gear_ratio,
            wheel_size,
            gear_set,
            odom,
        }
    }
}

impl TankDrive for SimpleDrive {
    fn opc_drive(&mut self, controller_state: ControllerState) {
        // Get joystick values
        let left_y = controller_state.left_stick.y();
        let right_y = controller_state.right_stick.y();

        // Set motor voltages based on joystick input
        let _ = self.left_drive.set_voltage((left_y * 12000.0) as i32);
        let _ = self.right_drive.set_voltage((right_y * 12000.0) as i32);
    }

    fn drive_to(&mut self, distance: f64) {
        // TODO: Implement PID control to drive to distance
        // Convert distance (inches) to encoder ticks based on gear_ratio and wheel_size
    }

    fn left_drive(&mut self) -> &mut MotorGroup {
        &mut self.left_drive
    }

    fn right_drive(&mut self) -> &mut MotorGroup {
        &mut self.right_drive
    }
}

impl TankDrive for OdomDrive {
    fn opc_drive(&mut self, controller_state: ControllerState) {
        // Get joystick values
        let left_y = controller_state.left_stick.y();
        let right_y = controller_state.right_stick.y();

        // Set motor voltages based on joystick input
        let _ = self.left_drive.set_voltage((left_y * 12000.0) as i32);
        let _ = self.right_drive.set_voltage((right_y * 12000.0) as i32);

        // Update odometry with current motor positions
        // self.dual_odom.update();
    }

    fn drive_to(&mut self, distance: f64) {
        // TODO: Implement odometry-based PID control to drive to distance
    }

    fn left_drive(&mut self) -> &mut MotorGroup {
        &mut self.left_drive
    }

    fn right_drive(&mut self) -> &mut MotorGroup {
        &mut self.right_drive
    }
}

pub struct XDrive {
    east_drive: MotorGroup, // Front Left && Back Right
    west_drive: MotorGroup, // Front Right && Back Left
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    spline: Spine,
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
            spline: spline(0.0, 0.0, 0.0),
        }
    }

    fn opc_drive(&mut self, controller_state: ControllerState) {}

    fn drive_to_coordinates(&mut self, controller_state: ControllerState) {}
}

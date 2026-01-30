/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: drive.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Jan 29th 2025 10:30PM          */
/*    Description: Eclipselib smart drivetrain      */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use core::f64::consts::PI;

use vexide::{controller::ControllerState, prelude::*};

use crate::eclipselib;

/// Trait defining the interface for tank drive systems
pub trait TankDrive {
    /// Operator control drive using controller input
    fn opc_drive(&mut self, controller_state: ControllerState);

    /// Drive to a specific distance using PID control
    fn drive_to(&mut self, distance: f64);
}

/// Simple tank drive with basic motor control
pub struct SimpleDrive {
    left_drive: eclipselib::motor::MotorGroup,
    right_drive: eclipselib::motor::MotorGroup,
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    inertial: InertialSensor,
}

/// Advanced tank drive with odometry capabilities
pub struct OdomDrive {
    left_drive: eclipselib::motor::MotorGroup,
    right_drive: eclipselib::motor::MotorGroup,
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    odom: eclipselib::odometry::Odometry,
}

#[allow(unused)]
impl SimpleDrive {
    pub fn new(
        left_drive: eclipselib::motor::MotorGroup,
        right_drive: eclipselib::motor::MotorGroup,
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
        left_drive: eclipselib::motor::MotorGroup,
        right_drive: eclipselib::motor::MotorGroup,
        gear_ratio: f64,
        wheel_size: f64,
        gear_set: Gearset,
        odom: eclipselib::odometry::Odometry,
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
        let _ = self.left_drive.set_voltage((left_y * 12000.0));
        let _ = self.right_drive.set_voltage((right_y * 12000.0));
    }

    fn drive_to(&mut self, distance: f64) {
        // TODO: Implement PID control to drive to distance
        // Convert distance (inches) to encoder ticks based on gear_ratio and wheel_size
    }
}

impl TankDrive for OdomDrive {
    fn opc_drive(&mut self, controller_state: ControllerState) {
        // Get joystick values
        let left_y = controller_state.left_stick.y();
        let right_y = controller_state.right_stick.y();

        // Set motor voltages based on joystick input
        let _ = self.left_drive.set_voltage((left_y * 12000.0));
        let _ = self.right_drive.set_voltage((right_y * 12000.0));

        // Update odometry with current motor positions
        // self.dual_odom.update();
    }

    fn drive_to(&mut self, distance: f64) {
        // TODO: Implement odometry-based PID control to drive to distance
    }
}

pub struct XDrive {
    east_drive: eclipselib::motor::MotorGroup, // Front Left && Back Right
    west_drive: eclipselib::motor::MotorGroup, // Front Right && Back Left
    gear_ratio: f64,
    wheel_size: f64,
    gear_set: Gearset,
    spline: eclipselib::spline::Spline,
}

#[allow(unused)]
impl XDrive {
    fn new(
        east_drive: eclipselib::motor::MotorGroup,
        west_drive: eclipselib::motor::MotorGroup,
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
            spline: eclipselib::spline::spline(0.0, 0.0, 0.0),
        }
    }

    fn opc_drive(&mut self, controller_state: ControllerState) {}

    fn drive_to_coordinates(&mut self, target: eclipselib::spline::Spline) {}
}

/// Converts inches to degrees
pub fn inches_to_degrees(inches: f64, wheel_rad: Option<f64>, external_gear_ratio: f64) -> f64 {
    let wheel_rad = wheel_rad.unwrap_or(3.25); // Defaults to 3.25 in wheels
    let wheel_circumference = 2.0 * PI * wheel_rad;
    let revolutions = (inches / wheel_circumference) * external_gear_ratio;
    revolutions * 360.0
}

/// Converts degrees to inches
pub fn degrees_to_inches(degrees: f64, wheel_rad: Option<f64>, external_gear_ratio: f64) -> f64 {
    let revolutions = degrees / 360.0;
    let wheel_rad = wheel_rad.unwrap_or(3.25); // Defaults to 3.25 in wheels
    let wheel_circumference = 2.0 * PI * wheel_rad;
    (revolutions / external_gear_ratio) * wheel_circumference
}

/// Converts centimeters to degrees
pub fn cm_to_degrees(cm: f64, wheel_rad: Option<f64>, external_gear_ratio: f64) -> f64 {
    let wheel_rad = wheel_rad.unwrap_or(8.255); // Defaults to 8.255 cm wheels
    let wheel_circumference = 2.0 * PI * wheel_rad;
    let revolutions = (cm / wheel_circumference) * external_gear_ratio;
    revolutions * 360.0
}

/// Converts degrees to centimeters
pub fn degrees_to_cm(degrees: f64, wheel_rad: Option<f64>, external_gear_ratio: f64) -> f64 {
    let revolutions = degrees / 360.0;
    let wheel_rad = wheel_rad.unwrap_or(8.255); // Defaults to 8.255 cm wheels
    let wheel_circumference = 2.0 * PI * wheel_rad;
    (revolutions / external_gear_ratio) * wheel_circumference
}

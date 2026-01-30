/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib / swerve           */
/*    File: swervedrive.rs                          */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: swerve drive module              */
/*                                                  */
/* ------------------------------------------------ */
use core::f64::consts::PI;

use vexide::{controller::ControllerState, prelude::*};

use crate::eclipselib;

pub trait SwerveDrive {
    fn position(&mut self) -> &mut eclipselib::spline::Spline;

    fn update_position(&mut self, target_spline: eclipselib::spline::Spline) {
        *self.position() = target_spline;
    }

    fn calculate_approach_angle(&mut self, target_spline: eclipselib::spline::Spline) -> f64 {
        let north_error = target_spline.north() - self.position().north(); // Δy
        let west_error = target_spline.west() - self.position().west(); // Δx
        let angle = north_error.atan2(west_error) * (180.0 / PI); // angle robot must face to go directly toward the target
        angle
    }

    fn opc_drive(&mut self, controller_state: ControllerState) {
        let axis3 = controller_state.left_stick.x();
        let axis4 = controller_state.left_stick.y();
        let axis1 = controller_state.right_stick.x();
    }
}

pub struct DualSwerveDrive {
    left_module: eclipselib::swerve::swervemod::SwerveModule,
    right_module: eclipselib::swerve::swervemod::SwerveModule,
    inertial: InertialSensor,
    position: eclipselib::spline::Spline,
    pid: eclipselib::pid::PIDController,
}

#[allow(unused)]
impl DualSwerveDrive {
    pub fn new(
        left_module: eclipselib::swerve::swervemod::SwerveModule,
        right_module: eclipselib::swerve::swervemod::SwerveModule,
        inertial: InertialSensor,
        pid: eclipselib::pid::PIDController,
    ) -> Self {
        Self {
            left_module,
            right_module,
            inertial,
            position: eclipselib::spline::spline(0.0, 0.0, 0.0),
            pid,
        }
    }

    pub fn drive_to_coordinates(&mut self, target: eclipselib::spline::Spline) {
        let north_error = target.north() - self.position.north(); // Δy
        let west_error = target.west() - self.position.west(); // Δx
        // angle robot must face to go directly toward the target
        let angle = north_error.atan2(west_error) * (180.0 / PI);
        let error = (north_error.powf(2.0) + west_error.powf(2.0)).powf(0.5);
    }
}

impl SwerveDrive for DualSwerveDrive {
    fn position(&mut self) -> &mut eclipselib::spline::Spline {
        &mut self.position
    }
}

pub struct QuadSwerveDrive {
    backleft_module: eclipselib::swerve::swervemod::SwerveModule,
    backright_module: eclipselib::swerve::swervemod::SwerveModule,
    frontleft_module: eclipselib::swerve::swervemod::SwerveModule,
    frontright_module: eclipselib::swerve::swervemod::SwerveModule,
    inertial: InertialSensor,
    position: eclipselib::spline::Spline,
    pid: eclipselib::pid::PIDController,
}

impl QuadSwerveDrive {
    pub fn new(
        backleft_module: eclipselib::swerve::swervemod::SwerveModule,
        backright_module: eclipselib::swerve::swervemod::SwerveModule,
        frontleft_module: eclipselib::swerve::swervemod::SwerveModule,
        frontright_module: eclipselib::swerve::swervemod::SwerveModule,
        inertial: InertialSensor,
        pid: eclipselib::pid::PIDController,
    ) -> Self {
        Self {
            backleft_module,
            backright_module,
            frontleft_module,
            frontright_module,
            inertial,
            position: eclipselib::spline::spline(0.0, 0.0, 0.0),
            pid,
        }
    }

    pub fn drive_to_coordinates(&mut self, target: eclipselib::spline::Spline) {}
}

impl SwerveDrive for QuadSwerveDrive {
    fn position(&mut self) -> &mut eclipselib::spline::Spline {
        &mut self.position
    }
}

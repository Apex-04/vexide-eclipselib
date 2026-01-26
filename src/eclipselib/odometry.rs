/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: odometry.rs                             */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: Eclipselib Odometry              */
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::{
    devices::{smart::imu::InertialError, PortError},
    prelude::*,
};

pub struct Odometry {
    rotation_back: RotationSensor,
    rotation_front: RotationSensor,
    inertial: InertialSensor,
}
#[allow(unused)]
impl Odometry {
    pub fn new(
        rotation_back: RotationSensor,
        rotation_front: RotationSensor,
        inertial: InertialSensor,
    ) -> Self {
        Self {
            rotation_back,
            rotation_front,
            inertial,
        }
    }

    pub fn update_position(&mut self) {
        // Function Body
    }

    pub fn get_heading(&mut self) -> Result<f64, InertialError> {
        self.inertial.heading()
    }
    pub fn get_back_position(&mut self) -> Result<Position, PortError> {
        self.rotation_back.position()
    }
    pub fn get_front_position(&mut self) -> Result<Position, PortError> {
        self.rotation_front.position()
    }
    pub fn reset(&mut self) {
        let _ = self.rotation_back.set_position(Position::from_degrees(0.0));
        let _ = self
            .rotation_front
            .set_position(Position::from_degrees(0.0));
        let _ = self.inertial.set_heading(0.0);
    }
}

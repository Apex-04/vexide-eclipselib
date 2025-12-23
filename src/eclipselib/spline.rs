/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: spline.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Dec 12th 2025 10:30AM          */
/*    Description: swerveib modules                 */
/*                                                  */
/* ------------------------------------------------ */

// Spline Coordinate are based on Red alliance, Right corner
pub struct Spline {
    north: f64, // Torwards blue alliance
    west: f64,  // Torwards Head Ref station
    up: f64,
}
#[allow(unused)]
impl Spline {
    pub fn new(north: f64, west: f64, up: f64) -> Self {
        Self { north, west, up }
    }
    pub fn west(&mut self) -> f64 {
        self.west
    }
    pub fn north(&mut self) -> f64 {
        self.north
    }
}

pub fn spline(north: f64, west: f64, up: f64) -> Spline {
    Spline::new(north, west, up)
}

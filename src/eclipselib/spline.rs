/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: spline.rs                               */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Jan 25th 2025 20:45AM          */
/*    Description: Spline Movement System           */
/*                                                  */
/* ------------------------------------------------ */

// Spline Coordinate are based on Red alliance, Right corner
pub struct Spline {
    north: f64, // Towards blue alliance
    west: f64,  // Towards Head Ref station
    up: f64,    // Vertical position

    // Previous position for velocity/derivative calculation
    previous_north: f64,
    previous_west: f64,
    previous_up: f64,
}

#[allow(unused)]
impl Spline {
    pub fn new(north: f64, west: f64, up: f64) -> Self {
        Self {
            north,
            west,
            up,
            previous_north: north,
            previous_west: west,
            previous_up: up,
        }
    }

    /// Update the spline position and return the change (velocity)
    pub fn update(&mut self, north: f64, west: f64, up: f64) -> (f64, f64, f64) {
        // Calculate change in position
        let delta_north = north - self.north;
        let delta_west = west - self.west;
        let delta_up = up - self.up;

        // Store previous position
        self.previous_north = self.north;
        self.previous_west = self.west;
        self.previous_up = self.up;

        // Update current position
        self.north = north;
        self.west = west;
        self.up = up;

        // Return velocity (change in position)
        (delta_north, delta_west, delta_up)
    }

    /// Get current position
    pub fn position(&self) -> (f64, f64, f64) {
        (self.north, self.west, self.up)
    }

    /// Get previous position
    pub fn previous_position(&self) -> (f64, f64, f64) {
        (self.previous_north, self.previous_west, self.previous_up)
    }

    pub fn west(&self) -> f64 {
        self.west
    }

    pub fn north(&self) -> f64 {
        self.north
    }

    pub fn up(&self) -> f64 {
        self.up
    }
}

pub fn spline(north: f64, west: f64, up: f64) -> Spline {
    Spline::new(north, west, up)
}

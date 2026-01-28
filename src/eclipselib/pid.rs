/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: pid.rs                                  */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Jan 25th 2025 10:30PM          */
/*    Description: pid functions                    */
/*                                                  */
/* ------------------------------------------------ */

/// Calculate kp, ki, and kd based on Ziegler–Nichols tuning rules.
///
/// # Parameters
/// - `ku`: ultimate gain
/// - `tu`: oscillation period
/// - `mode`: controller type (1–7)
///
/// # Modes
/// 1: P controller
/// 2: PI controller
/// 3: PD controller
/// 4: Classic PID
/// 5: Pessen Integral Rule
/// 6: Some Overshoot
/// 7: No Overshoot
pub fn calc_gains(ku: f64, tu: f64, mode: i8) -> (f64, f64, f64) {
    let (mut kp, mut ki, mut kd) = (0.0, 0.0, 0.0);
    match mode {
        // 1: P controller
        1 => {
            kp = 0.5 * ku;
        }
        // 2: PI controller
        2 => {
            kp = 0.45 * ku;
            ki = (0.54 * ku) / tu;
        }
        // 3: PD controller
        3 => {
            kp = 0.8 * ku;
            kd = 0.125 * tu;
        }
        // 4: Classic PID controller
        4 => {
            kp = 0.6 * ku;
            ki = 1.2 * ku / tu;
            kd = 0.075 * ku * tu;
        }
        // 5: Pessen Integral Rule
        5 => {
            kp = 0.7 * ku;
            ki = 1.75 * ku / tu;
            kd = 0.105 * ku * tu;
        }
        // 6: Some Overshoot
        6 => {
            kp = 0.33 * ku;
            ki = 0.66 * ku / tu;
            kd = 0.11 * ku * tu;
        }
        // 7: No Overshoot
        7 => {
            kp = 0.2 * ku;
            ki = 0.4 * ku / tu;
            kd = 0.066 * ku * tu;
        }
        // default / invalid mode
        _ => {
            println!("Invalid mode: must be between 1 and 7.");
        }
    }
    (kp, ki, kd)
}

pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    target: f64,
    integral: f64,
    previous_error: f64,
}

#[allow(unused)]
impl PIDController {
    pub fn set_gains(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            target: 0.0,
            integral: 0.0,
            previous_error: 0.0,
        }
    }

    /// Assigns `target` to the PID object for `.calculate()` to read
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Calculate the output given the current position for one itteration of the loop
    pub fn calculate(&mut self, encoder_value: f64) -> f64 {
        // Calculate error
        let error = self.target - encoder_value;

        // Accumulate integral
        self.integral += error;

        // Calculate derivative
        let derivative = error - self.previous_error;

        // Store error for next iteration
        self.previous_error = error;

        // Calculate PID output
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;

        output
    }

    /// Reset the controller state (integral and previous error)
    pub fn reset(&mut self) {
        self.target = 0.0;
        self.integral = 0.0;
        self.previous_error = 0.0;
    }
}

/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: robot.rs                                */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Dec 12th 2025 10:30AM          */
/*    Description: Robot Constructors               */
/*                                                  */
/* ------------------------------------------------ */

use vexide::prelude::*;

/// How to use robot.rs
///
/// robot.rs is a dedicated file to define robot subsystems.
/// doesn't robot.rs go against the ethos of eclipselib?
/// Yes. eclipselib aims to provide a simple solution that is easily implementable without having to write dedicated functions and subsystems
///
/// HOWEVER: we understand every robot is different, and we figured we'd provide a template for you to create dedicated subsystems.
/// below we included a very stripped down subsystem for a lift.

struct LiftExample {
    motor: Motor,
    encoder: RotationSensor,
}
#[allow(unused)]
impl LiftExample {}

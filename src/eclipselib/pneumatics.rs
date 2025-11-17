/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: pneumatics.rs                           */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Nov 17th 2025 3:00PM           */
/*    Team: BBR1                                    */
/*    Description: Eclipselib pneumnatics           */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;
extern crate alloc;
pub use alloc::vec;

pub struct Solonoid{
    solonoid: AdiDigitalOut
}
impl Solonoid{
    pub fn new(port: AdiPort) -> Self{
        Self{
            solonoid: AdiDigitalOut::new(port)
        }

    }
    // Toggle is not very verbose as it doesnt tell you exactly which state it's in, best practice only use set_high and set_low
    pub fn set_high(&mut self){
        let _ = self.solonoid.set_high();
    }
    pub fn set_low(&mut self){
        let _ = self.solonoid.set_low();
    }
    pub fn toggle(&mut self){
        let _ = self.solonoid.toggle();
    }
}



// Using ADI devices instead of just taking the ports to make it easier on the backend
pub struct SolonoidGroup{
    solonoids: vec::Vec<AdiDigitalOut>
}
impl SolonoidGroup{
    pub fn new2_adi_group(device1: AdiPort, device2: AdiPort) -> Self{
        Self{
            solonoids: vec![AdiDigitalOut::new(device1), AdiDigitalOut::new(device2)]
        }
    }

    // Toggle is not very verbose as it doesnt tell you exactly which state it's in, best practice only use set_high and set_low
    pub fn set_high(&mut self){
        for solonoid in self.solonoids.iter_mut(){
            let _ = solonoid.set_high();
        }
    }
    pub fn set_low(&mut self){
        for solonoid in self.solonoids.iter_mut(){
            let _ = solonoid.set_low();
        }
    }
        pub fn toggle(&mut self){
        for solonoid in self.solonoids.iter_mut(){
            let _ = solonoid.toggle();
        }
    }
}
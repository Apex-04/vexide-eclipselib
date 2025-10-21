/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: pneumatics.rs                           */
/*    Author: Andrew Bobay                          */
/*    Date Created: Oct 21st 2025 11:20AM           */
/*    Date Modified: Oct 21st 2025 11:20AM          */
/*    Team: BBR1                                    */
/*    Description: Eclipselib pneumnatics           */ 
/*                 definitions                      */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec; 
use vexide::devices::smart::imu::InertialError;
use vexide::devices::PortError; 
use vexide::prelude::*;

pub struct Solonoid{
    solonoid: AdiDigitalOut
}
impl Solonoid{
    pub fn new(port: AdiPort) -> Self{
        Self{
            solonoid: AdiDigitalOut::new(port)
        }

    }
    pub fn set_high(&mut self){
        let _ = self.solonoid.set_high();
    }
    pub fn set_low(&mut self){
        let _ = self.solonoid.set_low();
    }
}



// Using ADI devices instead of just taking the ports to make it easier on the backend
pub struct SolonoidGroup{
    adi_devices: vec::Vec<AdiDigitalOut>
}
impl SolonoidGroup{
    pub fn new2_adi_group(device1: AdiPort, device2: AdiPort) -> Self{
        Self{
            adi_devices: vec![AdiDigitalOut::new(device1), AdiDigitalOut::new(device2)]
        }
    }

    // Toggle is not very verbose as ut doesnt tell you exactly which state it's in, best practice only use set_high and set_low
    pub fn set_high(&mut self){
        for adi_device in self.adi_devices.iter_mut(){
            let _ = adi_device.set_high();
        }
    }
    pub fn set_low(&mut self){
        for adi_device in self.adi_devices.iter_mut(){
            let _ = adi_device.set_low();
        }
    }
}
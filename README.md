# Eclipse Lib
eclipselib is library to extend the base features of vexide, that aims to provide higher level  constructors to vexide while also keeping a clear system to design the program for your robot.

## What is Vexide
vexide is a Rust library for programming VEX robots, providing a platform for running rust code on the VEX brain to control your robot serving as an alternative to VEXcode or PROS. vexide benefits from rust's advanced features like task scheduling and memory allocation giving you full control over your robot. 

## Why Eclipse Lib?
Eclipse lib was developed to make Vexide slightly more approchable, vexide by its default is very Do it Yourself. while this allows much more freedom than with Vexcode or Pros. it is a double edged sword. Eclipse lib aims to provide a system of constructs and functions to get started and make using vexide more approachable

## Swerve
In the FIRST Robotics Competition Swerve Modules dominate every other type of drivetrian, they have the power efficency of Tank Drive and the versatility of X-Drive. In VEX Robotics they have not seen much success due to some of the sacrifices you have to make. you can read more about this in our documentation available on github. on the programming side we saw the need of designing a clear structure to program swerve modules. This is all a part of the "swerve" extension to the base eclipselib (src/eclipselib/swerve) 

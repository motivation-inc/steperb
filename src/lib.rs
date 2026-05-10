//! steperb - a stepper motor control library for embedded systems.
//!
//! For examples and references, see the `examples` folder in [steperb’s repository](https://github.com/motivation-inc/steperb/tree/main/examples).

#![no_std]

mod stepper_controller;

pub use stepper_controller::StepperController;

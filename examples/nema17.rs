use steperb::StepperController;

fn main() {
    // 1.8° per step, 90° max range, -90° min range, initialized at 15°
    let mut stepper_motor = StepperController::new(1.8, 90.0, -90.0, 15.0);
    stepper_motor
        .set_desired_angle(45.0)
        .expect("Error setting angle");

    while stepper_motor.needs_movement() {
        if stepper_motor.reversed_direction() {
            println!("Moving in opposite direction.");
        }

        stepper_motor.step();
        println!("{}", stepper_motor.current_angle());
    }
}

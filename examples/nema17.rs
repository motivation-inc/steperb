use steperb::StepperController;

fn main() {
    // 1.8° per step (200 steps per revolution)
    let mut controller = StepperController::new(200);
    controller.set_desired_angle(45.0);

    while controller.needs_movement() {
        if controller.is_reversed() {
            println!("Moving in opposite direction.");
        }

        controller.apply_step();
        println!("{}", controller.current_steps());
    }
}

# `steperb`
`steperb` is a superb stepper motor control library designed for robotics and embedded systems. It is fully `no-std` compatible, and is designed for calculating _what_, _where_, _when_ and _why_ the motor should be doing actions.

```rust
use steperb::StepperController;

fn main() {
    // 1.8° per step (200 steps per revolution), 90° max range, -90° min range, initialized at 15°
    let mut controller = StepperController::new(200);
    controller.set_desired_angle(45.0);

    while controller.needs_movement() {
        if controller.is_reversed() {
            println!("Moving in opposite direction.");
        }

        controller.apply_step(); // one step
        println!("{}", controller.current_steps());
    }
}
```

# Features

- Easy and simple to use
- Safe, idiomatic interfacing (no `unsafe` blocks)
- `no-std` compatible and memory efficient for embedded devices 
- Accurate positioning and calculation logic

## Using `steperb`

`steperb` is explicitly designed to model stepper motors - it doesn't actually implement stepper motor pulse logic. You can model a stepper motor with a `StepperController` object.

```rust
use steperb::StepperController;

fn main() {
    let mut controller = StepperController::new(/* steps per revolution */);
}
```

`StepperController` objects allow you to "control" a stepper motor, modeling steps and angle calculations. To use the controller, set a desired angle.

```rust
// ...
controller.set_desired_angle(45.0).expect("Error setting angle");
```

Increment through each step with motor actions.

```rust
// ... 
while controller.needs_movement() {
    controller.apply_step();

    // stepper motor pulse logic here
}
```

You can also check if the motor should be moving in the opposite direction.

```rust
// ... 
while controller.needs_movement() {
    controller.apply_step();

    if controller.is_reversed() {
        // stepper motor reversing pulse logic here
    }
}
```

## Free & Open-Source

`steperb` is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.

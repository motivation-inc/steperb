/// A stepper motor controller.
pub struct StepperController {
    step_degree: f32,
    max_range: f32,
    min_range: f32,
    target_step_count: i32,
    current_angle: f32,
}

impl StepperController {
    /// Constructs a new stepper controller.
    ///
    /// - `step_degree`: the step degree of the stepper motor (e.g. 1.8 for a Nema 17)
    /// - `max_range`: the maximum range (in degrees) of the stepper motor
    /// - `min_range`: the minimum range (in degrees) of the stepper motor
    /// - `initial_angle`: the initial angle (in degrees) of the stepper motor
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// ```
    pub fn new(step_degree: f32, max_range: f32, min_range: f32, initial_angle: f32) -> Self {
        Self {
            step_degree,
            max_range,
            min_range,
            target_step_count: 0,
            current_angle: initial_angle,
        }
    }

    /// Calculates the step count of the specified angle and sets it as the current target.
    ///
    /// - `angle`: the desired angle
    ///
    /// # Errors
    ///
    /// This function will return an `Err` if:
    /// - `angle` is greater than 360
    /// - `angle` is above the maximum allowed range
    /// - `angle` is below the minimum allowed range
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(45.0).expect("Error setting angle");
    ///
    /// assert_eq!(controller.needs_movement(), true);
    /// ```
    pub fn set_desired_angle(&mut self, angle: f32) -> Result<(), &'static str> {
        self.target_step_count = 0;

        if angle > 360.0 {
            return Err("desired angle cannot be past 360 degrees");
        }

        if angle > self.max_range {
            return Err("desired angle is above the maximum range");
        }

        if angle < self.min_range {
            return Err("desired angle is below the minimum range");
        }

        let steps = (angle / self.step_degree) as i32;

        self.target_step_count = steps;
        self.current_angle = angle;

        Ok(())
    }

    /// Increments the target step count by 1, incrementing the current angle by one step degree.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(45.0).expect("Error setting angle");
    /// controller.increment_step();
    ///
    /// assert_eq!(controller.current_target_steps(), 24);
    /// ```
    pub fn step(&mut self) {
        if self.target_step_count < 0 {
            self.target_step_count += 1;
            self.current_angle += self.step_degree // one step
        } else if self.target_step_count > 0 {
            self.target_step_count -= 1;
            self.current_angle -= self.step_degree
        }
    }

    /// Returns the step degree of the stepper controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// assert_eq!(controller.step_degree(), 1.8);
    /// ```
    pub fn step_degree(&self) -> f32 {
        self.step_degree
    }

    /// Returns the maximum range (in degrees) of the stepper controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// assert_eq!(controller.max_range(), 90.0);
    /// ```
    pub fn max_range(&self) -> f32 {
        self.max_range
    }

    /// Returns the minimum range (in degrees) of the stepper controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// assert_eq!(controller.min_range(), -90.0);
    /// ```
    pub fn min_range(&self) -> f32 {
        self.min_range
    }

    /// Returns if the direction is reversed (the desired angle is negative)
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(-45.0).expect("Error setting angle");
    ///
    /// assert_eq!(controller.reversed_direction(), true);
    /// ```
    pub fn reversed_direction(&self) -> bool {
        self.target_step_count < 0
    }

    /// Returns the requirement for movement (if the target step count is not 0)
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(45.0).expect("Error setting angle");
    ///
    /// assert_eq!(controller.needs_movement(), true);
    /// ```
    pub fn needs_movement(&self) -> bool {
        self.target_step_count != 0
    }

    /// Returns the current angle of the stepper motor controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(1.8, 90.0, -90.0, 45.0);
    /// assert_eq!(controller.current_angle(), 45.0);
    /// ```
    pub fn current_angle(&self) -> f32 {
        self.current_angle
    }

    /// Returns the current target step count of the stepper motor controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(45.0).expect("Error setting angle");
    ///
    /// assert_eq!(controller.current_target_steps(), 25);
    /// ```
    pub fn current_target_steps(&self) -> i32 {
        self.target_step_count
    }

    /// Resets the stepper controller's desired angle to 0.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(1.8, 90.0, -90.0, 0.0);
    /// controller.set_desired_angle(45.0).expect("Error setting angle");
    ///
    /// controller.reset();
    ///
    /// assert_eq!(controller.current_angle(), 0.0);
    /// assert_eq!(controller.current_target_steps(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.target_step_count = 0;
        self.current_angle = 0.0;
    }
}

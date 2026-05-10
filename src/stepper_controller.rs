/// A stepper motor controller.
pub struct StepperController {
    steps_per_revolution: u32,
    current_steps: i32,
    target_steps: i32,
}

impl StepperController {
    /// Constructs a new stepper controller.
    ///
    /// - `steps_per_revolution`: the steps per full revolution of the stepper motor (e.g. 200 for a Nema 17)
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(200);
    /// ```
    pub fn new(steps_per_revolution: u32) -> Self {
        Self {
            steps_per_revolution,
            current_steps: 0,
            target_steps: 0,
        }
    }

    /// Calculates the step count of the specified angle and sets it as the current target.
    ///
    /// - `angle`: the desired angle
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    /// controller.set_desired_angle(45.0);
    ///
    /// assert_eq!(controller.current_steps(), 25);
    /// ```
    pub fn set_desired_angle(&mut self, angle: f32) {
        let resolution = 360.0 / self.steps_per_revolution as f32;
        let steps = (angle / resolution) as i32;

        self.target_steps = steps;
        self.current_steps = steps;
    }

    /// Increments the target step count by 1, incrementing the current angle by one step degree.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    /// controller.set_desired_angle(45.0);
    /// controller.apply_step();
    ///
    /// assert_eq!(controller.current_steps(), 24);
    /// ```
    pub fn apply_step(&mut self) {
        if self.current_steps < 0 {
            self.current_steps += 1 // one step
        } else if self.current_steps > 0 {
            self.current_steps -= 1
        }

        if self.current_steps == 0 {
            self.target_steps = 0;
        }
    }

    /// Returns the steps per revolution of the stepper controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let controller = StepperController::new(200);
    /// assert_eq!(controller.steps_per_revolution(), 200);
    /// ```
    pub fn steps_per_revolution(&self) -> u32 {
        self.steps_per_revolution
    }

    /// Returns if the direction is reversed (the desired steps are negative)
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    /// controller.set_desired_angle(-45.0);
    ///
    /// assert_eq!(controller.is_reversed(), true);
    /// ```
    pub fn is_reversed(&self) -> bool {
        self.target_steps < 0
    }

    /// Returns the requirement for movement (if the target step count is not 0)
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    /// controller.set_desired_angle(45.0);
    ///
    /// assert_eq!(controller.needs_movement(), true);
    /// ```
    pub fn needs_movement(&self) -> bool {
        self.current_steps != 0
    }

    /// Returns the current target step count of the stepper motor controller.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    ///
    /// assert_eq!(controller.current_steps(), 0);
    /// ```
    pub fn current_steps(&self) -> i32 {
        self.current_steps
    }

    /// Resets the stepper controller's target step count to 0.
    ///
    /// # Example
    ///
    /// ```
    /// use steperb::StepperController;
    ///
    /// let mut controller = StepperController::new(200);
    /// controller.set_desired_angle(45.0);
    ///
    /// controller.reset();
    ///
    /// assert_eq!(controller.current_steps(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.target_steps = 0;
        self.current_steps = 0;
    }
}

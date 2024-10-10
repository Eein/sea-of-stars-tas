pub trait ControllerInterface {
  fn request_control(&self) -> bool;
  fn release(&self);
  fn execute(&self);
}


struct GameManager<'a> {
  current_controller: &'a dyn ControllerInterface,
  release_requested_by: Option<&'a dyn ControllerInterface>,
  controllers: Vec<&'a dyn ControllerInterface>
}

impl GameManager<'_> {
  // Executes the main game manager loop
  pub fn execute(&mut self) {
    // Checks non-active controllers if they think they
    // should have control.
    // TODO(eein): 
    // 1. how to handle contention?
    // 2. how to skip current controller
    for controller in &self.controllers {
      if controller.request_control() {
        if self.release_requested_by.is_none() {
          self.release_requested_by = Some(*controller);
        } else {
          panic!("Should not contend with another controller for control")
        }
      }
    }

    // Check if the current controller is being asked
    // to be released, and start the cleanup process;
    // then swap to the requested controller
    if self.release_requested_by.is_some() {
      self.current_controller.release();
      self.swap_to_requested();
    }

    // Execute the existing controllers action
    self.current_controller.execute();
  }

  // Swaps the current controller for the controller 
  // asking for release and resets the release request.
  fn swap_to_requested(&mut self) {
    if let Some(released) = &self.release_requested_by {
      self.current_controller = *released;
      self.release_requested_by = None;
    }
  }
}



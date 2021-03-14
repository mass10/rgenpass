///
/// Time keeper
///
/// Manage termination of the application.
///
pub struct TimeKeeper {
	/// Start time
	start: Option<std::time::Instant>,
}

impl TimeKeeper {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance of TimeKeeper
	pub fn new() -> TimeKeeper {
		return TimeKeeper { start: None };
	}

	/// Start keeper
	pub fn start(&mut self) {
		self.start = Some(std::time::Instant::now());
	}

	/// Check whether time keeper is started or not.
	///
	/// ### Returns
	/// `true` if started.
	fn started(&mut self) -> bool {
		return self.start.is_some();
	}

	/// Check the elapsed time.
	///
	/// ### Returns
	/// `true` when it's over.
	pub fn is_timed_out(&mut self) -> bool {
		if !self.started() {
			// Not started.
			return false;
		}

		let current_time = std::time::Instant::now();
		let erapsed = current_time - self.start.unwrap();
		if erapsed.as_millis() < 700 {
			// valid
			return false;
		}

		// It's over
		return true;
	}
}

///
/// Complexity controller
///
pub struct ComplexityController {
	/// Start time
	start: std::time::Instant,
	/// Complexity
	complexity: u8,
}

impl ComplexityController {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance of [ComplexityTimeKeeper]
	pub fn new() -> ComplexityController {
		return ComplexityController { start: std::time::Instant::now(), complexity: 0 };
	}

	/// Increment innternal value.
	fn increment(&mut self) {
		// prevent MAX overflow.
		self.complexity = std::cmp::max(self.complexity, self.complexity + 1);
	}

	/// Decrement innternal value.
	fn decrement(&mut self) {
		// prevent MIN overflow.
		self.complexity = std::cmp::min(self.complexity, self.complexity - 1);
	}

	/// Refresh complexity.
	///
	/// ### Returns
	/// The current complexity.
	pub fn refresh(&mut self) -> u8 {
		// elapsed time
		let current_time = std::time::Instant::now();
		let erapsed = current_time - self.start;

		// Reset internal timeer.
		self.start = current_time;

		if erapsed.as_millis() < 180 {
			// Up
			self.increment();
			return self.complexity;
		}

		if erapsed.as_millis() < 250 {
			// No updates.
			return self.complexity;
		}

		// Down
		self.decrement();
		return self.complexity;
	}
}

#[test]
pub fn test1() {
	let mut complexity_controller = ComplexityController::new();
	let value = complexity_controller.refresh();
	assert!(1 == value, "現在の値が0でないこと");
}

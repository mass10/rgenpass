///
/// Time keeper
///
/// Manage termination of the application.
///
struct TimeKeeper {
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
	pub fn refresh(&mut self) {
		self.start = Some(std::time::Instant::now());
	}

	/// Check whether time keeper is started or not.
	///
	/// ### Returns
	/// `true` if started.
	fn is_started(&self) -> bool {
		return self.start.is_some();
	}

	/// Check the elapsed time.
	///
	/// ### Returns
	/// `true` when it's over.
	pub fn is_timed_out(&self) -> bool {
		if !self.is_started() {
			// Not started. (Maybe no keys are pressed yet.)
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
/// Keyboard queue.
///
pub struct KeyboardQueue {
	time_keeper: TimeKeeper,
}

impl KeyboardQueue {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance.
	pub fn new() -> KeyboardQueue {
		// Internal time keeper
		return KeyboardQueue { time_keeper: TimeKeeper::new() };
	}

	/// Detects key press.
	///
	/// ### Returns
	/// Pressed key identifier if exists. No more keys will be returned when timed out.
	pub fn pop(&mut self) -> Result<Option<crossterm::event::Event>, Box<dyn std::error::Error>> {
		loop {
			// Handle termination.
			if self.time_keeper.is_timed_out() {
				// shutdown
				return Ok(None);
			}

			// Detect some key pressed.
			let result = crossterm::event::poll(std::time::Duration::from_millis(5))?;
			if !result {
				continue;
			}

			// Detect key press.
			let key = crossterm::event::read()?;

			// Once any key pressed, time keeper refreshes.
			self.time_keeper.refresh();

			return Ok(Some(key));
		}
	}
}

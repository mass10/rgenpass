///
/// Time keeper
///
pub struct TimeKeeper {
	start: Option<std::time::Instant>,
	times: u32,
}

impl TimeKeeper {
	pub fn new() -> TimeKeeper {
		return TimeKeeper { start: None, times: 0 };
	}

	pub fn start(&mut self) {
		self.start = Some(std::time::Instant::now());
	}

	/// Check time keeper started or not.
	///
	///
	fn started(&mut self) -> bool {
		return self.start.is_some();
	}

	/// Check elapsed.
	///
	/// # Returns
	/// true when time over.
	pub fn is_over(&mut self) -> bool {
		if !self.started() {
			// Not started.
			return false;
		}

		self.times += 1;
		let current_time = std::time::Instant::now();
		let erapsed = current_time - self.start.unwrap();
		if erapsed.as_millis() < 400 {
			return false;
		}
		return true;
	}
}

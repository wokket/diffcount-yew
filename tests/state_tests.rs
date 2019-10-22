//we have to use wasm_bindgen_tests, because the compiled app isn't a valid win32 app for normal cargo tests.
#![allow(unused_variables)]
fn main() {
	extern crate wasm_bindgen_test;
	use diffcount::state::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn clear_resets_value() {
		let mut s = State {
			alarm_count: 10,
			total: 5,
		};

		assert_eq!(s.total, 5, "State failed to init correctly");
		s.on_cleared();
		assert_eq!(s.total, 0, "State failed to clear total correctly");
	}

	#[wasm_bindgen_test]
	fn increment_increases_total() {
		let mut s = State {
			alarm_count: 10,
			total: 5,
		};

		assert_eq!(s.total, 5, "State failed to init correctly");
		s.on_incremented();
		assert_eq!(s.total, 6, "State failed to inc total correctly");
	}

	#[wasm_bindgen_test]
	fn alarm_fires_when_total_matches() {
		let mut s = State {
			alarm_count: 10,
			total: 9,
		};

		assert_eq!(s.total, 9, "State failed to init correctly");
		assert_eq!(s.is_alarm_triggered(), false, "Alarm fired early?");
		s.on_incremented();
		assert_eq!(s.total, 10, "State failed to inc total correctly");
		assert_eq!(s.is_alarm_triggered(), true, "Alarm failed to fire");
		s.on_incremented();
		assert_eq!(s.total, 11, "State failed to init correctly");
		assert_eq!(s.is_alarm_triggered(), false, "Alarm failed to reset");
	}
}

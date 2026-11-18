#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use wasm::Instant;

#[cfg(target_arch = "wasm32")]
mod wasm {
    /// A wall-clock reading in milliseconds since the Unix epoch, from
    /// JavaScript's `Date.now()` (there is no monotonic `std::time::Instant`
    /// on wasm32-unknown-unknown).
    #[derive(Debug, Clone, Copy)]
    pub struct Instant {
        epoch_millis: f64,
    }

    impl Instant {
        pub fn now() -> Self {
            Instant {
                epoch_millis: js_sys::Date::now(),
            }
        }

        /// The time from `earlier` to `self`.
        ///
        /// # Panics
        /// When `earlier` is later than `self` (the wall clock went back).
        pub fn duration_since(&self, earlier: Instant) -> std::time::Duration {
            let millis = self.epoch_millis - earlier.epoch_millis;
            assert!(millis >= 0.0, "Duration is negative");
            std::time::Duration::from_secs_f64(millis / 1000.0)
        }
    }
}

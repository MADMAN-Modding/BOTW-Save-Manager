use std::sync::{Arc, Mutex};

// Adds necessary traits to the ThreadData structure
#[derive(Clone, Copy, Debug)]

/// Used for sharing data between the `http_server` thread and the thread the app is running on
pub struct ThreadData {
    /// `stop` - For telling the thread to stop itself
    pub(crate) stop: bool,
}

impl ThreadData {
    /// Returns `ThreadData { stop: false}` so the server doesn't stop on start
    pub fn setup() -> ThreadData {
        Self { stop: false }
    }

    /// Accessor function for `stop`
    pub fn get_stop(&self) -> bool {
        self.stop
    }

    /// Setter for changing `stop`
    pub fn set_stop(&mut self, stop_value: bool) {
        self.stop = stop_value;
    }
}

/// Returns an Arc-Mutex-ThreadData object
pub fn thread_data_setup() -> Arc<Mutex<ThreadData>> {
    return Arc::new(Mutex::new(ThreadData::setup()))
}
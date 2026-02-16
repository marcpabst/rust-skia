use crate::graphite::{types::BackendApi, Recording};
use crate::prelude::*;
use skia_bindings as sb;
use std::fmt;

pub type Recorder = RefHandle<sb::skgpu_graphite_Recorder>;
unsafe_send_sync!(Recorder);

impl NativeDrop for sb::skgpu_graphite_Recorder {
    fn drop(&mut self) {
        unsafe { sb::C_Recorder_delete(self) }
    }
}

impl fmt::Debug for Recorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder")
            .field("backend", &self.backend())
            .finish()
    }
}

impl Recorder {
    pub fn snap(&mut self) -> Option<Recording> {
        let recording_ptr = unsafe { sb::C_Recorder_snap(self.native_mut()) };
        if recording_ptr.is_null() {
            None
        } else {
            Recording::from_ptr(recording_ptr)
        }
    }

    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_Recorder_backend(self.native()) }
    }
}

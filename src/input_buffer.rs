use crate::encoder::Encoder;
use crate::NV_ENC_INPUT_PTR;

/// An NVENC input buffer for submitting raw frames.
pub struct InputBuffer {
    pub(crate) encoder: Encoder,
    pub(crate) buffer: NV_ENC_INPUT_PTR,
}

impl Drop for InputBuffer {
    fn drop(&mut self) {
        if let Err(e) = self.encoder.destroy_input_buffer(self.buffer) {
            tracing::warn!("[nvenc] destroy_input_buffer failed in Drop: {e:?}");
        }
    }
}

/// RAII guard representing a locked input buffer ready for writing pixel data.
pub struct InputBufferLock<'a> {
    pub(crate) input_buffer: &'a InputBuffer,
    pub(crate) lock_ptr: NV_ENC_INPUT_PTR,
}

impl Drop for InputBufferLock<'_> {
    fn drop(&mut self) {
        if let Err(e) = self
            .input_buffer
            .encoder
            .unlock_input_buffer(self.lock_ptr)
        {
            tracing::warn!("[nvenc] unlock_input_buffer failed in Drop: {e:?}");
        }
    }
}

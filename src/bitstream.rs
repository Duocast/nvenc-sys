use crate::encoder::Encoder;
use crate::NV_ENC_OUTPUT_PTR;

/// An NVENC output bitstream buffer for receiving encoded data.
pub struct BitStream {
    pub(crate) encoder: Encoder,
    pub(crate) buffer: NV_ENC_OUTPUT_PTR,
}

impl Drop for BitStream {
    fn drop(&mut self) {
        if let Err(e) = self.encoder.destroy_bitstream_buffer(self.buffer) {
            tracing::warn!("[nvenc] destroy_bitstream_buffer failed in Drop: {e:?}");
        }
    }
}

/// RAII guard representing a locked bitstream buffer for reading encoded output.
pub struct BitStreamLockGuard<'a> {
    pub(crate) buffer: &'a BitStream,
}

impl<'a> Drop for BitStreamLockGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self
            .buffer
            .encoder
            .unlock_bit_stream_buffer(self.buffer.buffer)
        {
            tracing::warn!("[nvenc] unlock_bit_stream_buffer failed in Drop: {e:?}");
        }
    }
}

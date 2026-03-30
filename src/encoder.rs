use std::sync::Arc;

use crate::{
    NV_ENCODE_API_FUNCTION_LIST, NV_ENC_INPUT_PTR, NV_ENC_OUTPUT_PTR, NV_ENC_REGISTERED_PTR,
    NVENCSTATUS,
};

/// Wrapper around the NVENC encoder session and its function list.
#[derive(Clone)]
pub struct Encoder {
    pub(crate) ptr: *mut std::os::raw::c_void,
    pub(crate) fns: Arc<NV_ENCODE_API_FUNCTION_LIST>,
}

unsafe impl Send for Encoder {}
unsafe impl Sync for Encoder {}

impl Encoder {
    /// Destroy an input buffer previously created with `create_input_buffer`.
    pub fn destroy_input_buffer(
        &self,
        buffer: NV_ENC_INPUT_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self.fns.nvEncDestroyInputBuffer.expect("nvEncDestroyInputBuffer not loaded");
        let status = unsafe { func(self.ptr, buffer) };
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unlock an input buffer previously locked with `lock_input_buffer`.
    pub fn unlock_input_buffer(
        &self,
        buffer: NV_ENC_INPUT_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self.fns.nvEncUnlockInputBuffer.expect("nvEncUnlockInputBuffer not loaded");
        let status = unsafe { func(self.ptr, buffer) };
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Destroy a bitstream buffer previously created with `create_bitstream_buffer`.
    pub fn destroy_bitstream_buffer(
        &self,
        buffer: NV_ENC_OUTPUT_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self
            .fns
            .nvEncDestroyBitstreamBuffer
            .expect("nvEncDestroyBitstreamBuffer not loaded");
        let status = unsafe { func(self.ptr, buffer) };
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unlock a bitstream buffer previously locked with `lock_bitstream`.
    pub fn unlock_bit_stream_buffer(
        &self,
        buffer: NV_ENC_OUTPUT_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self.fns.nvEncUnlockBitstream.expect("nvEncUnlockBitstream not loaded");
        let status = unsafe { func(self.ptr, buffer) };
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unmap a previously mapped input resource.
    pub unsafe fn unmap_input_resource(
        &self,
        mapped: NV_ENC_INPUT_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self
            .fns
            .nvEncUnmapInputResource
            .expect("nvEncUnmapInputResource not loaded");
        let status = func(self.ptr, mapped);
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregister a previously registered resource.
    pub unsafe fn unregister_resource(
        &self,
        registered: NV_ENC_REGISTERED_PTR,
    ) -> Result<(), NVENCSTATUS> {
        let func = self
            .fns
            .nvEncUnregisterResource
            .expect("nvEncUnregisterResource not loaded");
        let status = func(self.ptr, registered);
        if status == NVENCSTATUS::NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// A registered and mapped external resource (e.g. a CUDA or D3D texture).
pub struct RegisteredResource {
    pub(crate) encoder: Encoder,
    pub(crate) mapped: NV_ENC_INPUT_PTR,
    pub(crate) registered: NV_ENC_REGISTERED_PTR,
}

impl Drop for RegisteredResource {
    fn drop(&mut self) {
        if let Err(e) = unsafe { self.encoder.unmap_input_resource(self.mapped) } {
            tracing::warn!("[nvenc] unmap_input_resource failed in Drop: {e:?}");
        }
        if let Err(e) = unsafe { self.encoder.unregister_resource(self.registered) } {
            tracing::warn!("[nvenc] unregister_resource failed in Drop: {e:?}");
        }
    }
}

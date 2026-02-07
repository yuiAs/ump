//! COM initialization and D2D factory creation.
#![cfg(feature = "d2d")]

use windows::Win32::Graphics::Direct2D::{
    D2D1CreateFactory, D2D1_FACTORY_TYPE_SINGLE_THREADED, ID2D1Factory,
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

use crate::renderer::RenderError;

/// Initialize COM for the current thread.
/// Ignores RPC_E_CHANGED_MODE (0x80010106) — COM already initialized by winit.
pub fn init_com() -> Result<(), RenderError> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        // S_OK, S_FALSE (already initialized same mode), or RPC_E_CHANGED_MODE are all acceptable
        if hr.is_err() && hr.0 != 0x80010106u32 as i32 {
            hr.ok().map_err(|e| {
                RenderError::PlatformError(format!("CoInitializeEx failed: {}", e))
            })?;
        }
    }
    Ok(())
}

/// Create a single-threaded D2D1 factory.
pub fn create_d2d_factory() -> Result<ID2D1Factory, RenderError> {
    unsafe {
        D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).map_err(|e| {
            RenderError::PlatformError(format!("D2D1CreateFactory failed: {}", e))
        })
    }
}

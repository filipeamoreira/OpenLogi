//! macOS Input Monitoring (TCC) status for the HID++ transport.
//!
//! `openlogi-hid` opens Logitech HID nodes through `IOHIDManager` (via
//! `async-hid`), which macOS gates behind the Input Monitoring privacy
//! permission — without it, every `IOHIDDeviceOpen` is silently denied and no
//! HID++ device ever appears, with no error surfaced beyond a debug log.
//!
//! Checking never registers the app in System Settings; [`request_access`]
//! records the request, and it must run in this process (the agent), not the
//! GUI; a TCC grant is scoped to the code-signing identity that asks for it,
//! and the agent (not the GUI) is the one that actually opens HID devices.

use std::cfg_select;

#[cfg(target_os = "macos")]
mod macos {
    use objc2_core_graphics::CGRequestListenEventAccess;
    use objc2_io_kit::{IOHIDAccessType, IOHIDCheckAccess, IOHIDRequestType};

    pub(super) fn has_access() -> bool {
        matches!(
            IOHIDCheckAccess(IOHIDRequestType::ListenEvent),
            IOHIDAccessType::Granted
        )
    }

    pub(super) fn request_access() -> bool {
        // `IOHIDRequestAccess` no longer prompts for ListenEvent on macOS 26.
        // Core Graphics exposes the supported user-consent request for the
        // same Input Monitoring TCC service.
        CGRequestListenEventAccess()
    }
}

/// Whether this process currently holds Input Monitoring access.
///
/// Always `true` off macOS, where HID access has no privacy gate.
#[must_use]
pub fn has_access() -> bool {
    cfg_select! {
        target_os = "macos" => { macos::has_access() }
        _ => { true }
    }
}

/// Register this process with macOS as requesting Input Monitoring access.
///
/// macOS may require the user to add the app manually under System Settings →
/// Privacy & Security → Input Monitoring. Run this off the async runtime in
/// case the OS blocks while handling the request. Returns whether access was
/// granted. Always returns `true` off macOS.
#[must_use]
pub fn request_access() -> bool {
    cfg_select! {
        target_os = "macos" => { macos::request_access() }
        _ => { true }
    }
}

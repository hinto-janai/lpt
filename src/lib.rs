#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[inline]
#[cfg(target_os = "windows")]
pub fn lpt() -> Result<(), ()> {
	use windows::Win32::System::Threading::*;

	// SAFETY: calling C.
	unsafe {
		match SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_IDLE) {
			Ok(_)  => Ok(()),
			Err(_) => Err(()),
		}
	}
}

#[inline]
#[cfg(target_family = "unix")]
pub fn lpt() -> Result<(), ()> {
	const NICE_MAX: libc::c_int = if cfg!(target_os = "linux") { 19 } else { 20 };

	// SAFETY: calling C.
	unsafe {
		if libc::nice(NICE_MAX) != -1 {
			Ok(())
		} else {
			Err(())
		}
	}
}
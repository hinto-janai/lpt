#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[inline]
#[cfg(target_os = "windows")]
/// Set the calling thread to the lowest possible priority
///
/// This function will never fail.
/// ```
pub fn lpt() {
	use windows::Win32::System::Threading::*;

	// SAFETY: calling C.
	//
	// We are _lowering_ our priority, not increasing, so this function should never fail.
	unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_IDLE) };
}

#[inline]
#[cfg(target_family = "unix")]
/// Set the calling thread to the lowest possible priority
///
/// This function will never fail.
/// ```
pub fn lpt() {
	const NICE_MAX: libc::c_int = if cfg!(target_os = "linux") { 19 } else { 20 };

	// SAFETY: calling C.
	//
	// We are _lowering_ our priority, not increasing, so this function should never fail.
	unsafe { libc::nice(NICE_MAX) };
}

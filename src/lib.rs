#![deny(
	dead_code,
	unused_imports,
	unused_must_use,
	unused_variables,
	unused_mut
)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(
	clippy::as_conversions,
	clippy::dbg_macro,
	clippy::float_cmp_const,
	clippy::lossy_float_literal,
	clippy::string_to_string,
	clippy::unneeded_field_pattern,
	clippy::verbose_file_reads,
	clippy::unwrap_used,
	clippy::panic,
	clippy::needless_update,
	clippy::match_like_matches_macro,
	clippy::from_over_into,
	clippy::useless_conversion
)]

use std::alloc::System;
use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

///
pub struct TrackingAlloc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

///
pub fn get_allocated() -> usize {
	ALLOCATED.load(Ordering::Relaxed)
}

unsafe impl GlobalAlloc for TrackingAlloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let ret = System.alloc(layout);
		if !ret.is_null() {
			ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
		}
		ret
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		System.dealloc(ptr, layout);
		ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
	}
}

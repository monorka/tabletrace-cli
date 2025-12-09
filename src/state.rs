//! Global state management module

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;

/// Whether connection has been lost
pub static CONNECTION_LOST: AtomicBool = AtomicBool::new(false);

/// Whether currently selecting tables
pub static SELECTING_TABLES: AtomicBool = AtomicBool::new(false);

/// Change counter
pub static CHANGE_COUNT: AtomicUsize = AtomicUsize::new(0);

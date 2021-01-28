#![deny(unconditional_recursion)]
#![feature(const_fn_transmute)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod prelude;

mod audio_session_types;
pub use audio_session_types::*;

mod core_audio_base_types;
pub use core_audio_base_types::*;

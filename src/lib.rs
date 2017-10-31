//! This library provides a driver for the HD44780 family of LCD displays.
//!
//! These are traditionally 1 or 2-lines 16-columns character displays.
//!
//! This crate provides the logic for such a display,
//! while being generic on the actual IO.
//!
//! In order to use this, you will need to implement some device-specific IO
//! traits, over which the [`driver`](driver/struct.Driver.html) struct is
//! generic.
//!
//! More specifically, you will need:
//!
//! * An output pin trait: `gpio_traits::pin::Output`, that implements
//!   `high(&mut self)` and `low(&mut self)`.
//! * A `PinGroup` trait composed of 5 or 9 pins: a "flush" pin,
//!   and 4 or 8 data pins.
//!
//!   `PinGroup` is implemented for groups of `Output` pins, but you
//!   can also decide to use something like a pin multiplexer instead.
//! * A sleep function that sleeps the given number of microseconds.
#![no_std]

extern crate gpio_traits;

mod gpio;
mod driver;
pub mod commands;

pub use driver::{Driver, PinDriver};
pub use gpio::{PinGroup, Sleep};

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(doc)]
use bmp388::*;

pub mod mars_climate_orbiter;
#[cfg(feature = "std")]
pub mod type_driven_101;

/// BMP388 driver crate that utilises a generic for using either [`Async`] or [`Blocking`] modes of the API
/// and a [`ConfigBuilder`] for setting up the sensor.
///
/// 1. [`Async`] & [`Blocking`] modes of the API.
/// A public trait [`Mode`] is exported which requires a trait bound to a private one.
/// In a nutshell, only the crate's developer can create such structs
/// but anyone can use them form the public API.
///
/// 2. Configuration builder struct for the setting up the BMP388 on initialisation.
/// Utilises the [`typed_builder`] derive macro to generate a Type-state builder struct.
///
/// # Examples
///
/// `embassy-rp::spi::Spi`: <https://docs.rs/embassy-rp/latest/embassy_rp/spi/struct.Spi.html>
/// 
/// Where `Blocking` implements only the `embedded_hal` trait while `Async` implements both `embedded_hal` and `embedded_hal_async`
pub mod bmp388 {
    pub use bmp388::{Async, Blocking, ConfigBuilder, Mode, BMP388};
}

pub mod bms;
pub mod typenum_multiplier;

/// Your generic structures for sharing between apps.
/// - For Radio communication it could be Ground software to Spacecraft,
/// - For host to Spacecraft, communication between spacecraft subsystems, e.g. USB packets
pub mod shared {
    /// Device configuration used for host-to-device communication, use on device (`no_std`)
    ///
    /// `#[cfg_attr(feature = "my-protocol", derive(DeriveMyProtocol))]`
    ///
    /// `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`
    ///
    /// `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
    pub struct DeviceConfiguration {}

    impl DeviceConfiguration {
        pub fn to_radio(&self, buf: &mut [u8]) -> Result<(), ()> {
            todo!("for no_std you could add the serialized radio protocol data to the buffer, e.g. `postcard`")
        }
    }
}

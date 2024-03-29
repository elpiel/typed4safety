//! The Battery management system (BMS)

use core::{
    marker::PhantomData,
    ops::{self, Mul},
};

use num::{rational::Ratio, ToPrimitive};

pub const V5V: Voltage5v = Voltage5v {};
pub struct Divider<const R1: usize, const R2: usize>;

pub struct CBattery<const CUT_OFF_MV: usize, const CHARGED_MV: usize>;

// Voltage mode - 1.1V, 3.3V, 5V, etc.
#[allow(private_bounds)]
pub trait VMode: mode::Sealed {}

/// Pin that uses 3.3V for it's pins
/// E.g. RP Pico, ESP, etc.
pub struct Voltage3v3 {}
/// Pin that uses 5V for it's pins
/// E.g. Arduino
pub struct Voltage5v {}

impl mode::Sealed for Voltage3v3 {}
impl VMode for Voltage3v3 {}

impl mode::Sealed for Voltage5v {}
impl VMode for Voltage5v {}

mod mode {
    pub(super) trait Sealed {}
}

/// GPIO pin
/// E.g. <https://docs.rs/esp-hal/latest/esp_hal/gpio/struct.GpioPin.html>
pub struct Pin<const PIN: u8, VMODE: VMode>(PhantomData<VMODE>);

pub struct MeasureBattery<VMODE: VMode> {
    pin: Pin<5, VMODE>,
    divider: VoltageDivider<500_000, 300_000>,
}

impl MeasureBattery<Voltage3v3> {
    /// Convert a measured mV on the GPIO in mV through the given voltage divider
    /// # Returns
    /// mV
    pub fn checked_voltage(&self) -> Option<usize> {
        todo!()
    }
}

// pub trait ToBatteryV {
//     fn to_battery_voltage<const R1: usize, const R2: usize, const CHARGED_MV: usize>(&self);
// }

// pub fn from_mv() ->

// impl ToBatteryV for usize {
//     fn to_battery_voltage<const R1: usize, const R2: usize, const CHARGED_MV: usize>(&self) {
//         VoltageDivider<>
//     }
// }

/// R2 is towards the positive (+) side
/// R1 is towards the GND
///
/// <https://ohmslawcalculator.com/voltage-divider-calculator>
///
/// Not very useful in Application code unless you want to measure the resistors individually on each board
/// and bake it in the firmware using a type.
/// Could be improved with [`typenum`] alternative for runtime initialisation
// - [`TVoltageDivider`]
#[derive(Debug, Clone, Copy)]
pub struct VoltageDivider<const R1: usize, const R2: usize>;

impl<const R1: usize, const R2: usize> VoltageDivider<R1, R2> {
    pub fn ratio(&self) -> Ratio<usize> {
        Ratio::new(R2, R1 + R2)
    }

    /// The voltage divider value
    pub fn divider(&self) -> f32 {
        if R2 == 0 {
            return 1.0;
        }
        let r1_f32 = R1 as f32;
        let r2_f32 = R2 as f32;

        r2_f32 / (r1_f32 + r2_f32)
    }
}

impl<const R1: usize, const R2: usize> ops::Div<VoltageDivider<R1, R2>> for usize {
    type Output = usize;

    fn div(self, rhs: VoltageDivider<R1, R2>) -> Self::Output {
        Ratio::new(self, 1)
            .mul(rhs.ratio())
            .round()
            .to_usize()
            .unwrap()
    }
}

// /// [`typenum`] version of the const generics [`VoltageDivider`]
// pub struct TVoltageDivider<R1, R2> {
//     r1: R1,
//     r2: R2,
// }

// impl<const R1: usize, const R2: usize> ops::Div<VoltageDivider<R1, R2>> for f32 {
//     type Output = f32;

//     fn div(self, rhs: VoltageDivider<R1, R2>) -> Self::Output {
//         let divider = rhs.ratio().to_f32().unwrap();
//         Ratio::new(self, divider).to_f32()
//     }
// }
#[cfg(test)]
mod tests {
    use super::VoltageDivider;

    #[test]
    fn test_usize_voltage_divider() {
        // 4.2 Li-ion battery at 100% (4.2V)
        // with voltage divider on the 3.3V ADC input of the pin
        {
            let divider = VoltageDivider::<470_000, 470_000>;
            // in mV
            let battery_voltage = 4_200;
            assert_eq!(2_100, battery_voltage / divider);
        }

        // divider = R2 / R1 + R2
        // divider = 300 kOhm / 300 + 500 kOhm = 0.375
        {
            let divider = VoltageDivider::<500_000, 300_000>;
            // in mV
            let battery_voltage = 3_200;
            assert_eq!(1_200, battery_voltage / divider);
        }
    }
}

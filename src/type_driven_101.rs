use core::{num::NonZeroU64, str::FromStr};
use std::fmt::Display;

#[allow(unused_variables)]
pub fn primitive_types() -> Result<(), &'static str> {
    let always_positive: i64 = 50;
    let instead_use: u64 = 50;

    let non_zero = NonZeroU64::new(50).ok_or("Zero provided!")?;

    Ok(())
}

pub struct SpacecraftName(String);
impl FromStr for SpacecraftName {
    type Err = &'static str;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('_') {
            return Err("Spacecraft name should start with `_`");
        }

        Ok(Self(s.to_string()))
    }
}

impl Display for SpacecraftName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

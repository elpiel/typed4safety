
pub const MULTIPLIER: usize = typed4safety::typenum_multiplier::compile_time_mult::<typenum::U3, typenum::U4>();

fn main() {
    #[cfg(feature = "uom")]
    let _test_mars_climate_orbiter =
        typed4safety::mars_climate_orbiter::test_f64_uom_jpl_spacecraft_vs_lockheed_ground_software(
        );

    #[cfg(feature = "multiplier")]
    println!("3 * 4 = {}", MULTIPLIER);
}

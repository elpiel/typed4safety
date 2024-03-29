//! > The problem is what the spacecraft was sending back to Earth.
//! > It was Newton seconds and the software in the ground station was reading those results as pound seconds!
//! > It then reported those results to the guidance and navigation teams and was off by a factor of 4.45 times.
//! - Everyday Astronaut

use uom::si::{
    self, length::meter, mass::kilogram, momentum::kilogram_meter_per_second, time::second,
};

/// # Spacecraft - JPL used the readings as newton-second and Ground software - Lockheed Martin Astronautics read those values as pound-second for impulse
///
/// Wikipedia: <https://en.wikipedia.org/wiki/Newton-second>
///
///
/// # Space Shuttle launched from Earth to orbit
///
/// > Space Shuttle weight here includes the heaviest possible payload (27500 kg), empty external tank (30000 kg), and the shuttle itself (75000 kg) all in a low Earth orbit (8.05 km/s). As the Space Shuttle uses staging, not all launched components reach all the way to orbit (e.g. the boosters). The total impulse gained by all stages together during the launch is 5.7×10^9 Ns.
/// >
/// > Wikipedia
///
/// Speed: 8 050 m/s
///
/// Mass: 132 500 kg
///
/// Impulse (N⋅s): 1.07×10^9
///
/// # Newton-second
/// > It is dimensionally equivalent to the momentum unit kilogram-metre per second (kg⋅m/s)
/// >
/// > Wikipedia
#[cfg(feature = "std")]
pub fn test_f64_uom_jpl_spacecraft_vs_lockheed_ground_software() {
    use uom::si::f64::{Force, Length, Mass, Momentum, Time};

    let _spacecraft = {
        let length = Length::new::<meter>(8050.0);
        let time = Time::new::<second>(1.0);

        // Velocity for launching to orbit
        let speed = length / time;
        //let error = length + time; // error[E0308]: mismatched types

        // Mass of the Space Shuttle*
        let mass = Mass::new::<kilogram>(132_500.0);

        // Momentum (N⋅s)
        let spacecraft_impulse: Momentum = mass * speed;
        let metric = spacecraft_impulse.get::<kilogram_meter_per_second>();
        println!(
            "\n\nMomentum / the impulse of Space Shuttle (kilogram-metres per second: {} kg⋅m/s (or N⋅s)",
            metric,
        );
        // Momentum (N⋅s) = 1.07 * 10^9 [Ns]
        assert_eq!(1_066_625_000.0, metric);

        metric
    };

    // Ground was reading pound-second
    // same example with floats
    let (_actual_gs, _shown_gs) = {
        let time = Time::new::<si::time::second>(1.0);

        let force_actual = Force::new::<si::force::pound_force>(239_786_838.9337);
        let groundstation_impulse_actual = force_actual * time;
        let imperial_actual = groundstation_impulse_actual.get::<kilogram_meter_per_second>();

        let force_shown = Force::new::<si::force::pound_force>(1_066_625_000.0);
        let groundstation_impulse_shown = force_shown * time;
        let imperial_shown = groundstation_impulse_shown.get::<kilogram_meter_per_second>();

        println!("\n\nImperial pound-second");
        println!("Actual: {imperial_actual} kg⋅m/s (or N⋅s)");
        println!("Shown in GS: {imperial_shown} kg⋅m/s (or N⋅s)");
        println!("Difference of x{}", imperial_shown / imperial_actual);

        (imperial_actual, imperial_shown)
    };
}

#[cfg(feature = "std")]
pub fn test_u64_uom_jpl_spacecraft_vs_lockheed_ground_software() {
    use uom::si::u64;

    let spacecraft = {
        let length = u64::Length::new::<meter>(8050);
        let time = u64::Time::new::<second>(1);

        // Velocity for launching to orbit
        let speed = length / time;
        // Mass of the Space Shuttle*
        let mass = u64::Mass::new::<kilogram>(132_500);

        // Momentum (N⋅s)
        let spacecraft_impulse: u64::Momentum = mass * speed;
        let metric = spacecraft_impulse.get::<kilogram_meter_per_second>();
        println!(
        "\n\nMomentum / the impulse of Space Shuttle (kilogram-metres per second: {} kg⋅m/s (or N⋅s)",
        metric,
    );
        // Momentum (N⋅s) = 1.07 * 10^9 [Ns]
        assert_eq!(1_066_625_000, metric);

        metric
    };

    let (_actual_gs, _shown_gs) = {
        let time = u64::Time::new::<si::time::second>(1);

        let force_actual = u64::Force::new::<si::force::pound_force>(239_691_011);
        let groundstation_impulse_actual = force_actual * time;
        let imperial_actual = groundstation_impulse_actual.get::<kilogram_meter_per_second>();

        let force_shown = u64::Force::new::<si::force::pound_force>(1_066_625_000);
        let groundstation_impulse_shown = force_shown * time;
        let imperial_shown = groundstation_impulse_shown.get::<kilogram_meter_per_second>();

        println!("\n\nImperial pound-second");
        println!("Actual: {imperial_actual}");
        println!("Shown in GS: {imperial_shown}");
        println!(
            "Difference of x{:.3}",
            imperial_shown as f64 / imperial_actual as f64
        );

        // Momentum (Pound-force second) 2.396_91 × 10^8 [lbf·s]
        (imperial_actual, imperial_shown)
    };
}

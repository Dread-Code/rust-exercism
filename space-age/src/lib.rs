#[derive(Debug)]
pub struct Duration {
    time: f64, // Represents the time in Earth years
}

// Earth seconds in one Earth year
const EARTH_SECONDS: u64 = 31_557_600;

impl From<u64> for Duration {
    /// Converts a duration from seconds to a Duration object in Earth years.
    fn from(s: u64) -> Self {
        let time = s as f64 / EARTH_SECONDS as f64; // Convert seconds to Earth years
        Duration { time }
    }
}

/// Macro to implement the Planet trait for multiple structs with their orbital periods.
/// This macro reduces repetitive boilerplate for each planet.
///
/// IMPORTANT: The second arm of this macro is key to supporting multiple structs in a single invocation.
macro_rules! implement_years_during {
    // First arm: Match a single struct and its orbital period
    ($struct_name: ident, $number: expr) => {
        impl Planet for $struct_name {
            /// Calculates the number of "planet years" for the given Duration.
            fn years_during(d: &Duration) -> f64 {
                d.time / $number // Divides Earth's years by the planet's orbital period
            }
        }
    };

    // Second arm: Match multiple structs and their orbital periods
    ($($struct_name:ident, $number:expr);* $(;)?) => {
        // Explanation of the second arm:
        // - `$()` captures a repeating pattern. Inside `$()`, the pattern specifies how each item should look.
        // - `$struct_name:ident` matches an identifier (like `Mercury` or `Venus`).
        // - `$number:expr` matches an expression (like `0.2408467`).
        // - The pattern is repeated for each planet, separated by semicolons (`;`).
        // - The `*` means "match this pattern zero or more times."
        // - The `$(;)?` matches an optional trailing semicolon after the list.

        $(
            // For each captured struct and its number, invoke the first arm of the macro.
            // This generates the `impl Planet` block for that struct using its orbital period.
            implement_years_during!($struct_name, $number);
        )*
    };
}

/// The Planet trait defines the behavior of calculating planetary years.
pub trait Planet {
    /// Takes a reference to a Duration and returns the equivalent planet years.
    fn years_during(d: &Duration) -> f64;
}

// Define structs for each planet
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

// Use the macro to implement the Planet trait for all planets with their orbital periods.
// IMPORTANT: Here's where the macro is used to implement Planet for multiple structs at once.
implement_years_during! {
    Mercury, 0.2408467;  // Orbital period in Earth years
    Venus, 0.61519726;
    Earth, 1.0;
    Mars, 1.8808158;
    Jupiter, 11.862615;
    Saturn, 29.447498;
    Uranus, 84.016846;
    Neptune, 164.79132;
}

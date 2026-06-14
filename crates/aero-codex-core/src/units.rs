use crate::{validation, AeroResult};

/// Angle stored internally in radians.
///
/// Phase 0.001 keeps angles as a lightweight typed wrapper because many gas-
/// dynamics and flight-dynamics equations mix degrees in published examples
/// with radians in trigonometric implementations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    /// Zero radians.
    pub const ZERO: Self = Self(0.0);

    /// Creates an angle from radians.
    #[must_use]
    pub const fn from_radians(radians: f64) -> Self {
        Self(radians)
    }

    /// Creates an angle from degrees.
    #[must_use]
    pub fn from_degrees(degrees: f64) -> Self {
        Self(degrees.to_radians())
    }

    /// Returns the stored angle in radians.
    #[must_use]
    pub const fn as_radians(self) -> f64 {
        self.0
    }

    /// Returns the stored angle in degrees.
    #[must_use]
    pub fn as_degrees(self) -> f64 {
        self.0.to_degrees()
    }

    /// Sine of the angle.
    #[must_use]
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// Cosine of the angle.
    #[must_use]
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    /// Tangent of the angle.
    #[must_use]
    pub fn tan(self) -> f64 {
        self.0.tan()
    }
}

/// Dimensionless Mach number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mach(f64);

impl Mach {
    /// Creates a Mach number from a finite value with `M >= 0`.
    pub fn new(value: f64) -> AeroResult<Self> {
        validation::ensure_nonnegative("mach", value)?;
        Ok(Self(value))
    }

    /// Returns the dimensionless Mach number.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Ratio of specific heats, `gamma = c_p / c_v`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gamma(f64);

impl Gamma {
    /// Creates a specific-heat ratio from a finite value with `gamma > 1`.
    pub fn new(value: f64) -> AeroResult<Self> {
        validation::ensure_greater_than("gamma", value, 1.0)?;
        Ok(Self(value))
    }

    /// Returns the dimensionless specific-heat ratio.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

macro_rules! nonnegative_scalar {
    ($(#[$meta:meta])* $name:ident, $ctor:ident, $getter:ident, $parameter:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl $name {
            /// Creates the scalar from a finite nonnegative SI value.
            pub fn $ctor(value: f64) -> AeroResult<Self> {
                validation::ensure_nonnegative($parameter, value)?;
                Ok(Self(value))
            }

            /// Returns the scalar in its canonical SI unit.
            #[must_use]
            pub const fn $getter(self) -> f64 {
                self.0
            }
        }
    };
}

macro_rules! signed_scalar {
    ($(#[$meta:meta])* $name:ident, $ctor:ident, $getter:ident, $checked_ctor:ident, $parameter:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl $name {
            /// Creates the scalar from its canonical SI value.
            ///
            /// Use the `try_*` constructor for untrusted input that must be
            /// checked for finite values before storage.
            #[must_use]
            pub const fn $ctor(value: f64) -> Self {
                Self(value)
            }

            /// Creates the scalar from a finite SI value.
            pub fn $checked_ctor(value: f64) -> AeroResult<Self> {
                validation::ensure_finite($parameter, value)?;
                Ok(Self(value))
            }

            /// Returns the scalar in its canonical SI unit.
            #[must_use]
            pub const fn $getter(self) -> f64 {
                self.0
            }
        }
    };
}

nonnegative_scalar!(
    /// Absolute pressure stored in pascals.
    Pressure,
    from_pascal,
    as_pascal,
    "pressure"
);
nonnegative_scalar!(
    /// Absolute temperature stored in kelvin.
    Temperature,
    from_kelvin,
    as_kelvin,
    "temperature"
);
nonnegative_scalar!(
    /// Mass density stored in kilograms per cubic metre.
    Density,
    from_kg_per_m3,
    as_kg_per_m3,
    "density"
);
nonnegative_scalar!(
    /// Length stored in metres.
    Length,
    from_meter,
    as_meter,
    "length"
);
nonnegative_scalar!(
    /// Area stored in square metres.
    Area,
    from_square_meter,
    as_square_meter,
    "area"
);
nonnegative_scalar!(
    /// Mass stored in kilograms.
    Mass,
    from_kg,
    as_kg,
    "mass"
);
nonnegative_scalar!(
    /// Time interval stored in seconds.
    Time,
    from_second,
    as_second,
    "time"
);
nonnegative_scalar!(
    /// Speed stored in metres per second.
    Velocity,
    from_meter_per_second,
    as_meter_per_second,
    "velocity"
);
nonnegative_scalar!(
    /// Acceleration magnitude stored in metres per second squared.
    Acceleration,
    from_meter_per_second_squared,
    as_meter_per_second_squared,
    "acceleration"
);

signed_scalar!(
    /// Signed force stored in newtons.
    Force,
    from_newton,
    as_newton,
    try_from_newton,
    "force"
);
signed_scalar!(
    /// Signed heat flux stored in watts per square metre.
    HeatFlux,
    from_watt_per_square_meter,
    as_watt_per_square_meter,
    try_from_watt_per_square_meter,
    "heat_flux"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_radian_conversion_round_trips() {
        let angle = Angle::from_degrees(180.0);
        assert!((angle.as_radians() - std::f64::consts::PI).abs() < 1.0e-12);
        assert!(
            (Angle::from_radians(std::f64::consts::PI / 2.0).as_degrees() - 90.0).abs() < 1.0e-12
        );
    }

    #[test]
    fn angle_trigonometry_uses_radians_internally() {
        let right_angle = Angle::from_degrees(90.0);
        assert!(right_angle.sin() > 0.999_999_999_999);
        assert!(right_angle.cos().abs() < 1.0e-12);
        assert_eq!(Angle::ZERO.tan(), 0.0);
    }

    #[test]
    fn invalid_gamma_rejected() {
        assert!(Gamma::new(1.0).is_err());
        assert!(Gamma::new(f64::NAN).is_err());
    }

    #[test]
    fn negative_pressure_rejected() {
        assert!(Pressure::from_pascal(-1.0).is_err());
    }

    #[test]
    fn mach_zero_accepted() {
        assert_eq!(Mach::new(0.0).unwrap().value(), 0.0);
    }

    #[test]
    fn mach_negative_and_nonfinite_values_are_rejected() {
        assert!(Mach::new(-0.01).is_err());
        assert!(Mach::new(f64::INFINITY).is_err());
    }

    #[test]
    fn nonnegative_scalars_accept_zero_and_report_canonical_si_units() {
        assert_eq!(Pressure::from_pascal(0.0).unwrap().as_pascal(), 0.0);
        assert_eq!(Temperature::from_kelvin(0.0).unwrap().as_kelvin(), 0.0);
        assert_eq!(Density::from_kg_per_m3(0.0).unwrap().as_kg_per_m3(), 0.0);
        assert_eq!(Length::from_meter(0.0).unwrap().as_meter(), 0.0);
        assert_eq!(Area::from_square_meter(0.0).unwrap().as_square_meter(), 0.0);
        assert_eq!(Mass::from_kg(0.0).unwrap().as_kg(), 0.0);
        assert_eq!(Time::from_second(0.0).unwrap().as_second(), 0.0);
        assert_eq!(
            Velocity::from_meter_per_second(0.0)
                .unwrap()
                .as_meter_per_second(),
            0.0
        );
        assert_eq!(
            Acceleration::from_meter_per_second_squared(0.0)
                .unwrap()
                .as_meter_per_second_squared(),
            0.0
        );
    }

    #[test]
    fn nonnegative_scalars_reject_negative_and_nonfinite_inputs() {
        assert!(Temperature::from_kelvin(-1.0).is_err());
        assert!(Density::from_kg_per_m3(-1.0).is_err());
        assert!(Length::from_meter(-1.0).is_err());
        assert!(Area::from_square_meter(-1.0).is_err());
        assert!(Mass::from_kg(-1.0).is_err());
        assert!(Time::from_second(-1.0).is_err());
        assert!(Velocity::from_meter_per_second(-1.0).is_err());
        assert!(Acceleration::from_meter_per_second_squared(-1.0).is_err());
        assert!(Pressure::from_pascal(f64::NAN).is_err());
    }

    #[test]
    fn signed_scalars_preserve_sign_and_can_check_finite_inputs() {
        assert_eq!(Force::from_newton(-12.0).as_newton(), -12.0);
        assert_eq!(
            HeatFlux::from_watt_per_square_meter(-3.0).as_watt_per_square_meter(),
            -3.0
        );
        assert_eq!(Force::try_from_newton(12.0).unwrap().as_newton(), 12.0);
        assert!(Force::try_from_newton(f64::NAN).is_err());
        assert!(HeatFlux::try_from_watt_per_square_meter(f64::INFINITY).is_err());
    }
}

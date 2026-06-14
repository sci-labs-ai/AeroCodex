use crate::{AeroError, AeroResult};

pub fn ensure_finite(parameter: &'static str, value: f64) -> AeroResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AeroError::OutOfDomain {
            parameter,
            value,
            expected: "finite value",
        })
    }
}

pub fn ensure_positive(parameter: &'static str, value: f64) -> AeroResult<()> {
    ensure_finite(parameter, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(AeroError::NonPositiveInput { parameter, value })
    }
}

pub fn ensure_nonnegative(parameter: &'static str, value: f64) -> AeroResult<()> {
    ensure_finite(parameter, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(AeroError::NegativeInput { parameter, value })
    }
}

pub fn ensure_greater_than(parameter: &'static str, value: f64, lower: f64) -> AeroResult<()> {
    ensure_finite(parameter, value)?;
    if value > lower {
        Ok(())
    } else {
        Err(AeroError::OutOfDomain {
            parameter,
            value,
            expected: "strictly greater than documented lower bound",
        })
    }
}

pub fn ensure_range(
    parameter: &'static str,
    value: f64,
    min: f64,
    max: f64,
    expected: &'static str,
) -> AeroResult<()> {
    ensure_finite(parameter, value)?;
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(AeroError::OutOfDomain {
            parameter,
            value,
            expected,
        })
    }
}

pub fn require_supersonic(mach: f64) -> AeroResult<()> {
    ensure_finite("mach", mach)?;
    if mach >= 1.0 {
        Ok(())
    } else {
        Err(AeroError::RequiresSupersonic { mach, minimum: 1.0 })
    }
}

pub fn require_strictly_supersonic(mach: f64) -> AeroResult<()> {
    ensure_finite("mach", mach)?;
    if mach > 1.0 {
        Ok(())
    } else {
        Err(AeroError::RequiresSupersonic { mach, minimum: 1.0 })
    }
}

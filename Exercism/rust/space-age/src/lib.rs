// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SEC: f64 = 31_557_600.;

#[derive(Debug)]
pub struct Duration{
    sec: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{sec: s as f64}
    }
}

pub trait Planet {
    const EARTH_TO_PLANET: f64;

    fn years_during(d: &Duration) -> f64{
        d.sec as f64 / (EARTH_YEAR_IN_SEC*Self::EARTH_TO_PLANET)
    }
}

macro_rules! impl_Planet {
    ($(($p:ident, $r:literal)), *) =>{
        $(
            pub struct $p;

            impl Planet for $p {
                const EARTH_TO_PLANET: f64 = $r;
            }
        )*
    };
}

impl_Planet!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);

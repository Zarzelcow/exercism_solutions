use duplicate::duplicate_item;
#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600f64
    }
}
//   - Mercury: orbital period 0.2408467 Earth years
//    - Venus: orbital period 0.61519726 Earth years
//    - Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
//    - Mars: orbital period 1.8808158 Earth years
//    - Jupiter: orbital period 11.862615 Earth years
//    - Saturn: orbital period 29.447498 Earth years
//    - Uranus: orbital period 84.016846 Earth years
//    - Neptune: orbital period 164.79132 Earth years

#[duplicate_item(
planet_type  orbital_period;
[ Mercury ]     [ 0.2408467 ];
[ Venus ]       [ 0.61519726 ];
[ Mars ]        [ 1.8808158 ];
[ Jupiter ]     [ 11.862615 ];
[ Saturn ]      [ 29.447498 ];
[ Uranus ]      [ 84.016846 ];
[ Neptune ]     [ 164.79132 ];
)]
impl Planet for planet_type {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / orbital_period
    }
}

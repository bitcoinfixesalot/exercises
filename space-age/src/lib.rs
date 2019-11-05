#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_YEAR_ON_EARTH / Self::EARTH_YEARS
    }
}

const SECONDS_IN_YEAR_ON_EARTH: f64 = 31557600.0;

macro_rules! planet_trait_impl {
    ($($type:ident => $x:expr,)+) => (planet_trait_impl!{$($type => $x),+});
    ($($type:ident => $x:expr),*) => ($(
        pub struct $type;
        impl Planet for $type {
            const EARTH_YEARS: f64 = $x;
        }
    )*)
}

planet_trait_impl! {
    Mercury => 0.240_846_7,
    Venus => 0.615_197_26,
    Earth => 1.0,
    Mars => 1.880_815_8,
    Jupiter => 11.862_615,
    Saturn => 29.447_498,
    Uranus => 84.016_846,
    Neptune => 164.791_32,
}

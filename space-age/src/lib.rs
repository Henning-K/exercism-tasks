// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

static EARTH_ORBIT_PERIOD: f64 = 31_557_600.0;

pub struct Duration {
    secs: usize,
}

impl Duration {
    pub fn new(d: usize) -> Self {
        Duration { secs: d }
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration::new(s as usize)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / EARTH_ORBIT_PERIOD
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

impl Planet for Earth {}
impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 0.2408467)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 0.61519726)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 1.8808158)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        (d.secs as f64) / (EARTH_ORBIT_PERIOD * 164.79132)
    }
}

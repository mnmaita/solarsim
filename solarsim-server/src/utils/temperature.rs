pub trait TemperatureExt {
    fn as_kelvin(&self) -> f32;
    fn as_celsius(&self) -> f32;
}

impl TemperatureExt for f32 {
    fn as_kelvin(&self) -> f32 {
        self + 273.15
    }

    fn as_celsius(&self) -> f32 {
        self - 273.15
    }
}

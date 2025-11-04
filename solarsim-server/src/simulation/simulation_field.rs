use bevy::{
    prelude::{Deref, DerefMut},
    reflect::{Reflect, ReflectDeserialize, ReflectSerialize},
};
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
#[reflect(Serialize, Deserialize)]
pub enum SimulationFieldKind {
    /// This field is editable via a Slider
    Slider,
    /// This field is read-only, it will only be mutated by the simulation.
    #[default]
    ReadOnly,
}

#[derive(Reflect, Serialize, Deserialize, Deref, DerefMut)]
#[reflect(Serialize, Deserialize)]
pub struct SimulationField {
    kind: SimulationFieldKind,
    max: f32,
    min: f32,
    #[deref]
    value: f32,
}

impl SimulationField {
    pub fn new(value: f32, min: f32, max: f32, kind: SimulationFieldKind) -> Self {
        Self {
            kind,
            max,
            min,
            value: value.clamp(min, max),
        }
    }

    pub fn new_percentile(value: f32) -> Self {
        Self {
            kind: SimulationFieldKind::Slider,
            max: 1.0,
            min: 0.0,
            value,
        }
    }

    pub fn kind(&self) -> SimulationFieldKind {
        self.kind
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn set_max(&mut self, max: f32) {
        self.max = max;
    }
}

impl core::fmt::Display for SimulationField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}", self.value)
    }
}

impl Default for SimulationField {
    fn default() -> Self {
        Self {
            kind: SimulationFieldKind::ReadOnly,
            max: 1.0,
            min: 0.0,
            value: 0.0,
        }
    }
}

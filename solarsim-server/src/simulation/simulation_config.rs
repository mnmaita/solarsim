use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{SimulationField, SimulationFieldKind};

#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Ambient temperature, measured in °C
    pub(super) ambient_temp: SimulationField,
    /// Cloud percentile factor (0.0–1.0)
    pub(super) cloud_factor: SimulationField,
    /// Insulation efficiency factor (0.0-1.0), affects [SimulationConfig::overall_heat_loss_coefficient]
    pub(super) insulation_efficiency: SimulationField,
    /// Water leak rate factor (0.0-1.0)
    pub(super) leak_rate: SimulationField,
    /// Solar panel area, measured in m²
    pub(super) panel_area: SimulationField,
    /// Solar panel efficiency factor "η" (0.0-1.0)
    pub(super) panel_efficiency: SimulationField,
    /// Panel heat loss coefficient, measured in W/(m²·K)
    pub(super) panel_heat_loss_coefficient: SimulationField,
    /// Solar panel loss area, measured in m²
    pub(super) panel_loss_area: SimulationField,
    /// Solar irradiance (energy/area), measured in W/m²
    pub(super) solar_irradiance: SimulationField,
    /// Average water tank temperature, measured in °C
    pub(super) tank_average_temp: SimulationField,
    /// Water tank heat loss coefficient, measured in W/(m²·K)
    pub(super) tank_heat_loss_coefficient: SimulationField,
    /// Water tank volume (capacity), measured in m³
    pub(super) tank_volume: SimulationField,
    /// Volumetric pump flow rate, measured in m³/s
    pub(super) pump_flow_rate: SimulationField,
    /// Water temperature entering panel, measured in °C
    pub(super) water_temp_in: SimulationField,
    /// Water temperature leaving panel, measured in °C
    pub(super) water_temp_out: SimulationField,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ambient_temp: SimulationField::new(25.0, -88.0, 58.0, SimulationFieldKind::Slider),
            cloud_factor: SimulationField::new_percentile(0.2),
            panel_heat_loss_coefficient: SimulationField::new(
                0.0,
                4.0,
                19.0,
                SimulationFieldKind::Slider,
            ),
            insulation_efficiency: SimulationField::new_percentile(1.0), // TODO: Use me
            leak_rate: SimulationField::new_percentile(0.0),             // TODO: Use me
            panel_area: SimulationField::new(2.0, 1.0, 3.0, SimulationFieldKind::Slider),
            panel_efficiency: SimulationField::new_percentile(0.8),
            panel_loss_area: SimulationField::new(0.1, 0.0, 3.0, SimulationFieldKind::Slider),
            solar_irradiance: SimulationField::new(800.0, 0.0, 1365.4, SimulationFieldKind::Slider),
            tank_average_temp: SimulationField::new(
                25.0,
                10.0,
                60.0,
                SimulationFieldKind::ReadOnly,
            ),
            tank_heat_loss_coefficient: SimulationField::new(
                0.0,
                3.231,
                20.0,
                SimulationFieldKind::Slider,
            ),
            tank_volume: SimulationField::new(0.5, 0.5, 10_000.0, SimulationFieldKind::Slider),
            pump_flow_rate: SimulationField::new(0.05, 0.05, 0.5, SimulationFieldKind::Slider),
            water_temp_in: SimulationField::new(25.0, 10.0, 60.0, SimulationFieldKind::ReadOnly),
            water_temp_out: SimulationField::new(25.0, 10.0, 60.0, SimulationFieldKind::ReadOnly),
        }
    }
}

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{SimulationField, SimulationFieldKind};

#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Ambient temperature, measured in °C
    pub(super) ambient_temp: SimulationField,
    /// Mass flow rate of water drawn from the storage tank by the user (kg/s)
    pub(super) load_mass_flow_rate: SimulationField,
    /// Temperature of incoming water, measured in °C
    pub(super) load_temp: SimulationField,
    /// Solar panel area, measured in m²
    pub(super) panel_area: SimulationField,
    /// Solar panel efficiency factor "η" (0.0-1.0)
    pub(super) panel_efficiency: SimulationField,
    /// Panel heat loss coefficient, measured in W/(m²·K)
    pub(super) panel_heat_loss_coefficient: SimulationField,
    /// Solar panel loss area, measured in m²
    pub(super) panel_loss_area: SimulationField,
    /// Outer surface area of the pipe, measured in m²
    pub(super) pipe_outer_surface_area: SimulationField,
    /// Overall heat transfer coefficient for pipe insulation, measured in W/(m²·K)
    pub(super) pipe_overall_heat_transfer_coefficient: SimulationField,
    /// Solar irradiance (energy/area), measured in W/m²
    pub(super) solar_irradiance: SimulationField,
    /// Average tank temperature, measured in °C
    pub(super) tank_average_temp: SimulationField,
    /// Tank heat loss coefficient, measured in W/(m²·K)
    pub(super) tank_heat_loss_coefficient: SimulationField,
    /// Tank height to diameter ratio to define its shape.
    pub(super) tank_height_diameter_ratio: SimulationField,
    /// Tank surface area, measured in m²
    pub(super) tank_surface_area: SimulationField,
    /// Amount of water in the tank, measured in Kg
    pub(super) tank_water_mass: SimulationField,
    /// Volumetric pump flow rate, measured in m³/s
    pub(super) pump_flow_rate: SimulationField,
    /// Water temperature entering panel, measured in °C
    pub(super) water_temp_in: SimulationField,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ambient_temp: SimulationField::new(25.0, -88.0, 58.0, SimulationFieldKind::Slider),
            panel_heat_loss_coefficient: SimulationField::new(
                0.0,
                4.0,
                19.0,
                SimulationFieldKind::Slider,
            ),
            load_mass_flow_rate: SimulationField::new(0.1, 0.0, 10.0, SimulationFieldKind::Slider),
            load_temp: SimulationField::new(20.0, 10.0, 60.0, SimulationFieldKind::Slider),
            panel_area: SimulationField::new(2.0, 1.0, 3.0, SimulationFieldKind::Slider),
            panel_efficiency: SimulationField::new_percentile(0.25),
            panel_loss_area: SimulationField::new(0.1, 0.0, 3.0, SimulationFieldKind::Slider),
            pipe_outer_surface_area: SimulationField::new(
                3.0,
                0.5,
                12.0,
                SimulationFieldKind::Slider,
            ),
            pipe_overall_heat_transfer_coefficient: SimulationField::new(
                0.03,
                0.02,
                0.07,
                SimulationFieldKind::Slider,
            ),
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
            tank_height_diameter_ratio: SimulationField::new(
                2.0,
                1.0,
                3.0,
                SimulationFieldKind::Slider,
            ),
            tank_surface_area: SimulationField::new(5.0, 1.0, 20.0, SimulationFieldKind::Slider),
            tank_water_mass: SimulationField::new(100.0, 0.0, 2000.0, SimulationFieldKind::Slider),
            pump_flow_rate: SimulationField::new(0.05, 0.05, 0.5, SimulationFieldKind::Slider),
            water_temp_in: SimulationField::new(25.0, 10.0, 60.0, SimulationFieldKind::ReadOnly),
        }
    }
}

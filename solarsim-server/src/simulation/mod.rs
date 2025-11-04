use core::f32::consts::PI;

use bevy::prelude::*;

mod simulation_config;
mod simulation_field;

pub use simulation_config::*;
pub use simulation_field::*;

/// Constant approximation of water heat capacity (J/kg·K)
const WATER_HEAT_CAPACITY: f32 = 4181.0;
/// Liquid water density (kg/m³);
const WATER_DENSITY: f32 = 1000.0;

pub fn plugin(app: &mut App) {
    app.register_type::<SimulationConfig>()
        .init_resource::<SimulationConfig>();

    app.add_systems(
        FixedUpdate,
        (update_tank_geometry.before(run_simulation), run_simulation)
            .run_if(resource_exists::<SimulationConfig>),
    );
}

fn run_simulation(time: Res<Time<Fixed>>, mut cfg: ResMut<SimulationConfig>) {
    let dt = time.delta_secs();
    let load_mdot = *cfg.load_mass_flow_rate;
    let tank_mass = *cfg.tank_water_mass;
    let tank_temp = *cfg.tank_average_temp;
    let ambient_temp = *cfg.ambient_temp;

    let cp_water = WATER_HEAT_CAPACITY;
    // Solar heat input to panel.
    let q_solar = *cfg.solar_irradiance * *cfg.panel_area * *cfg.panel_efficiency;
    // Panel heat loss.
    let q_panel_loss =
        *cfg.panel_heat_loss_coefficient * *cfg.panel_loss_area * (tank_temp - ambient_temp);
    // Net panel heating rate.
    let q_panel_net = q_solar - q_panel_loss;
    // Pipe heat loss.
    let q_pipe_loss = *cfg.pipe_overall_heat_transfer_coefficient
        * *cfg.pipe_outer_surface_area
        * (tank_temp - ambient_temp);
    // Tank heat loss.
    let q_tank_loss =
        *cfg.tank_heat_loss_coefficient * *cfg.tank_surface_area * (tank_temp - ambient_temp);
    // Net heat going into the tank.
    let q_net = q_panel_net - q_pipe_loss - q_tank_loss;

    let mut new_tank_temp = tank_temp;

    if tank_mass > 0.0 {
        // Temperature change due to panel and losses.
        let delta_temp = q_net * dt / (tank_mass * cp_water);
        // Temperature change due to water draw.
        let delta_temp_load = load_mdot * (tank_temp - *cfg.load_temp) * dt / tank_mass;

        new_tank_temp += delta_temp;
        new_tank_temp -= delta_temp_load;
    }

    // Update tank average temperature.
    *cfg.tank_average_temp = new_tank_temp;
    // Update water temperature entering the panel.
    // We assume it mixes and averages with the tank temperature.
    *cfg.water_temp_in = new_tank_temp;
}

fn update_tank_geometry(mut cfg: ResMut<SimulationConfig>) {
    let k = *cfg.tank_height_diameter_ratio;
    let radius = (*cfg.tank_surface_area / (2.0 * PI * (1.0 + 2.0 * k))).sqrt();

    let volume_if_full = 2.0 * PI * k * radius.powi(3);
    let mass_if_full = volume_if_full * WATER_DENSITY;

    cfg.tank_water_mass.set_max(mass_if_full);

    if *cfg.tank_water_mass > cfg.tank_water_mass.max() {
        *cfg.tank_water_mass = cfg.tank_water_mass.max();
    }
}

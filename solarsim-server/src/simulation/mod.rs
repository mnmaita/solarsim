use bevy::prelude::*;

mod simulation_config;
mod simulation_field;

pub use simulation_config::*;
pub use simulation_field::*;

pub fn plugin(app: &mut App) {
    app.register_type::<SimulationConfig>()
        .init_resource::<SimulationConfig>();

    app.add_systems(
        FixedUpdate,
        run_simulation.run_if(resource_exists::<SimulationConfig>),
    );
}

fn run_simulation(time: Res<Time<Fixed>>, mut cfg: ResMut<SimulationConfig>) {
    /// Density of water "ρ" (rho), measured in kg/m³
    const WATER_DENSITY: f32 = 1000.0;
    let dt = time.delta_secs();
    // Mass flow rate of water "ṁ" (m dot), measured in kg/s
    let mass_flow_rate = WATER_DENSITY * *cfg.pump_flow_rate;
    // Empirical water heat capacity formula for liquid water
    let water_heat_capacity =
        4176.2 - (0.0909 * *cfg.water_temp_in) + (0.000393 * cfg.water_temp_in.powf(2.0));

    // WIP
}

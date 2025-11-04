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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::FIXED_TIMESTEP_SECS;

    use super::*;

    fn setup_app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP_SECS));
        app.insert_resource(SimulationConfig::default());
        app
    }

    fn tank_mass(cfg: &SimulationConfig) -> (f32, f32) {
        (*cfg.tank_water_mass, cfg.tank_water_mass.max())
    }

    #[test]
    fn tank_heats_due_to_solar() {
        let mut app = setup_app();

        app.add_systems(Update, run_simulation);

        let initial_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.solar_irradiance = 1000.0;
        *cfg.panel_area = 3.0;
        *cfg.panel_efficiency = 0.3;
        *cfg.panel_heat_loss_coefficient = 0.0;
        *cfg.panel_loss_area = 0.0;
        *cfg.pipe_overall_heat_transfer_coefficient = 0.0;
        *cfg.tank_heat_loss_coefficient = 0.0;
        *cfg.load_mass_flow_rate = 0.0;

        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(Duration::from_secs_f64(FIXED_TIMESTEP_SECS));
        app.update();

        let new_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;

        assert!(
            new_temp > initial_temp,
            "Tank should heat up due to solar input"
        );
    }

    #[test]
    fn tank_cools_due_to_water_draw() {
        let mut app = setup_app();

        app.add_systems(Update, run_simulation);

        let initial_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.load_mass_flow_rate = 0.1;
        *cfg.load_temp = 10.0;

        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(Duration::from_secs_f64(FIXED_TIMESTEP_SECS));

        app.update();

        let new_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;

        assert!(
            new_temp < initial_temp,
            "Tank should cool due to water being drawn"
        );
    }

    #[test]
    fn zero_tank_mass_results_in_no_change() {
        let mut app = setup_app();

        app.add_systems(Update, run_simulation);

        let initial_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;
        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.tank_water_mass = 0.0;

        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(Duration::from_secs_f64(60.0));
        app.update();

        let new_temp = *app.world().resource::<SimulationConfig>().tank_average_temp;

        assert!(
            (new_temp - initial_temp).abs() < f32::EPSILON,
            "Tank temperature should not change when mass is zero"
        );
    }

    #[test]
    fn water_temp_in_matches_tank_after_simulation() {
        let mut app = setup_app();

        app.add_systems(Update, run_simulation);
        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(Duration::from_secs_f64(60.0));
        app.update();

        let cfg = app.world().resource::<SimulationConfig>();

        assert!(
            (*cfg.water_temp_in - *cfg.tank_average_temp).abs() < f32::EPSILON,
            "Water entering panel should equal tank average temperature after update"
        );
    }

    #[test]
    fn surface_area_grows_mass_unchanged() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();
        let original_mass = *cfg.tank_water_mass;

        *cfg.tank_surface_area = 10.0;

        app.update();

        let (mass, max_mass) = tank_mass(app.world().resource::<SimulationConfig>());

        assert_eq!(mass, original_mass);
        assert!(max_mass > mass);
    }

    #[test]
    fn surface_area_shrinks_mass_clamped() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.tank_surface_area = 1.0;
        *cfg.tank_water_mass = 1000.0;

        app.update();

        let (mass, max_mass) = tank_mass(app.world().resource::<SimulationConfig>());

        assert!(mass <= max_mass);
        assert!(max_mass < 1000.0);
    }

    #[test]
    fn max_mass_updated_original_mass_preserved() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let original_mass = *app.world().resource::<SimulationConfig>().tank_water_mass;

        app.update();

        let (mass, max_mass) = tank_mass(app.world().resource::<SimulationConfig>());

        assert_eq!(mass, original_mass);
        assert_eq!(max_mass, 797.88446);
    }

    #[test]
    fn zero_surface_area_clamps_mass() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.tank_surface_area = 0.0;
        *cfg.tank_water_mass = 500.0;

        app.update();

        let (mass, max_mass) = tank_mass(app.world().resource::<SimulationConfig>());

        assert_eq!(max_mass, 0.0);
        assert_eq!(mass, 0.0);
    }

    #[test]
    fn min_max_height_diameter_ratio() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();

        *cfg.tank_height_diameter_ratio = cfg.tank_height_diameter_ratio.min();

        app.update();

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();
        let (_, max_mass_min) = tank_mass(&cfg);

        *cfg.tank_height_diameter_ratio = cfg.tank_height_diameter_ratio.max();

        app.update();

        let (_, max_mass_max) = tank_mass(app.world().resource::<SimulationConfig>());

        assert!(max_mass_max < max_mass_min);
    }

    #[test]
    fn mass_exactly_at_max_remains() {
        let mut app = setup_app();

        app.add_systems(Update, update_tank_geometry);

        let mut cfg = app.world_mut().resource_mut::<SimulationConfig>();
        let k = *cfg.tank_height_diameter_ratio;
        let radius = (*cfg.tank_surface_area / (2.0 * PI * (1.0 + 2.0 * k))).sqrt();
        let mass_if_full = 2.0 * PI * k * radius.powi(3) * WATER_DENSITY;

        *cfg.tank_water_mass = mass_if_full;

        app.update();

        let (mass, max_mass) = tank_mass(app.world().resource::<SimulationConfig>());

        assert_eq!(mass, max_mass);
    }
}

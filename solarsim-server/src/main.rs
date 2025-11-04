use bevy::{
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabGroup, TabNavigationPlugin},
    },
    prelude::*,
    remote::{
        RemotePlugin,
        http::{Headers, RemoteHttpPlugin},
    },
    ui_widgets::{UiWidgetsPlugins, ValueChange, observe},
};
use serde::{Deserialize, Serialize};

use crate::{
    simulation::{SimulationField, SimulationFieldKind},
    utils::{Maybe, capitalize},
    widgets::slider,
};

mod simulation;
mod utils;
mod widgets;

fn main() {
    let mut app = App::new();

    let cors_headers = Headers::new()
        .insert("Access-Control-Allow-Origin", "*")
        .insert("Access-Control-Allow-Headers", "Content-Type");

    app.add_plugins((
        DefaultPlugins,
        UiWidgetsPlugins,
        InputDispatchPlugin,
        TabNavigationPlugin,
        RemotePlugin::default(),
        RemoteHttpPlugin::default().with_headers(cors_headers),
    ));

    bevy::asset::load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/Segoe UI.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.add_plugins(widgets::plugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            solar_update_system.run_if(resource_exists::<SimulationConfig>),
        )
        .add_systems(
            Update,
            update_field_values.run_if(resource_changed::<SimulationConfig>),
        )
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .register_type::<SimulationConfig>()
        .init_resource::<SimulationConfig>();

    app.run();
}

#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
struct SimulationConfig {
    /// Ambient temperature, measured in °C
    ambient_temp: SimulationField,
    /// Cloud percentile factor (0.0–1.0)
    cloud_factor: SimulationField,
    /// Insulation efficiency factor (0.0-1.0), affects [SimulationConfig::overall_heat_loss_coefficient]
    insulation_efficiency: SimulationField,
    /// Water leak rate factor (0.0-1.0)
    leak_rate: SimulationField,
    /// Solar panel area, measured in m²
    panel_area: SimulationField,
    /// Solar panel efficiency factor "η" (0.0-1.0)
    panel_efficiency: SimulationField,
    /// Panel heat loss coefficient, measured in W/(m²·K)
    panel_heat_loss_coefficient: SimulationField,
    /// Solar panel loss area, measured in m²
    panel_loss_area: SimulationField,
    /// Solar irradiance (energy/area), measured in W/m²
    solar_irradiance: SimulationField,
    /// Average water tank temperature, measured in °C
    tank_average_temp: SimulationField,
    /// Water tank heat loss coefficient, measured in W/(m²·K)
    tank_heat_loss_coefficient: SimulationField,
    /// Water tank volume (capacity), measured in m³
    tank_volume: SimulationField,
    /// Volumetric pump flow rate, measured in m³/s
    pump_flow_rate: SimulationField,
    /// Water temperature entering panel, measured in °C
    water_temp_in: SimulationField,
    /// Water temperature leaving panel, measured in °C
    water_temp_out: SimulationField,
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

#[derive(Component)]
struct FieldValueText;

fn solar_update_system(time: Res<Time<Fixed>>, mut cfg: ResMut<SimulationConfig>) {
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

fn setup(mut commands: Commands, cfg: Res<SimulationConfig>) {
    commands.spawn(Camera2d);

    let mut slider_children = Vec::default();
    let mut read_only_children = Vec::default();

    if let Ok(s) = cfg.reflect_ref().as_struct() {
        for i in 0..s.field_len() {
            if let Some(name) = s.name_at(i) {
                let (min, max, value, kind) =
                    s.field_at(i)
                        .map_or((0.0, 100.0, 0.0, SimulationFieldKind::ReadOnly), |f| {
                            f.try_downcast_ref::<SimulationField>()
                                .map_or((0.0, 100.0, 0.0, SimulationFieldKind::ReadOnly), |field| {
                                    (field.min(), field.max(), **field, field.kind())
                                })
                        });
                let child_entity = commands
                    .spawn((
                        Node {
                            width: percent(50),
                            align_items: AlignItems::Stretch,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(5),
                            ..default()
                        },
                        children![
                            (
                                Node {
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                children![
                                    (
                                        Text::new(format!("{}: ", capitalize(name))),
                                        TextFont::from_font_size(16.0)
                                    ),
                                    (
                                        FieldValueText,
                                        Name::new(name.to_string()),
                                        Text::new(value.to_string()),
                                        TextFont::from_font_size(16.0)
                                    )
                                ]
                            ),
                            (
                                Name::new(name.to_string()),
                                Maybe::new(
                                    (kind == SimulationFieldKind::Slider).then_some((
                                        slider(min, max, value),
                                        observe(
                                            |value_change: On<ValueChange<f32>>,
                                            mut cfg: ResMut<SimulationConfig>,
                                            names: Query<&Name>| {
                                                if let Ok(reflect_struct) = cfg.reflect_mut().as_struct()
                                                    && let Ok(name) = names.get(value_change.event_target())
                                                    && let Some(field) = reflect_struct.field_mut(name)
                                                    && let Some(value) = field.try_downcast_mut::<SimulationField>()
                                                {
                                                    **value = value_change.value;
                                                }
                                            },
                                        ),
                                    ))
                                ),
                            ),
                        ],
                    ))
                    .id();

                match kind {
                    SimulationFieldKind::Slider => {
                        slider_children.push(child_entity);
                    }
                    SimulationFieldKind::ReadOnly => {
                        read_only_children.push(child_entity);
                    }
                }
            } else {
                warn!("Unknown field");
            }
        }
    }

    let slider_node_entity = commands
        .spawn(Node {
            width: percent(50),
            height: percent(100),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        })
        .add_children(slider_children.as_slice())
        .id();

    let read_only_node_entity = commands
        .spawn(Node {
            width: percent(50),
            height: percent(100),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        })
        .add_children(read_only_children.as_slice())
        .id();

    let fields_node_entity = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            TabGroup::default(),
        ))
        .add_children(&[slider_node_entity, read_only_node_entity])
        .id();

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(px(20.0)),
                row_gap: px(20),
                ..default()
            },
            children![(Text::new("Solarsim"), TextFont::from_font_size(32.0))],
        ))
        .add_child(fields_node_entity);
}

fn update_field_values(
    cfg: Res<SimulationConfig>,
    mut texts: Query<(&mut Text, &Name), With<FieldValueText>>,
) {
    for (mut text, name) in texts.iter_mut() {
        if let Ok(reflect_struct) = cfg.reflect_ref().as_struct()
            && let Some(field) = reflect_struct.field(name)
            && let Some(value) = field.try_downcast_ref::<SimulationField>()
        {
            **text = value.to_string();
        }
    }
}

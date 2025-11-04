use bevy::{
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::{ValueChange, observe},
};

use crate::{
    simulation::{SimulationConfig, SimulationField, SimulationFieldKind},
    ui::widgets::slider,
    utils::{Maybe, capitalize},
};

pub mod widgets;

pub fn plugin(app: &mut App) {
    bevy::asset::load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../../assets/fonts/Segoe UI.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.add_plugins(widgets::plugin);

    app.add_systems(Startup, setup);

    app.add_systems(
        Update,
        update_field_values.run_if(resource_changed::<SimulationConfig>),
    );
}

#[derive(Component)]
struct FieldValueText;

fn setup(mut commands: Commands, cfg: Res<SimulationConfig>) {
    commands.spawn(Camera2d);

    let mut slider_children = Vec::default();
    let mut read_only_children = Vec::default();
    let default_field = (0.0, 100.0, 0.0, SimulationFieldKind::ReadOnly);

    if let Ok(s) = cfg.reflect_ref().as_struct() {
        for i in 0..s.field_len() {
            if let Some(name) = s.name_at(i) {
                let (min, max, value, kind) = s.field_at(i).map_or(default_field, |f| {
                    f.try_downcast_ref::<SimulationField>()
                        .map_or(default_field, |field| {
                            (field.min(), field.max(), **field, field.kind())
                        })
                });
                let child_entity = commands
                    .spawn((
                        Node {
                            min_width: percent(35),
                            align_items: AlignItems::Stretch,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(5),
                            ..default()
                        },
                        children![
                            (
                                Node {
                                    align_items: AlignItems::Stretch,
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
            width: percent(70),
            height: percent(100),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::Wrap,
            column_gap: px(20),
            row_gap: px(10),
            ..default()
        })
        .add_children(slider_children.as_slice())
        .id();

    let read_only_node_entity = commands
        .spawn(Node {
            width: percent(30),
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

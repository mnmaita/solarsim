use bevy::{
    prelude::*,
    ui_widgets::{Slider, SliderValue},
};

mod slider;

pub use slider::*;

use crate::{SimulationConfig, simulation::SimulationField};

fn update_widget_values(
    cfg: Res<SimulationConfig>,
    mut sliders: Query<(Entity, &Name), With<Slider>>,
    mut commands: Commands,
) {
    for (slider_ent, name) in sliders.iter_mut() {
        if let Ok(reflect_struct) = cfg.reflect_ref().as_struct()
            && let Some(field) = reflect_struct.field(name)
            && let Some(value) = field.try_downcast_ref::<SimulationField>()
        {
            commands.entity(slider_ent).insert(SliderValue(**value));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(slider::plugin);

    app.add_systems(
        Update,
        update_widget_values.run_if(resource_changed::<SimulationConfig>),
    );
}

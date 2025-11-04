use bevy::{
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};
use serde::{Deserialize, Serialize};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RemotePlugin::default(),
        RemoteHttpPlugin::default(),
    ))
    .add_systems(Startup, hello_world_ui_system)
    .register_type::<HelloWorld>()
    .init_resource::<HelloWorld>();

    app.run();
}

#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct HelloWorld(pub String);

impl FromWorld for HelloWorld {
    fn from_world(world: &mut World) -> Self {
        Self("Hello World!".to_string())
    }
}

fn hello_world_ui_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(Text::new("Hello World!"))],
    ));
}

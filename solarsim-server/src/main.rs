use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    remote::{
        RemotePlugin,
        http::{Headers, RemoteHttpPlugin},
    },
    ui_widgets::UiWidgetsPlugins,
};

mod simulation;
mod ui;
mod utils;

fn main() {
    let mut app = App::new();

    let cors_headers = Headers::new()
        .insert("Access-Control-Allow-Origin", "*")
        .insert("Access-Control-Allow-Headers", "Content-Type");

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        UiWidgetsPlugins,
        InputDispatchPlugin,
        TabNavigationPlugin,
        RemotePlugin::default(),
        RemoteHttpPlugin::default().with_headers(cors_headers),
    ));

    app.insert_resource(ClearColor(Color::linear_rgb(0.017, 0.017, 0.019)));

    app.add_plugins((simulation::plugin, ui::plugin));

    app.insert_resource(Time::<Fixed>::from_seconds(0.5));

    app.run();
}

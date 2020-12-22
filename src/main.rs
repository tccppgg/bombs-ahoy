use bevy::prelude::*;

fn funny(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CameraUiBundle::default()).spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            ..Default::default()
        },
        text: Text {
            value: String::from("Bombs Ahoy! :)"),
            font: asset_server.load("fonts/Langar-Regular.ttf"),
            style: TextStyle {
                font_size: 60.,
                color: Color::WHITE,
                ..Default::default()
            },
        },
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(funny.system())
        .run();
}

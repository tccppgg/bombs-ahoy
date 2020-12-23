use bevy::{app::AppExit, prelude::*};

struct ExitButton;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            hovered: materials.add(Color::rgb(0.85, 0.85, 0.85).into()),
            pressed: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        }
    }
}

fn funny(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect::all(Val::Px(30.)),
                        ..Default::default()
                    },
                    text: Text {
                        value: String::from("Bombs Ahoy!"),
                        font: asset_server.load("fonts/Langar-Regular.ttf"),
                        style: TextStyle {
                            font_size: 60.,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: String::from("Play"),
                            font: asset_server.load("fonts/Langar-Regular.ttf"),
                            style: TextStyle {
                                font_size: 40.,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                })
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with(ExitButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: String::from("Quit"),
                            font: asset_server.load("fonts/Langar-Regular.ttf"),
                            style: TextStyle {
                                font_size: 40.,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                });
        });
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn exit_system(
    mut query: Query<&Interaction, (Mutated<Interaction>, With<ExitButton>)>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for interaction in query.iter_mut() {
        if let Interaction::Clicked = interaction {
            app_exit_events.send(AppExit);
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_startup_system(funny.system())
        .add_system(button_system.system())
        .add_system(exit_system.system())
        .run();
}

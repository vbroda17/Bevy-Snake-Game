use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())    // default_nearest is good for pixle art
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("baker.png");

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            texture,
            ..default()
        }
    );
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100. * time.delta_seconds();
        }
        else if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100. * time.delta_seconds();
        }
        else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100. * time.delta_seconds();
        }
        else if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100. * time.delta_seconds();
        }
    }
}
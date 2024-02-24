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

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.},
    ));
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        else if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        else if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
    }
}
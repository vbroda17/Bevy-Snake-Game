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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.8, 0.0),
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            ..default()
        },
        Player { 
            speed: 100.0,
            last_dir: Some(KeyCode::KeyW),
        },
    ));
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub last_dir: Option<KeyCode>,
}

fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        let direction = 
               if input.pressed(KeyCode::KeyW) && player.last_dir != Some(KeyCode::KeyS) {
            Some(KeyCode::KeyW)
        } else if input.pressed(KeyCode::KeyS) && player.last_dir != Some(KeyCode::KeyW) {
            Some(KeyCode::KeyS)
        } else if input.pressed(KeyCode::KeyD) && player.last_dir != Some(KeyCode::KeyA) {
            Some(KeyCode::KeyD)
        } else if input.pressed(KeyCode::KeyA) && player.last_dir != Some(KeyCode::KeyD) {
            Some(KeyCode::KeyA)
        } else {
            None
        };

        if let Some(dir) = direction {
            player.last_dir = Some(dir);
        }

        if let Some(last_dir) = player.last_dir {
            match last_dir {
                KeyCode::KeyW => transform.translation.y += movement_amount,
                KeyCode::KeyS => transform.translation.y -= movement_amount,
                KeyCode::KeyD => transform.translation.x += movement_amount,
                KeyCode::KeyA => transform.translation.x -= movement_amount,
                _ => {} // Handle other keys if necessary
            }
        }
    }
}
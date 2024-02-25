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
        .add_systems(Update, spawn_food)
        .add_systems(Update, food_check)
        .add_systems(Update, update_snake_segments)
        .run();
}

fn setup(mut commands: Commands, mut player_query: Query<&Player>) {
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
            last_n_directions: vec![KeyCode::KeyW],
        },
    ));


    let initial_direction_vector = vec![KeyCode::KeyW];
    commands.spawn((Transform::default(), SnakeSegment {
        position: Vec2::ZERO, // Set the initial position
        directions: initial_direction_vector, // Initialize the directions vector
    }));
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub last_dir: Option<KeyCode>,
    pub last_n_directions: Vec<KeyCode>,
}

fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player, &mut SnakeSegment)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    for (mut transform, mut player, mut snake_segments) in &mut characters {
        // player speed
        let movement_amount = player.speed * time.delta_seconds();

        // getting player input to set direction
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

            snake_segments.directions.push(dir);
        }

        // moving player
        if let Some(last_dir) = player.last_dir {
            match last_dir {
                KeyCode::KeyW => transform.translation.y += movement_amount,
                KeyCode::KeyS => transform.translation.y -= movement_amount,
                KeyCode::KeyD => transform.translation.x += movement_amount,
                KeyCode::KeyA => transform.translation.x -= movement_amount,
                _ => {} // Handle other keys if necessary
            }
        }

        
        // restricting player to the screen
        let window = window.single();
        let window_width = window.width();
        let window_height = window.height();

        let mut new_x = transform.translation.x;
        let mut new_y = transform.translation.y;
        // should break this up better, but just know center of screen is 0,0 and player is 10px
        new_x = new_x.max(-window_width / 2. + 5.).min(window_width / 2. - 5.);
        new_y = new_y.max(-window_height / 2. + 5.).min(window_height / 2. - 5.);

        transform.translation = Vec3::new(new_x, new_y, 0.0);
    }
}

#[derive(Component)]
pub struct Food;

fn spawn_food(
    mut commands: Commands, 
    window: Query<&Window>,
    query: Query<&Transform, With<Food>>
) {
    if !(query.iter().next().is_none()) {
        return;
    }

    let window = window.single();
    let window_width = window.width();
    let window_height = window.height();

    let x = (rand::random::<f32>() - 0.5) * window_width;
    let y = (rand::random::<f32>() - 0.5) * window_height;

    let x = x.max(-window_width / 2. + 5.).min(window_width / 2. - 5.);
    let y = y.max(-window_height / 2. + 5.).min(window_height / 2. - 5.);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.1, 0.0),
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        },
        Food,
    ));
}

#[derive(Component)]
pub struct SnakeSegment {
    pub position: Vec2,
    pub directions: Vec<KeyCode>,
}

fn food_check(
    mut commands: Commands,
    mut food_positions: Query<(Entity, &Transform), With<Food>>,
    mut player_positions: Query<(&Transform, &Player)>,
    mut snake_segments: Query<(&Transform, Entity), With<SnakeSegment>>,
) {
    let snake_segments_vec: Vec<(&Transform, Entity)> = snake_segments.iter_mut().collect();

    for (player_transformation, player) in &mut player_positions {
        for (food_entity, food_transform) in &mut food_positions {
            let distance = Vec2::new(
                player_transformation.translation.x - food_transform.translation.x,
                player_transformation.translation.y - food_transform.translation.y,
            ).length();

            let collision_distance = 10.;

            if distance < collision_distance {
                commands.entity(food_entity).despawn();

                let last_n_directions: Vec<KeyCode> = player.last_n_directions.clone();

                // adding new snake segment
                if let Some((mut last_segment_transform, _)) = snake_segments_vec.last() {
                    let mut last_segment_transform = last_segment_transform.clone();
                    last_segment_transform.translation += Vec3::new(20.0, 0.0, 0.0); // Adjust position of new segment
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.1, 0.8, 0.0),
                                custom_size: Some(Vec2::new(10., 10.)),
                                ..default()
                            },
                            transform: last_segment_transform.clone(),
                            ..default()
                        },
                        SnakeSegment {
                            position: Vec2::new(last_segment_transform.translation.x, last_segment_transform.translation.y),
                            directions: last_n_directions, // or any initial direction
                        },
                    ));
                }
            }
        }
    }
}

const SEGMENT_SIZE: f32 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn update_snake_segments(
    mut snake_segments: Query<(&mut SnakeSegment, &mut Transform)>,
) {
    for (mut segment, mut transform) in &mut snake_segments.iter_mut() {
        if let Some(direction) = segment.directions.pop() {
            // Update the position based on the direction vector
            let translation = match direction {
                KeyCode::KeyW => Vec3::new(0.0, SEGMENT_SIZE, 0.0),
                KeyCode::KeyS => Vec3::new(0.0, -SEGMENT_SIZE, 0.0),
                KeyCode::KeyA => Vec3::new(-SEGMENT_SIZE, 0.0, 0.0),
                KeyCode::KeyD => Vec3::new(SEGMENT_SIZE, 0.0, 0.0),
                _ => {
                    // Handle other key codes
                    // For now, just return zero vector
                    Vec3::ZERO
                }
            };

            transform.translation += translation;
        }
    }
}


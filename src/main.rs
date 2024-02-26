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
        .add_systems(Update, update_direction)
        .add_systems(Update, move_snake)
        .add_systems(Update, add_snake_segement)
        // .add_systems(Update, spawn_food)
        // .add_systems(Update, food_check)
        .run();
}

#[derive(PartialEq, Clone)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Component)]
// pub struct Snake {
//     pub speed: u32,
//     pub score: u32,
//     pub size: u32,
// }

#[derive(Component)]
pub struct SnakeHead {
    // pub visual: Entity,
    pub direction: Direction,
    pub previous_directions: Vec<Direction>,
    pub score: u32,
    pub speed: u32,
}

#[derive(Component)]
pub struct SnakeBody {
    // pub visual: Entity,
    // pub index: u32,
    pub direction: Direction,
    pub next_directions: Vec<Direction>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let snake_size = Some(Vec2::new(10., 10.));
    let snake_speed = 100;
    let snake_color = Color::rgb(0.1, 0.8, 0.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: snake_color,
                custom_size: snake_size,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        SnakeHead {
            direction: Direction::Up,
            previous_directions: Vec::new(),
            score: 0,
            speed: snake_speed,
        },
    ));
}

fn update_direction(
    // mut commands: Commands,
    mut snake_head: Query<&mut SnakeHead>,
    // mut snake_body: Query<&mut SnakeBody>,
    input: Res<ButtonInput<KeyCode>>,
){
    for mut head in snake_head.iter_mut() {
        let previous_direction = head.direction.clone();
        
        let new_direction = 
            if input.pressed(KeyCode::KeyW) && previous_direction != Direction::Down {
                println!("Now Going Up");
                Direction::Up
            } 
            else if input.pressed(KeyCode::KeyS) && previous_direction != Direction::Up {
                println!("Now Going Down");
                Direction::Down
            }
            else if input.pressed(KeyCode::KeyA) && previous_direction != Direction::Right {
                println!("Now Going Left");
                Direction::Left
            }
            else if input.pressed(KeyCode::KeyD) && previous_direction != Direction::Left {
                println!("Now Going Right");
                Direction::Right
            }
            else {
                previous_direction.clone()
            };

        head.direction = new_direction.clone();
        head.previous_directions.push(new_direction.clone());
        if head.previous_directions.len() >= (head.speed * (head.score + 1)) as usize {
            head.previous_directions.pop();
            // println!("DId a pop");
        }
    }

    // Now the snake body part
    // for mut body in snake_body.iter_mut() {
    //     // let mut 
    // }
}

fn move_snake(
    mut head_query: Query<(&mut Transform, &SnakeHead), With<SnakeHead>>,
    mut body_query: Query<(&mut Transform, &SnakeBody), Without<SnakeHead>>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    if let Ok((mut transform, head)) = head_query.get_single_mut() {
        match head.direction {
            Direction::Up => {
                transform.translation.y += 100. * time.delta_seconds();
                // println!("Going Up")
            },
            Direction::Down => {
                transform.translation.y -= 100. * time.delta_seconds();
                // println!("Going Down ")
            },
            Direction::Right => {
                transform.translation.x += 100. * time.delta_seconds();
                // println!("Going Right")
            },
            Direction::Left => {
                transform.translation.x -= 100. * time.delta_seconds();
                // println!("Going Left")
            },
            _ => {}
        }
        let window = window.single();
        let window_width = window.width();
        let window_height = window.height();
        let new_x = transform.translation.x.max(-window_width / 2. + 5.).min(window_width / 2. - 5.);
        let new_y = transform.translation.y.max(-window_height / 2. + 5.).min(window_height / 2. - 5.);
        transform.translation = Vec3::new(new_x, new_y, 0.0);
    }

    for (mut body_transformation, mut body) in &mut body_query {
        body_transformation.translation.y += 100. * time.delta_seconds();
    }
}


fn add_snake_segement(
    mut commands: Commands,
    mut head_query: Query<(&mut Transform, &mut SnakeHead), With<SnakeHead>>,
    mut body_query: Query<(&mut Transform, &SnakeBody), Without<SnakeHead>>,
    // mut snake: Query<&mut Snake>,
    // mut snake_position: Query<&mut Transform, 
    input: Res<ButtonInput<KeyCode>>,
){
    if !input.pressed(KeyCode::Space) {
        return;
    }
    // going to set some variables
    // if let Ok((mut transform, head)) = head_query.get_single_mut() {
        
    // }
    let snake_color = Color::rgb(0.1, 0.8, 0.5);
    let snake_size = Some(Vec2::new(10., 10.));

    if body_query.is_empty() {
        println!("Testing here");
        if let Ok((mut transform, mut head)) = head_query.get_single_mut() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: snake_color,
                        custom_size: snake_size,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                ..default()
                },
                SnakeBody {
                    direction: head.direction.clone(),
                    next_directions: head.previous_directions.clone()
                },
            ));

            head.score += 1;
        }
    }
}


// commands.spawn((
//     SpriteBundle {
//         sprite: Sprite {
//             color: snake_color,
//             custom_size: snake_size,
//             ..default()
//         },
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             scale: Vec3::new(1.0, 1.0, 1.0),
//             ..default()
//         },
//         ..default()
//     },
//     SnakeHead {
//         direction: Direction::Up,
//         previous_directions: Vec::new(),
//         score: 0,
//         speed: snake_speed,
//     },
// ));
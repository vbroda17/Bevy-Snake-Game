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
        // .add_systems(Update, character_movement)
        // .add_systems(Update, move_snake_segments)
        // .add_systems(Update, spawn_food)
        // .add_systems(Update, food_check)
        .run();
}

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Snake {
    pub speed: u32,
    pub score: u32,
    pub size: u32,

    pub snake_segments: Vec<SnakeSegment>,
}

#[derive(Component)]
pub struct SnakeSegment {
    pub offset: u32,
    // pub position: Vec
    pub direction: Direction,
    pub direction_queue: Vec<Vec2>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let snake_size = Some(Vec2::new(10., 10.));
    let snake_speed = 100;
    let snake_color = Color::rgb(0.1, 0.8, 0.0);

    let snake_head = SnakeSegment {
        offset: 0,
        direction: Direction::Up,
        direction_queue: Vec::new(),
    };

    let mut initial_segement = Vec:: new();
    initial_segement.push(snake_head);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: snake_color,
                custom_size: snake_size,
                ..default()
            },
            ..default()
        },
        Snake {
            speed: snake_speed,
            score: 0,
            size: snake_size.unwrap().x as u32,
            snake_segments: initial_segement,
        }
    ));


}
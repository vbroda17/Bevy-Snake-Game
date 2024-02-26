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
        // .add_systems(Update, move_snake_segments)
        // .add_systems(Update, spawn_food)
        // .add_systems(Update, food_check)
        .run();
}

#[derive(PartialEq, Clone)]
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
}

#[derive(Component)]
pub struct SnakeHead {
    pub visual: Entity,
    pub direction: Direction,
    pub previous_directions: Vec<Direction>,
}

#[derive(Component)]
pub struct SnakeBody {
    pub visual: Entity,
    pub index: u32,
    pub direction: Direction,
    pub next_directions: Vec<Direction>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let snake_size = Some(Vec2::new(10., 10.));
    let snake_speed = 100;
    let snake_color = Color::rgb(0.1, 0.8, 0.0);

    let snake = commands.spawn(
        Snake {
            speed: snake_speed,
            score: 0,
            size: 10,
        }
    ).id();

    let snake_head_visual = commands.spawn(
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
        }
    ).id();

    let snake_head = commands.spawn(
        SnakeHead {
            visual: snake_head_visual,
            direction: Direction::Up,
            previous_directions: Vec::new(),
        }
    );
}

fn update_direction(
    mut commands: Commands,
    mut snake_head: Query<&mut SnakeHead>,
    input: Res<ButtonInput<KeyCode>>,
){
    for mut head in snake_head.iter_mut() {
        let mut previous_direction = head.direction.clone();
        
        let new_direction = 
            if input.pressed(KeyCode::KeyW) && previous_direction != Direction::Up {
                println!("Now Going Up");
                Direction::Up
            } 
            else if input.pressed(KeyCode::KeyS) && previous_direction != Direction::Down {
                println!("Now Going Down");
                Direction::Down
            }
            else if input.pressed(KeyCode::KeyA) && previous_direction != Direction::Left {
                println!("Now Going Left");
                Direction::Left
            }
            else if input.pressed(KeyCode::KeyD) && previous_direction != Direction::Right {
                println!("Now Going Right");
                Direction::Right
            }
            else {
                previous_direction
            };

        head.direction = new_direction.clone();

    }
}

fn move_snake_segments(){}

fn add_snake_segement(
    // mut snake: Query<&mut Snake>,
    // mut snake_position: Query<&mut Transform, 
){

}
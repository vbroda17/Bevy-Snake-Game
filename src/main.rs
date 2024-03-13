use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::collections::VecDeque;


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
        .add_systems(Update, spawn_snake_body)
        .add_systems(Update, update_snake_component_direction)
        .add_systems(Update, move_snake)
        .run();
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct SnakeHead {
    segement_count: u32,

}

#[derive(Component)]
struct SnakeSegment {
    // position: Position,
    direction: Direction,
    direction_queue: VecDeque<Direction>,
    segement_index: u32,
}


fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    spawn_snake_head(commands, meshes, materials);
}

fn spawn_snake_head(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    let mesh = Mesh::from(Rectangle::new(10., 10.));
    let material = ColorMaterial::from(Color::GREEN);

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
        SnakeHead {
            segement_count: 0,
        },
        SnakeSegment {
            direction: Direction::Up,
            direction_queue: VecDeque::new(),
            segement_index: 0,
        },
    ));
}

fn spawn_snake_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    location: Vec3,
    new_direction: Direction,
    new_index: u32,
    old_queue: VecDeque<Direction>
)
{
    let mesh = Mesh::from(Rectangle::new(10., 10.));
    let material = ColorMaterial::from(Color::GREEN);

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    let mut new_queue = old_queue.clone();
    new_queue.extend(std::iter::repeat(new_direction).take(7));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new( location.x, location.y, location.z ),
                ..default()
            },
            ..default()
        },
        SnakeSegment {
            direction: new_direction,
            direction_queue: new_queue,
            segement_index: new_index
        },
    ));
}

fn spawn_offset(direction: Direction) -> Vec3 {
    match direction {
        Direction::Up => Vec3 { x: 0.0, y: -10.0, z: 0.5 },
        Direction::Down => Vec3 { x: 0.0, y: 10.0, z: 0.5 },
        Direction::Left => Vec3 { x: 10.0, y: 0.0, z: 0.5 },
        Direction::Right => Vec3 { x: -10.0, y: 0.0, z: 0.5 },
    }
}

fn spawn_snake_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
    snake_segment_query: Query<(&Transform, &SnakeSegment), With<SnakeSegment>>,
    mut snake_head_query: Query<&mut SnakeHead>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut snake_head = snake_head_query.single_mut();
        let segement_count = snake_head.segement_count;

        for (transform, segement) in snake_segment_query.iter() {
            // println!("Segment index: {}", segement.segement_index); // test to prove it looks at them in order
            if segement.segement_index == segement_count {
                let offset = spawn_offset(segement.direction) + transform.translation;
                let segement_queue = segement.direction_queue.clone();
                spawn_snake_segment(
                    &mut commands, 
                    &mut meshes, 
                    &mut materials, 
                    offset, 
                    segement.direction, 
                    segement_count + 1,
                    segement_queue
                );
                snake_head.segement_count += 1;
            }
        }

    }
}

fn update_snake_component_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut snake_head_query: Query<&mut SnakeSegment, With<SnakeHead>>,
    mut snake_segment_query: Query<&mut SnakeSegment, Without<SnakeHead>>
){
   let new_direction = update_snake_head_direction(input, snake_head_query);
//    update_snake_non_head_direction(snake_segment_query, new_direction);
}

fn update_snake_head_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut snake_head_query: Query<&mut SnakeSegment, With<SnakeHead>>,
) ->Direction {
    let mut snake_head = snake_head_query.single_mut();
    let previous_direction = snake_head.direction;
    let new_direction = 
        if input.pressed(KeyCode::KeyW) && previous_direction != Direction::Down && previous_direction != Direction::Up{
            println!("Now Going Up");
            Direction::Up
        } 
        else if input.pressed(KeyCode::KeyS) && previous_direction != Direction::Up && previous_direction != Direction::Down{
            println!("Now Going Down");
            Direction::Down
        }
        else if input.pressed(KeyCode::KeyA) && previous_direction != Direction::Right && previous_direction != Direction::Left{
            println!("Now Going Left");
            Direction::Left
        }
        else if input.pressed(KeyCode::KeyD) && previous_direction != Direction::Left && previous_direction != Direction::Right{
            println!("Now Going Right");
            Direction::Right
        }
        else {
            previous_direction.clone()
        };
    
    (*snake_head).direction = new_direction;
    new_direction
}

fn update_snake_non_head_direction(
    mut snake_segment_query: Query<(&mut SnakeSegment, &mut Transform), Without<SnakeHead>>,
    new_direction: Direction

) {
    for (mut snake_segment, _) in &mut snake_segment_query {
        snake_segment.direction_queue.push_back(new_direction);
        let direction = snake_segment.direction_queue.pop_front();
        snake_segment.direction = direction.expect("Direction");
    }
}

fn move_snake(
    time: Res<Time>,
    window: Query<&Window>,
    // mut snake_head_query: Query<(&mut SnakeSegment, &mut Transform), With<SnakeHead>>,
    // mut snake_segment_query: Query<(&mut SnakeSegment, &mut Transform), Without<SnakeHead>>
    mut snake_query: Query<(&mut SnakeSegment, &mut Transform)>
){
    let speed = 100.;
    let mut positions: VecDeque<Vec3> = VecDeque::new();
    // let size = 

    for (segement, transform) in &mut snake_query.iter_mut() {
        println!("Segment index: {}", segement.segement_index); // test to prove it looks at them in order
        positions.push_back(transform.translation.clone());
    }
    // positions.pop_back();   // don't care about last ones position

    for (mut segment, mut transform) in &mut snake_query.iter_mut() {
        if segment.segement_index != 0 {
            let mut new_translation = positions.pop_front();
            new_translation += spawn_offset(segment.direction);
            transform.translation = new_translation.clone().expect("Vec3");
        }
        else {
            let mut new_translation = transform.translation.clone();
            match segment.direction {
                Direction::Up => new_translation.y += speed * time.delta_seconds(),
                Direction::Down => new_translation.y -= speed * time.delta_seconds(),
                Direction::Right => new_translation.x += speed * time.delta_seconds(),
                Direction::Left => new_translation.x -= speed * time.delta_seconds(),
            }
            // new_head_translation = (spawn_offset(head_segment.direction) + new_head_translation) * time.delta_seconds();
            let window = window.single();
            let window_width = window.width();
            let window_height = window.height();
            new_translation.x = new_translation.x.max(-window_width / 2. + 5.).min(window_width / 2. - 5.);
            new_translation.y = new_translation.y.max(-window_height / 2. + 5.).min(window_height / 2. - 5.);

            transform.translation = new_translation;
        }
    }
}
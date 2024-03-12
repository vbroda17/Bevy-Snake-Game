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
        .run();
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment {
    position: Position,
    direction: Direction,
    direction_queue: VecDeque<Direction>,
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
        SnakeHead ,
        SnakeSegment {
            direction: Direction::Up,
            position: Position { x: 0., y: 0. },
            direction_queue: VecDeque::new()
        },
    ));
}

fn spawn_snake_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    location: Vec3,
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
            transform: Transform {
                translation: Vec3::new( location.x, location.y, location.z ),
                ..default()
            },
            ..default()
        },
        SnakeHead ,
        SnakeSegment {
            direction: Direction::Up,
            position: Position { x: 0., y: 0. },
            direction_queue: VecDeque::new()
        },
    ));
}

fn spawn_offset(direction: Direction) -> Vec3 {
    match direction {
        Direction::Up => Vec3 { x: 0.0, y: -10.0, z: 0.5 },
        Direction::Down => Vec3 { x: 0.0, y: 10.0, z: 0.5 },
        Direction::Left => Vec3 { x: -10.0, y: 0.0, z: 0.5 },
        Direction::Right => Vec3 { x: 10.0, y: 0.0, z: 0.5 },
    }
}

fn spawn_snake_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &SnakeSegment), With<SnakeSegment>>
) {
    if input.just_pressed(KeyCode::Space) {
        for (transform, segement) in &query {
            // println!("Vector: {:?}", transform.translation);
            let offset = spawn_offset(segement.direction) + transform.translation;
            // println!("offset: {:?}", offset);
            spawn_snake_segment(&mut commands, &mut meshes, &mut materials, offset);
        }
    }
}
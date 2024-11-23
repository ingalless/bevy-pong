use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
//// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Collider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("setup");
    commands.spawn(Camera2dBundle::default());

    // Paddle
    let paddle_y = LEFT_WALL + GAP_BETWEEN_PADDLE_AND_WALL;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_SIZE.y, PADDLE_SIZE.x))),
            material: materials.add(PADDLE_COLOR),
            transform: Transform::from_xyz(paddle_y, 0.0, 1.0),
            ..default()
        },
        Paddle,
    ));

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Ball,
    ));
}

fn move_paddle(
    keys: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keys.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let top_bound = TOP_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
    let bottom_bound = BOTTOM_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

    for mut paddle in &mut paddle_transform {
        let position = paddle.translation.y + direction * PADDLE_SPEED * time.delta_seconds();

        paddle.translation.y = position.clamp(bottom_bound, top_bound);
    }
}

fn _handle_wall_collisions() {
    todo!();
}

fn _handle_paddle_collisions() {
    todo!();
}

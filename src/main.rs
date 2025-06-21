use bevy::{color::palettes::css::ORANGE, prelude::*};

const PLAYER_MOVE_SPEED: f32 = 200.0;
const PLAYER_PADDLE_LENGTH: f32 = 50.;
const BLOCK_THICKNESS: f32 = 10.;
const LEFT_WALL_X: f32 = -400.;
const RIGHT_WALL_X: f32 = 400.;
const WALL_LENGTH: f32 = 800.;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_camera, add_walls, add_paddle))
        .add_systems(FixedUpdate, (handle_input, move_moving).chain())
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
fn add_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Left wall
    commands.spawn((
        Wall,
        Transform::from_xyz(LEFT_WALL_X, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: BLOCK_THICKNESS,
            y: WALL_LENGTH,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
    ));
    // Right wall
    commands.spawn((
        Wall,
        Transform::from_xyz(RIGHT_WALL_X, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: BLOCK_THICKNESS,
            y: WALL_LENGTH,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
    ));
    commands.spawn((
        Wall,
        Transform::from_xyz(0., 355., 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: WALL_LENGTH,
            y: BLOCK_THICKNESS,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
    ));
}

fn add_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerPaddle,
        Transform::from_xyz(0., -300., 0.0),
        Velocity(Vec2 { x: 0.0, y: 0.0 }),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: PLAYER_PADDLE_LENGTH,
            y: BLOCK_THICKNESS,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
    ));
}
fn move_moving(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let change = velocity.0 * time.delta_secs();
        transform.translation += Vec3::new(change.x, change.y, 0.0);
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<PlayerPaddle>>,
) {
    let mut vel: f32 = 0.0;
    let (mut velocity_comp, mut transform_comp) = query.single_mut().unwrap();
    if keys.pressed(KeyCode::KeyA) {
        vel -= PLAYER_MOVE_SPEED;
    }
    if keys.pressed(KeyCode::KeyD) {
        vel += PLAYER_MOVE_SPEED
    }
    let clamped = clamp_paddle_loc(transform_comp);
    if clamped {
        velocity_comp.0.x = 0.;
    }else {
          velocity_comp.0.x = vel;
    }
}

fn clamp_paddle_loc(mut transform: Mut<'_, Transform>) -> bool {
    let mut pos = transform.translation;
    let half_paddle_length = PLAYER_PADDLE_LENGTH /2.;
    let half_block_thickness = BLOCK_THICKNESS / 2.;
    let left_max = LEFT_WALL_X + half_block_thickness + half_paddle_length;
    if pos.x < left_max {
        pos.x = left_max;
        transform.translation = pos;
        return true;
    }
    let right_max = RIGHT_WALL_X - half_block_thickness - half_paddle_length;
    if pos.x > right_max {
        pos.x = right_max;
        transform.translation = pos;
        return true;
    }
    return false;
}

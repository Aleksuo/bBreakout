use bevy::{
    prelude::*,
    color::palettes::{
        css::ORANGE
    },
    render::camera::Viewport
};

const PLAYER_MOVE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_camera, add_paddle))
        .add_systems(FixedUpdate, (handle_input, move_moving ).chain())
        .run();
}

fn add_camera(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport { 
                physical_position: UVec2::new(0, 0), 
                physical_size: (window_size).as_uvec2(), 
                ..default()}
            ),
            ..default()
        },
    ));
}

fn add_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerPaddle,
        Transform::from_xyz(100.0, 100.0, 0.0),
        Velocity(Vec2 { x: 0.0, y: 0.0}),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 { x: 100.0, y: 20.0 }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
    ));
}


fn move_moving(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let change = velocity.0 * time.delta_secs();
        transform.translation += Vec3::new(change.x, change.y, 0.0);
    }
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<PlayerPaddle>>) {
    let mut vel: f32 = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        vel -= PLAYER_MOVE_SPEED;

    }
    if keys.pressed(KeyCode::KeyD) {
        vel += PLAYER_MOVE_SPEED
    }

    for mut vel_comp in &mut query.iter_mut() {
        vel_comp.0.x = vel;
    }
}

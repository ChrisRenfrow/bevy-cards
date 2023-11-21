use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};

// Useful for marking the "main" camera if we have many
#[derive(Component)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}

#[derive(Debug, Component)]
struct Card;

fn add_cards(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 40., y: 60. })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Card,
    ));
}

fn move_cards(
    time: Res<Time>,
    mut q_card: Query<(&mut Transform, &Card)>,
    q_win: Query<&Window, With<PrimaryWindow>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (cam, cam_transform) = q_cam.single();
    let window = q_win.single();
    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (mut card_transform, _) in &mut q_card {
            card_transform.translation.y = world_pos.y;
            card_transform.translation.x = world_pos.x;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500., 300.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, add_cards)
        .add_systems(Update, (close_on_esc, move_cards))
        .run();
}

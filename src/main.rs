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

#[derive(Component)]
struct Card;

const CARD_SIZE: (f32, f32) = (40., 60.);

fn add_cards(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: CARD_SIZE.0,
                    y: CARD_SIZE.1,
                })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform::from_xyz(-CARD_SIZE.0 - 2., 0., 0.),
            ..default()
        },
        Card,
        // Draggable,
        // Hoverable,
    ));
    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: CARD_SIZE.0,
                    y: CARD_SIZE.1,
                })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(CARD_SIZE.0 + 2., 0., 0.),
            ..default()
        },
        Card,
        Draggable,
        Hoverable,
    ));
}

// https://stackoverflow.com/questions/65396065/what-is-an-acceptable-approach-to-dragging-sprites-with-bevy-0-4
/// The player's cursor/pointer world position
#[derive(Default, Resource)]
struct Pointer {
    position: Vec2,
}
/// Whether an entity is draggable
#[derive(Component)]
struct Draggable;
/// When an entity is being dragged
#[derive(Component)]
struct Dragged;

/// Whether an entity is hover-able
#[derive(Component)]
struct Hoverable;
/// When the pointer is hovering above the entity
#[derive(Component)]
struct Hovered;

fn update_pointer(
    mut pointer: ResMut<Pointer>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        pointer.position = world_position;
    }
}

fn hoverable(
    mut commands: Commands,
    pointer: Res<Pointer>,
    q_hoverable: Query<(Entity, &Transform), (With<Hoverable>, Without<Dragged>)>,
) {
    for (entity, transform) in q_hoverable.iter() {
        let half_width = CARD_SIZE.0 / 2.;
        let half_height = CARD_SIZE.1 / 2.;

        if transform.translation.x - half_width < pointer.position.x
            && transform.translation.x + half_width > pointer.position.x
            && transform.translation.y - half_height < pointer.position.y
            && transform.translation.y + half_height > pointer.position.y
        {
            eprintln!("Hovering entity: {:?}", entity);
            commands.entity(entity).insert(Hovered);
        } else {
            eprintln!("Stopped hovering entity: {:?}", entity);
            commands.entity(entity).remove::<Hovered>();
        }
    }
}

fn draggable(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    q_draggable: Query<Entity, (With<Hovered>, With<Draggable>)>,
    q_dragged: Query<Entity, With<Dragged>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(entity) = q_draggable.iter().next() {
            eprintln!("Dragging entity: {:?}", entity);
            commands.entity(entity).insert(Dragged);
        }
    }
    if mouse_input.just_released(MouseButton::Left) {
        for entity in q_dragged.iter() {
            eprintln!("Dropping entity: {:?}", entity);
            commands.entity(entity).remove::<Dragged>();
        }
    }
}

fn dragged(mut q_dragged: Query<(Entity, &mut Transform), With<Dragged>>, pointer: Res<Pointer>) {
    for (_, mut transform) in q_dragged.iter_mut() {
        transform.translation.x = pointer.position.x;
        transform.translation.y = pointer.position.y;
    }
}

fn setup(mut commands: Commands) {
    commands.init_resource::<Pointer>();
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
        .add_systems(Startup, (setup, add_cards))
        .add_systems(
            Update,
            (close_on_esc, update_pointer, hoverable, draggable, dragged),
        )
        .run();
}

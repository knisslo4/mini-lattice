use bevy::prelude::*;
use bevy::render::mesh::shape::Cube;
use bevy::render::{RenderPlugin, settings::RenderCreation};
use bevy::render::settings::{WgpuSettings, Backends};
use bevy::window::{WindowPlugin, PresentMode};
// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::tonemapping::Tonemapping;

fn main() {
    // APP::new() creates a new application
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::GL | Backends::VULKAN),
                    power_preference: bevy::render::settings::PowerPreference::LowPower,
                    ..default()
                }),
            })
         ) // scene settings
        .add_systems(Startup, setup) // what gets executed at startup like the scene
        .add_systems(Update, rotate_cube) // actions
        .run();

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
)   {
    // add camera
    // Tranform - sets position in 3d space
    // looking at points the camera at (0,0,0) of the cube
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        ..default() // fill all non specified settings as defualt
    });

    // add the lighting
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // add the cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cube::new(1.0))), // Fix Cube syntax
            material: materials.add(Color::rgb(0.3, 0.5, 0.7).into()),
            ..default()
        },
        Spin,
    ));
}

// inherit all the component stuff from bevy (i think)
#[derive(Component)]
struct Spin; // custom component

fn rotate_cube (mut query: Query<&mut Transform, With<Spin>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(1.0 * time.delta_seconds());
    }
}

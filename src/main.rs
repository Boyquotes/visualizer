use bevy::prelude::*;

#[derive(Component)]
struct Link;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 4_000.,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, rotating_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(1.0, 5.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Link,
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotating_system(time: Res<Time>, mut query: Query<(&Link, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        let offset = Vec3::new(0., 5., 0.);
        let rotation_angle = time.elapsed_seconds() as f32 * 0.5;
        let translation_to_origin = Mat4::from_translation(offset);

        let rotation = Mat4::from_quat(Quat::from_rotation_z(rotation_angle));

        let translation_back = Mat4::from_translation(-offset);

        let combined_transform = translation_back * rotation * translation_to_origin;

        *transform = Transform::from_matrix(combined_transform);
    }
}

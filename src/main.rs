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
        .add_systems(Update, (rotating_system, update_revolute_joints))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(1.0, 5.0)),
                material: materials.add(Color::rgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Link,
            RevoluteJoint {
                translation: Vec3::new(0., 3., 0.),
                axis: Vec3::Z,
                angle: 0.,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cylinder::new(1.0, 5.0)),
                    material: materials.add(Color::rgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                },
                Link,
                RevoluteJoint {
                    translation: Vec3::new(0., 3., 0.),
                    axis: Vec3::Z,
                    angle: 0.,
                },
            ));
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
pub struct RevoluteJoint {
    pub translation: Vec3,
    pub axis: Vec3,
    pub angle: f32,
}

fn update_revolute_joints(mut query: Query<(&RevoluteJoint, &mut Transform), With<Link>>) {
    for (child_joint, mut child_transform) in &mut query {
        let translation_to_origin = Mat4::from_translation(child_joint.translation);
        let rotation = Mat4::from_quat(Quat::from_rotation_z(child_joint.angle));
        let translation_back = Mat4::from_translation(-child_joint.translation);
        let combined_transform = translation_back * rotation * translation_to_origin;

        *child_transform = Transform::from_matrix(combined_transform);
    }
}

fn rotating_system(time: Res<Time>, mut query: Query<&mut RevoluteJoint>) {
    for mut joint in &mut query {
        let rotation_angle = time.elapsed_seconds() * 0.5;
        joint.angle = rotation_angle;
    }
}

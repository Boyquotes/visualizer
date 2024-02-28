use bevy::prelude::*;

#[derive(Component)]
pub struct Link;

#[derive(Component)]
pub struct RevoluteJoint {
    pub translation: Vec3,
    pub axis: Vec3,
    pub angle: f32,
}

pub fn update_revolute_joints(mut query: Query<(&RevoluteJoint, &mut Transform), With<Link>>) {
    for (child_joint, mut child_transform) in &mut query {
        let translation_to_origin = Mat4::from_translation(child_joint.translation);
        let rotation = Mat4::from_quat(Quat::from_rotation_z(child_joint.angle));
        let translation_back = Mat4::from_translation(-child_joint.translation);
        let combined_transform = translation_back * rotation * translation_to_origin;

        *child_transform = Transform::from_matrix(combined_transform);
    }
}

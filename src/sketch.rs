use bevy::prelude::*;

pub fn gizmo_segment_2d(mut gizmos: &mut Gizmos, a: Vec2, b: Vec2, color: Color) {
    let position = (a + b) / 2.;
    let angle = cal_angle_from_points(a, b);
    let half_length = (a.x - b.x).powi(2) + (a.y - b.y).powi(2).sqrt() / 2.;
    gizmos.primitive_2d(
        Segment2d {
            direction: Direction2d::X,
            half_length,
        },
        position,
        angle,
        color,
    );
}

pub fn gizmos_segment_2d_angle(
    mut gizmos: &mut Gizmos,
    start_pos: Vec2,
    angle: f32, // Angle in radians
    length: f32,
    color: Color,
) -> Vec2 {
    let end_pos = start_pos + length * Vec2::from_angle(angle);
    let position = (end_pos + start_pos) / 2.;
    gizmos.primitive_2d(
        Segment2d {
            direction: Direction2d::X,
            half_length: length / 2.,
        },
        position,
        angle,
        color,
    );

    end_pos
}

fn cal_angle_from_points(a: Vec2, b: Vec2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dy.atan2(dx)
}

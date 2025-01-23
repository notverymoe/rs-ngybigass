// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Vec2Swizzles};
use tinyvec::{array_vec, ArrayVec};

use super::get_ramp_normal_from_dir;

pub(crate) fn get_polygon_data_for_ramp(direction: Vec2, length: f32) -> ([Vec2; 3], [Vec2; 3], [f32; 3]) {
    let size   = Vec2::new(direction.x, -direction.y) * length;
    let normal = direction.perp();

    let right = Vec2::X * size.x.signum();
    let up    = Vec2::Y * size.y.signum();

    // Ordering for CCW polygon 
    if (size.x >= 0.0) == (size.y >= 0.0) {
        (
            [  Vec2::ZERO, Vec2::new(size.x, 0.0), Vec2::new(0.0, size.y)],
            [         -up,                 normal,                 -right],
            [size.x.abs(),                 length,           size.y.abs()],
        )
    } else {
        (
            [  Vec2::ZERO, Vec2::new(0.0, size.y), Vec2::new(size.x, 0.0)],
            [      -right,                -normal,                    -up],
            [size.y.abs(),                 length,           size.x.abs()],
        )
    }
}

pub fn get_polygon_data_for_ramp_boxy(direction: Vec2, length: f32, rect_size_abs: Vec2) -> ([Vec2; 5], [Vec2; 5], [f32; 5]) {

    let offset = get_ramp_normal_from_dir(direction, length).signum()*rect_size_abs;

    let cross_dir = Vec2::new(direction.x, -direction.y);
    let    normal = direction.perp();
    let  tri_size = cross_dir * length;
    let rect_size = cross_dir.signum() * rect_size_abs;
    let rect_size_abs = rect_size.abs();
    let aabb_size     = rect_size_abs*2.0 + tri_size.abs();

    let origin = -rect_size*2.0;

    let point_vert_out = Vec2::new(origin.x, tri_size.y);
    let point_vert_in  = Vec2::new(     0.0, tri_size.y);

    let point_horz_out = Vec2::new(tri_size.x, origin.y);
    let point_horz_in  = Vec2::new(tri_size.x,      0.0);

    let right = Vec2::X * cross_dir.x.signum();
    let up    = Vec2::Y * cross_dir.y.signum();

    if (cross_dir.x >= 0.0) == (cross_dir.y >= 0.0) {
        (
            [     origin,      point_horz_out, point_horz_in,       point_vert_in, point_vert_out].map(|v| v+offset),
            [        -up,               right,        normal,                  up,         -right],
            [aabb_size.x, 2.0*rect_size_abs.y,        length, 2.0*rect_size_abs.x,    aabb_size.y],
        )
    } else {
        (
            [     origin,      point_vert_out, point_vert_in,       point_horz_in, point_horz_out].map(|v| v+offset),
            [     -right,                  up,       -normal,               right,            -up],
            [aabb_size.y, 2.0*rect_size_abs.x,        length, 2.0*rect_size_abs.y,    aabb_size.x],
        )
    }

}

pub fn get_polygon_data_for_oriented_rect_rected(
    origin:     Vec2,
    size:       Vec2,
    direction:  Vec2,
    outer_size: Vec2,
) -> ArrayVec<[Vec2; 8]> {

    // TODO OPT return normals and lengths

    if direction.y == 0.0 {
        let combined = size + outer_size;
        array_vec![
            [Vec2; 8] =>
            origin + Vec2::new( combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x, -combined.y).rotate(direction),
            origin + Vec2::new( combined.x, -combined.y).rotate(direction)
        ]
    } else if direction.x == 0.0 {
        let combined = size.yx() + outer_size;
        array_vec![
            [Vec2; 8] =>
            origin + Vec2::new( combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x, -combined.y).rotate(direction),
            origin + Vec2::new( combined.x, -combined.y).rotate(direction)
        ]
    } else {
        get_polygon_data_for_oriented_rect_rected_quick_impl(
            &[
                origin + Vec2::new( size.x,  size.y).rotate(direction),
                origin + Vec2::new(-size.x,  size.y).rotate(direction),
                origin + Vec2::new(-size.x, -size.y).rotate(direction),
                origin + Vec2::new( size.x, -size.y).rotate(direction),
            ], 
            &[
                direction.perp(),
                -direction,
                -direction.perp(),
                direction
            ],
            outer_size
        )
    }
}

fn get_polygon_data_for_oriented_rect_rected_quick_impl(points: &[Vec2; 4], norms: &[Vec2; 4], size: Vec2) -> ArrayVec<[Vec2; 8]> {

    // TODO OPT return normals and lengths

    let rect_points = [
        Vec2::new( size.x,  size.y),
        Vec2::new(-size.x,  size.y),
        Vec2::new(-size.x, -size.y),
        Vec2::new( size.x, -size.y),
    ];

    let mut result = ArrayVec::<[Vec2; 8]>::default();
    for i in 0..points.len() {
        let p  =  points[i];
        let n1 =   norms[i];
        let n0 = -n1.perp();
        let offset_0 = rect_points.iter().map(|&v| (v, n0.dot(v))).max_by(|(_, x), (_, y)| x.total_cmp(y)).unwrap().0;
        let offset_1 = rect_points.iter().map(|&v| (v, n1.dot(v))).max_by(|(_, x), (_, y)| x.total_cmp(y)).unwrap().0;

        result.extend([
            p + offset_0,
            p + offset_1
        ]);
    }

    result
}
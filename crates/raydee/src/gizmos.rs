// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::shape::ShapeDebugData;

#[derive(Debug, Clone, Copy)]
pub struct DebugDrawOptions {
    pub colour: Color,
    pub depth: f32,
    pub draw_normals: bool,
    pub draw_normals_calculated: bool,
}

impl DebugDrawOptions {

    #[must_use]
    pub const fn new() -> Self {
        Self{
            colour: Color::linear_rgb(0.0, 1.0, 0.0),
            depth: 0.0,
            draw_normals: false,
            draw_normals_calculated: false
        }
    }

    #[must_use]
    pub fn coloured(colour: impl Into<Color>) -> Self {
        Self{
            colour: colour.into(),
            ..Self::new()
        }
    }
    
    #[must_use]
    pub const fn with_colour(self, colour: Color) -> Self {
        Self{
            colour,
            ..self
        }
    }

    #[must_use]
    pub const fn with_depth(self, depth: f32) -> Self {
        Self{
            depth,
            ..self
        }
    }

    #[must_use]
    pub const fn with_draw_normals(self, draw_normals: bool) -> Self {
        Self{
            draw_normals,
            ..self
        }
    }

    #[must_use]
    pub const fn with_draw_normals_calculated(self, draw_normals_calculated: bool) -> Self {
        Self{
            draw_normals_calculated,
            ..self
        }
    }
}

impl Default for DebugDrawOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_shape_debug_data_2d(gizmos: &mut Gizmos, origin: Vec2, data: &ShapeDebugData, options: DebugDrawOptions) {
    render_shape_debug_data(
        gizmos,
        origin,
        data,
        options,
        |gizmos, iter       | { gizmos.linestrip_2d(iter, options.colour); },
        |gizmos, pos, radius| { gizmos.circle_2d(pos, radius, options.colour); },
        |gizmos, start,  end| { gizmos.line_2d(start, end, options.colour); },
    );
}

pub fn render_shape_debug_data_3d(gizmos: &mut Gizmos, origin: Vec2, data: &ShapeDebugData, options: DebugDrawOptions) {
    render_shape_debug_data(
        gizmos,
        origin,
        data,
        options,
        |gizmos, iter       | { gizmos.linestrip(iter.map(|v| v.extend(options.depth)), options.colour); },
        |gizmos, pos, radius| { gizmos.circle(pos.extend(options.depth), radius, options.colour); },
        |gizmos, start,  end| { gizmos.line(start.extend(options.depth), end.extend(options.depth), options.colour); },
    );
}

pub fn render_shape_debug_data(
    gizmos: &mut Gizmos, 
    origin: Vec2, 
    data: &ShapeDebugData, 
    options: DebugDrawOptions,
    mut render_linestrip: impl FnMut(&mut Gizmos, &mut dyn Iterator<Item = Vec2>),
    mut render_circle:    impl FnMut(&mut Gizmos, Vec2, f32),
    mut render_segment:   impl FnMut(&mut Gizmos, Vec2, Vec2),
) {
    match data {
        ShapeDebugData::Circle { radius } => { 
            render_circle(gizmos, origin, *radius); 
        },
        ShapeDebugData::Polygon { .. } => {
            let ShapeDebugData::Polygon { points, .. } = &data else { unreachable!() };
            render_linestrip(gizmos, &mut (0..points.len()).chain(core::iter::once(0)).map(|i| origin + points[i])); 
            for ([from, to, norm], offset) in data.iter_segments() {
                let from = origin + from;
                let to   = origin + to;
                let offset = norm * offset;
                if options.draw_normals {
                    render_normal(gizmos, &mut render_segment, &mut render_circle, from, to, norm, offset);
                }
                if options.draw_normals_calculated {
                    render_normal(gizmos, &mut render_segment, &mut render_circle, from, to, -(to - from).normalize().perp(), offset);
                }
            }
        },
        ShapeDebugData::PolygonRound { radius, .. } => {
            for ([from, to, norm], offset) in data.iter_segments() {
                let from = origin + from;
                let to   = origin + to;
                let offset = norm * offset;
                if *radius > 0.0 {
                    render_circle(gizmos, from, *radius);
                }
                render_segment(gizmos, offset + from, offset + to);
                
                if options.draw_normals {
                    render_normal(gizmos, &mut render_segment, &mut render_circle, from, to, norm, offset);
                }

                if options.draw_normals_calculated {
                    render_normal(gizmos, &mut render_segment, &mut render_circle, from, to, -(to - from).normalize().perp(), offset);
                }
            }
        },
    }
}

fn render_normal(
    gizmos: &mut Gizmos, 
    render_segment: &mut impl FnMut(&mut Gizmos, Vec2, Vec2),
    render_circle: &mut impl FnMut(&mut Gizmos, Vec2, f32),
    from:   Vec2, 
    to:     Vec2, 
    norm:   Vec2,
    offset: Vec2, 
) {
    let mid = offset + (from + to)*0.5;
    let len = (0.25*from.distance(to)).max(1.0);
    render_segment(gizmos, mid, mid + norm*len);
    render_circle(gizmos, mid, len*0.1);
}
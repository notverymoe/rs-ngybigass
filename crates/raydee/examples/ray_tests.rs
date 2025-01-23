// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{color::palettes::css as Colors, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use raydee::prelude::*;

pub trait ShapeMarkerTrait: ShapeDebug + RayTarget + Send + Sync {}
impl<T: ShapeDebug + RayTarget + Send + Sync + 'static> ShapeMarkerTrait for T {}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::new(5))
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_static, update_raycaster, check_colliders).chain())
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
pub struct Shape(Box<dyn ShapeMarkerTrait>, usize);

impl Shape {
    fn new() -> Self {
        Self(Self::get_shape_at_index(0), 0)
    }

    fn next(&mut self) {
        let next = (self.1+1) % 23;
        self.0 = Self::get_shape_at_index(next);
        self.1 = next;
    }

    fn get_shape_at_index(idx: usize) -> Box<dyn ShapeMarkerTrait> {
        match idx {
             1 => Box::new(Rectangle{half_size: Vec2::new(100.0, 50.0)}),
             2 => Box::new(RectangleRounded::new(Rectangle{half_size: Vec2::new(100.0, 50.0)}, 25.0)),
             3 => Box::new(BoxOriented::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize())),
             4 => Box::new(BoxOrientedRound::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), 25.0)),
             5 => Box::new(BoxOrientedBoxy::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0))),
             6 => Box::new(BoxOrientedBoxyRound::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0), 25.0)),
             7 => Box::new(Ramp::new(Vec2::new( 2.0, -1.0).normalize(), 200.0)),
             8 => Box::new(Ramp::new(Vec2::new(-2.0, -1.0).normalize(), 200.0)),
             9 => Box::new(Ramp::new(Vec2::new(-2.0,  1.0).normalize(), 200.0)),
            10 => Box::new(Ramp::new(Vec2::new( 2.0,  1.0).normalize(), 200.0)),
            11 => Box::new(RampBoxy::new(Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))),
            12 => Box::new(RampBoxy::new(Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))),
            13 => Box::new(RampBoxy::new(Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))),
            14 => Box::new(RampBoxy::new(Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))),
            15 => Box::new(RampRound::new(Vec2::new( 2.0, -1.0).normalize(), 200.0, 25.0)),
            16 => Box::new(RampRound::new(Vec2::new(-2.0, -1.0).normalize(), 200.0, 25.0)),
            17 => Box::new(RampRound::new(Vec2::new(-2.0,  1.0).normalize(), 200.0, 25.0)),
            18 => Box::new(RampRound::new(Vec2::new( 2.0,  1.0).normalize(), 200.0, 25.0)),
            19 => Box::new(RampBoxyRound::new(Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)),
            20 => Box::new(RampBoxyRound::new(Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)),
            21 => Box::new(RampBoxyRound::new(Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)),
            22 => Box::new(RampBoxyRound::new(Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)),
             _ => Box::new(Circle::new(50.0)),
        }
    }
}

#[derive(Component)]
pub struct RayCasterCollider {
    origin:    Vec2,
    direction: Vec2,
    hits: Vec<(Entity, [RayIntersection; 2])>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, direction: Vec2::X, hits: Vec::default()});
    commands.spawn(Shape::new());
}

fn update_static(
    mut q: Query<&mut Shape>, 
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Backslash) {
        for mut collider in &mut q {
            collider.next();
        }
    }
}

fn update_raycaster(
    mut q: Query<&mut RayCasterCollider>, 
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
 ) {
    let mut caster = q.get_single_mut().unwrap();
    let mut offset_origin = Vec2::ZERO;
    let mut offset_target = 0.0;

    if keys.pressed(KeyCode::KeyW) {
        offset_origin += Vec2::Y;
    }

    if keys.pressed(KeyCode::KeyA) {
        offset_origin -= Vec2::X;
    }

    if keys.pressed(KeyCode::KeyS) {
        offset_origin -= Vec2::Y;
    }

    if keys.pressed(KeyCode::KeyD) {
        offset_origin += Vec2::X;
    }

    if keys.pressed(KeyCode::KeyQ) {
        offset_target += 1.0;
    }

    if keys.pressed(KeyCode::KeyE) {
        offset_target -= 1.0;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        offset_origin *= 2.0;
        offset_target *= 2.0;
    }

    if offset_origin != Vec2::ZERO {
        offset_origin *= 150.0 * time.delta_secs();
        caster.origin += offset_origin;
    }


    if offset_target != 0.0 {
        offset_target *= time.delta_secs();
        caster.direction = caster.direction.rotate(Vec2::from_angle(offset_target)).normalize();
    }
}

fn check_colliders(
    mut q_caster:  Query<&mut RayCasterCollider>,
    q_static: Query<(Entity, &Shape)>,
) {
    for mut caster in &mut q_caster {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, caster.direction);
        for (shape_id, Shape(shape, _)) in q_static.iter() {
            if let Some(projection) = shape.raycast(Vec2::ZERO, &ray) {
                caster.hits.push((shape_id, projection));
            }
        }
    }

}

fn render(
    mut gizmos: Gizmos, 
    q_shapes: Query<(Entity, &Shape)>,
    q_caster:  Query<&RayCasterCollider>,
) {

    let caster = q_caster.single();
    gizmos.circle_2d(caster.origin, 10.0, Colors::ORANGE_RED);
    gizmos.line_2d(caster.origin, caster.origin + caster.direction * 10000.0, if caster.hits.is_empty() { Colors::GREEN } else { Colors::LIGHT_SEA_GREEN });
    for hit in &caster.hits {
        gizmos.circle_2d(hit.1[0].point, 10.0, Colors::PURPLE       );
        gizmos.circle_2d(hit.1[1].point, 10.0, Colors::MIDNIGHT_BLUE);
        gizmos.line_2d(hit.1[0].point, hit.1[1].point, Color::BLACK);
    }

    for (entity, Shape(shape, _)) in q_shapes.iter() {
        let colour = if caster.hits.iter().any(|v| v.0 == entity) { Colors::RED } else { Colors::PINK };
        render_shape_debug_data_2d(&mut gizmos, Vec2::ZERO, &shape.get_debug_shape_data(), DebugDrawOptions::coloured(colour));
    }

}
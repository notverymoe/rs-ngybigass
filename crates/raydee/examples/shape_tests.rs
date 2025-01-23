// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{color::palettes::css as Colors, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use raydee::prelude::*;

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
pub struct Shape(ShapeStatic, usize);

impl Shape {
    fn new() -> Self {
        Self(Self::get_shape_at_index(0), 0)
    }

    fn next(&mut self) {
        let next = (self.1+1) % 13;
        self.0 = Self::get_shape_at_index(next);
        self.1 = next;
    }

    fn get_shape_at_index(idx: usize) -> ShapeStatic {
        match idx {
            1 => Rectangle{half_size: Vec2::new(100.0, 50.0)}.into(),
            2 => RectangleRounded::new(Rectangle{half_size: Vec2::new(100.0, 50.0)}, 25.0).into(),
            3 => BoxOriented::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize()).into(),
            4 => BoxOrientedRound::new(Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), 25.0).into(),
            5 => Ramp::new(Vec2::new( 2.0, -1.0).normalize(), 200.0).into(),
            6 => Ramp::new(Vec2::new(-2.0, -1.0).normalize(), 200.0).into(),
            7 => Ramp::new(Vec2::new(-2.0,  1.0).normalize(), 200.0).into(),
            8 => Ramp::new(Vec2::new( 2.0,  1.0).normalize(), 200.0).into(),
            9 => RampRound::new(Vec2::new( 2.0, -1.0).normalize(), 200.0, 25.0).into(),
           10 => RampRound::new(Vec2::new(-2.0, -1.0).normalize(), 200.0, 25.0).into(),
           11 => RampRound::new(Vec2::new(-2.0,  1.0).normalize(), 200.0, 25.0).into(),
           12 => RampRound::new(Vec2::new( 2.0,  1.0).normalize(), 200.0, 25.0).into(),
            _ => Circle::new(50.0).into(),
        }
    }
}

#[derive(Component)]
pub struct RayCasterCollider {
    origin:    Vec2,
    direction: Vec2,
    hits: Vec<(Entity, [RayIntersection; 2])>,
    is_cube: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, direction: Vec2::X, hits: Vec::default(), is_cube: false});
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

    if keys.just_pressed(KeyCode::Tab) {
        caster.is_cube = !caster.is_cube;
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

fn make_caster_shape(is_cube: bool) -> ShapeMoving {
    if is_cube { 
        Rectangle{half_size: Vec2::new(100.0, 50.0)}.into() 
    } else { 
        Circle::new(50.0).into() 
    }
}

fn check_colliders(
    mut q_caster:  Query<&mut RayCasterCollider>,
    q_static: Query<(Entity, &Shape)>,
) {
    for mut caster in &mut q_caster {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, caster.direction);
        let caster_shape = make_caster_shape(caster.is_cube);
        for (shape_id, Shape(target_shape, _)) in q_static.iter() {
            let combined = ShapeCombined::between_moving_and_static(&caster_shape, target_shape);
            if let Some(projection) = combined.raycast(Vec2::ZERO, &ray) {
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
    let caster_shape: ShapeMoving = make_caster_shape(caster.is_cube);

    render_shape_debug_data_2d(&mut gizmos, caster.origin, &caster_shape.get_debug_shape_data(), DebugDrawOptions::coloured(Colors::GREEN));

    let first_hit = caster.hits.iter().reduce(|p, c| if p.1[0].distance < c.1[0].distance { p } else { c });
    if let Some(first_hit) = first_hit {
        let hit_shape: ShapeMoving = make_caster_shape(caster.is_cube);
        render_shape_debug_data_2d(&mut gizmos, first_hit.1[0].point, &hit_shape.get_debug_shape_data(), DebugDrawOptions::coloured(Colors::RED));
    }

    gizmos.circle_2d(caster.origin, 10.0, Colors::ORANGE_RED);
    gizmos.line_2d(caster.origin, caster.origin + caster.direction * 10000.0, if caster.hits.is_empty() { Colors::GREEN } else { Colors::LIGHT_SEA_GREEN });
    for hit in &caster.hits {
        gizmos.circle_2d(hit.1[0].point, 10.0, Colors::PURPLE       );
        gizmos.circle_2d(hit.1[1].point, 10.0, Colors::MIDNIGHT_BLUE);
        gizmos.line_2d(hit.1[0].point, hit.1[1].point, Colors::BLACK);
    }

    for (entity, Shape(shape, _)) in q_shapes.iter() {
        let colour = if caster.hits.iter().any(|v| v.0 == entity) { Colors::RED } else { Colors::PINK };
        render_shape_debug_data_2d(&mut gizmos, Vec2::ZERO, &shape.get_debug_shape_data(), DebugDrawOptions::coloured(colour));

        let combined = ShapeCombined::between_moving_and_static(&caster_shape, shape);
        render_shape_debug_data_2d(&mut gizmos, Vec2::ZERO, &combined.get_debug_shape_data(), DebugDrawOptions::coloured(colour * 0.5));
    }

}

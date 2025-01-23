// Copyright 2023 Natalie Baker // AGPLv3 //

// // Circle // //

mod bevy_circle;

// // Circle Aligned // //

mod bevy_rectangle;

mod bevy_rectangle_rounded;
pub use bevy_rectangle_rounded::*;

// // Box Oriented // //

mod box_oriented;
pub use box_oriented::*;

mod box_oriented_round;
pub use box_oriented_round::*;

mod box_oriented_boxy;
pub use box_oriented_boxy::*;

mod box_oriented_boxy_round;
pub use box_oriented_boxy_round::*;

// // Ramp // //

mod ramp;
pub use ramp::*;

mod ramp_round;
pub use ramp_round::*;

mod ramp_boxy;
pub use ramp_boxy::*;

mod ramp_boxy_round;
pub use ramp_boxy_round::*;

// // NGon // //

mod polygon_small;
pub use polygon_small::*;

mod polygon_small_round;
pub use polygon_small_round::*;

// // Shape Types // //

mod shape_static;
pub use shape_static::*;

mod shape_moving;
pub use shape_moving::*;

mod shape_combined;
pub use shape_combined::*;

// // Misc // //

mod shape_common;
pub use shape_common::*;

mod shape_debug;
pub use shape_debug::*;

mod util;
pub(crate) use util::*;
mod hid;
mod offsets;

use skyline::{hook, hooks::InlineCtx};
use offsets::{COORDINATE_AXES_PHOTO_OFFSET, COORDINATE_AXES_REPLAY_OFFSET};

static mut COORDINATE_AXES_FLAG: bool = false;

#[hook(offset=COORDINATE_AXES_PHOTO_OFFSET+0x04, inline)]
unsafe fn coordinate_axes_photo_flag_hook(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut() as *mut _ as *mut bool;
    *w8 = COORDINATE_AXES_FLAG;
}

/*
#[hook(offset=COORDINATE_AXES_PHOTO_OFFSET+0x28C, inline)]
unsafe fn coordinate_axes_photo_vertices_hook(ctx: &mut InlineCtx) {
    let x11 = *ctx.registers[11].x.as_ref() as *mut f32;
    *x11.offset(0) = -5.0;  // X_AXIS_VERTEX_01_X
    *x11.offset(1) = 0.0;   // X_AXIS_VERTEX_01_Y
    *x11.offset(2) = 0.0;   // X_AXIS_VERTEX_01_Z
    *x11.offset(4) = 5.0;   // X_AXIS_VERTEX_02_X
    *x11.offset(5) = 0.0;   // X_AXIS_VERTEX_02_Y
    *x11.offset(6) = 0.0;   // X_AXIS_VERTEX_02_Z
    *x11.offset(8) = 0.0;   // Y_AXIS_VERTEX_01_X
    *x11.offset(9) = -5.0;  // Y_AXIS_VERTEX_01_Y
    *x11.offset(10) = 0.0;  // Y_AXIS_VERTEX_01_Z
    *x11.offset(12) = 0.0;  // Y_AXIS_VERTEX_02_X
    *x11.offset(13) = 5.0;  // Y_AXIS_VERTEX_02_Y
    *x11.offset(14) = 0.0;  // Y_AXIS_VERTEX_02_Z
    *x11.offset(16) = 0.0;  // Z_AXIS_VERTEX_01_X
    *x11.offset(17) = 0.0;  // Z_AXIS_VERTEX_01_Y
    *x11.offset(18) = -5.0; // Z_AXIS_VERTEX_01_Z
    *x11.offset(20) = 0.0;  // Z_AXIS_VERTEX_02_X
    *x11.offset(21) = 0.0;  // Z_AXIS_VERTEX_02_Y
    *x11.offset(22) = 5.0;  // Z_AXIS_VERTEX_02_Z
}

/// Color is stored in linear space and is adjusted to sRGB space in-game.
#[hook(offset=COORDINATE_AXES_PHOTO_OFFSET+0x2DC, inline)]
unsafe fn coordinate_axes_photo_color_hook(ctx: &mut InlineCtx) {
    let x8 = *ctx.registers[8].x.as_ref() as *mut f32;
    *x8.offset(0) = 1.0;    // X_AXIS_COLOR_R 
    *x8.offset(1) = 0.0;    // X_AXIS_COLOR_G 
    *x8.offset(2) = 0.0;    // X_AXIS_COLOR_B 
    *x8.offset(3) = 0.8;    // X_AXIS_COLOR_A 
    *x8.offset(4) = 0.0;    // Y_AXIS_COLOR_R 
    *x8.offset(5) = 1.0;    // Y_AXIS_COLOR_G 
    *x8.offset(6) = 0.0;    // Y_AXIS_COLOR_B 
    *x8.offset(7) = 0.8;    // Y_AXIS_COLOR_A 
    *x8.offset(8) = 0.0;    // Z_AXIS_COLOR_R 
    *x8.offset(9) = 0.0;    // Z_AXIS_COLOR_G 
    *x8.offset(10) = 1.0;   // Z_AXIS_COLOR_B
    *x8.offset(11) = 0.8;   // Z_AXIS_COLOR_A
}
*/

#[hook(offset=COORDINATE_AXES_REPLAY_OFFSET-0x4DC, inline)]
unsafe fn coordinate_axes_replay_flag_hook(ctx: &mut InlineCtx) {
    let w8 = ctx.registers[8].w.as_mut();
    if COORDINATE_AXES_FLAG {
        *w8 = 0;
    } else {
        *w8 = 60;
    }
}

/*
#[hook(offset=COORDINATE_AXES_REPLAY_OFFSET+0xF0, inline)]
unsafe fn coordinate_axes_replay_vertices_hook(ctx: &mut InlineCtx) {
    let x11 = *ctx.registers[11].x.as_ref() as *mut f32;
    *x11.offset(0) = -5.0;  // X_AXIS_VERTEX_01_X
    *x11.offset(1) = 0.0;   // X_AXIS_VERTEX_01_Y
    *x11.offset(2) = 0.0;   // X_AXIS_VERTEX_01_Z
    *x11.offset(4) = 5.0;   // X_AXIS_VERTEX_02_X
    *x11.offset(5) = 0.0;   // X_AXIS_VERTEX_02_Y
    *x11.offset(6) = 0.0;   // X_AXIS_VERTEX_02_Z
    *x11.offset(8) = 0.0;   // Y_AXIS_VERTEX_01_X
    *x11.offset(9) = -5.0;  // Y_AXIS_VERTEX_01_Y
    *x11.offset(10) = 0.0;  // Y_AXIS_VERTEX_01_Z
    *x11.offset(12) = 0.0;  // Y_AXIS_VERTEX_02_X
    *x11.offset(13) = 5.0;  // Y_AXIS_VERTEX_02_Y
    *x11.offset(14) = 0.0;  // Y_AXIS_VERTEX_02_Z
    *x11.offset(16) = 0.0;  // Z_AXIS_VERTEX_01_X
    *x11.offset(17) = 0.0;  // Z_AXIS_VERTEX_01_Y
    *x11.offset(18) = -5.0; // Z_AXIS_VERTEX_01_Z
    *x11.offset(20) = 0.0;  // Z_AXIS_VERTEX_02_X
    *x11.offset(21) = 0.0;  // Z_AXIS_VERTEX_02_Y
    *x11.offset(22) = 5.0;  // Z_AXIS_VERTEX_02_Z
}

/// Color is stored in linear space and is adjusted to sRGB space in-game.
#[hook(offset=COORDINATE_AXES_REPLAY_OFFSET+0x140, inline)]
unsafe fn coordinate_axes_replay_color_hook(ctx: &mut InlineCtx) {
    let x8 = *ctx.registers[8].x.as_ref() as *mut f32;
    *x8.offset(0) = 1.0;    // X_AXIS_COLOR_R 
    *x8.offset(1) = 0.0;    // X_AXIS_COLOR_G 
    *x8.offset(2) = 0.0;    // X_AXIS_COLOR_B 
    *x8.offset(3) = 0.8;    // X_AXIS_COLOR_A 
    *x8.offset(4) = 0.0;    // Y_AXIS_COLOR_R 
    *x8.offset(5) = 1.0;    // Y_AXIS_COLOR_G 
    *x8.offset(6) = 0.0;    // Y_AXIS_COLOR_B 
    *x8.offset(7) = 0.8;    // Y_AXIS_COLOR_A 
    *x8.offset(8) = 0.0;    // Z_AXIS_COLOR_R 
    *x8.offset(9) = 0.0;    // Z_AXIS_COLOR_G 
    *x8.offset(10) = 1.0;   // Z_AXIS_COLOR_B
    *x8.offset(11) = 0.8;   // Z_AXIS_COLOR_A
}
*/

#[skyline::main(name = "axes_toggle")]
fn main() {
    offsets::search_offsets();
    skyline::install_hooks!(
        coordinate_axes_photo_flag_hook,
        // coordinate_axes_photo_vertices_hook,
        // coordinate_axes_photo_color_hook,
        coordinate_axes_replay_flag_hook,
        // coordinate_axes_replay_vertices_hook,
        // coordinate_axes_replay_color_hook,
        hid::get_npad_handheld_state,
        hid::get_npad_joy_dual_state,
        hid::get_npad_full_key_state,
        hid::get_npad_gc_state
    );
}

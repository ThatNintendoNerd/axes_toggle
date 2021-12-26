use skyline::{hook, nn::hid::NpadHandheldState};
use crate::COORDINATE_AXES_FLAG;

#[hook(replace = skyline::nn::hid::GetNpadHandheldState)]
pub unsafe fn get_npad_handheld_state(state: *mut NpadHandheldState, controller_id: *const u32) {
    original!()(state, controller_id);
    coordinate_axes_toggle((*state).Buttons);
}

#[hook(replace = skyline::nn::hid::GetNpadJoyDualState)]
pub unsafe fn get_npad_joy_dual_state(state: *mut NpadHandheldState, controller_id: *const u32) {
    original!()(state, controller_id);
    coordinate_axes_toggle((*state).Buttons);
}

#[hook(replace = skyline::nn::hid::GetNpadFullKeyState)]
pub unsafe fn get_npad_full_key_state(state: *mut NpadHandheldState, controller_id: *const u32) {
    original!()(state, controller_id);
    coordinate_axes_toggle((*state).Buttons);
}

#[hook(replace = skyline::nn::hid::GetNpadGcState)]
pub unsafe fn get_npad_gc_state(state: *mut NpadHandheldState, controller_id: *const u32) {
    original!()(state, controller_id);
    coordinate_axes_toggle((*state).Buttons);
}

unsafe fn coordinate_axes_toggle(buttons: u64) {
    let key_y: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;
    let key_dpad_up: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000_0000;
    let key_dpad_down: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000;

    if (buttons & key_y) != 0 {
        if (buttons & key_dpad_up) != 0 {
            COORDINATE_AXES_FLAG = true;
        } else if (buttons & key_dpad_down) != 0 {
            COORDINATE_AXES_FLAG = false;
        }
    }
}

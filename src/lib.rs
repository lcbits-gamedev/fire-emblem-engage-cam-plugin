// ───────────────────────── crate-wide flags ──────────────────────────
#![feature(ptr_sub_ptr, const_ptr_sub_ptr)]   // still needed by unity::hook

use skyline::libc::c_void;
use unity::prelude::*;

// ───────────────────────── entry point ───────────────────────────────
#[skyline::main(name = "dont_disappear_and_no_death_cam")]
pub fn main() {
    println!("[NoVanish/NoDeathCam] loading …");
    skyline::install_hooks!(
        character_builder_set_visible_forced,
        camera_switch_switch_camera
    );
    println!("[NoVanish/NoDeathCam] ready!");
}

// ───────────────────────── VISIBILITY PATCH ──────────────────────────
// Combat.CharacterBuilder::SetVisibleForced(bool)
// index = 1  →  (this, value)
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced", 1)]
fn character_builder_set_visible_forced(
    this: &mut c_void,
    _value: bool,           // unused: we override it
    method_info: OptionalMethod,
) {
    // Always keep the unit rendered
    call_original!(this, true, method_info);
}

// ───────────────────────── CAMERA PATCH ──────────────────────────────
// Combat.CameraSwitch::SwitchCamera(int nextCamera, bool force)
// index = 2  →  (nextCamera, force) — `this` is implicit
#[unity::hook("Combat", "CameraSwitch", "SwitchCamera", 2)]
fn camera_switch_switch_camera(
    this: &mut c_void,
    next_camera: i32,
    force: bool,
    method_info: OptionalMethod,
) {
    const DEATH_CAM_IDS: [i32; 5] = [1000, 1002, 1003, 1010, 1012];

    if DEATH_CAM_IDS.contains(&next_camera) && !force {
        println!(
            "[NoVanish/NoDeathCam] blocked camera id {} (force = {})",
            next_camera, force
        );
        return;
    }

    call_original!(this, next_camera, force, method_info);
}

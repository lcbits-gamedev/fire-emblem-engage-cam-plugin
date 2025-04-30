// ───────────────────────── crate-wide flags ──────────────────────────
#![feature(ptr_sub_ptr, const_ptr_sub_ptr)]   // still needed by unity::hook

use skyline::libc::c_void;
use unity::prelude::*;

// Define placeholder structs for type safety if needed
// extern "C" {
//     type Combat_CameraManager_o;
//     type Combat_Character_o;
// }

// ───────────────────────── entry point ───────────────────────────────
#[skyline::main(name = "dont_disappear_and_no_death_cam_no_map_return_v5")]
pub fn main() {
    println!("[NoVanish/NoDeathCam/NoMapReturn] loading ...");
    skyline::install_hooks!(
        character_builder_set_visible_forced, // Keep this for initial visibility
        camera_switch_switch_camera,
        camera_manager_end_camera_hook,
        // character_hide_hook // Comment out or remove this hook
        character_teardown_for_combat_hook // Add the new hook
    );
    println!("[NoVanish/NoDeathCam/NoMapReturn] ready!");
}

// ───────────────────────── VISIBILITY PATCH (Initial) ──────────────────────────
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced", 1)]
fn character_builder_set_visible_forced(
    this: &mut c_void,
    _value: bool,
    method_info: OptionalMethod,
) {
    call_original!(this, true, method_info);
}

// ───────────────────────── CAMERA PATCH ──────────────────────────────
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
            "[NoVanish/NoDeathCam/NoMapReturn] blocked camera id {} (force = {})",
            next_camera, force
        );
        return;
    }
    call_original!(this, next_camera, force, method_info);
}

// ───────────────────────── MAP RETURN PATCH ──────────────────────────
#[unity::hook("Combat", "CameraManager", "EndCamera", 2)]
fn camera_manager_end_camera_hook(
    _this: &mut c_void, // Combat_CameraManager_o*
    _transition: bool,
    _camera_mode: i32,
    _method_info: OptionalMethod,
) {
    println!(
        "[NoVanish/NoDeathCam/NoMapReturn] Blocking CameraManager.EndCamera (transition={}, mode={})",
        _transition, _camera_mode
    );
    return; // Prevent original call
}

// ───────────────────────── PREVENT DESPAWN PATCH (v3) ────────────────
// Combat.Character$$TeardownForCombat() -> void
// index = 0 (instance method with no explicit C# args)
#[unity::hook("Combat", "Character", "TeardownForCombat", 0)]
fn character_teardown_for_combat_hook(
    _this: &mut c_void, // Combat_Character_o*
    _method_info: OptionalMethod,
) {
    println!("[NoVanish/NoDeathCam/NoMapReturn] Blocking Character.TeardownForCombat");
    // Prevent the original method from running, stopping the fade-out tween.
    return; // Original returns void.
}
#![feature(ptr_sub_ptr)] // Keep this if needed for other code, or remove if not used

use engage::gamedata::unit::Unit;
use skyline::libc::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

// --- Struct Definitions (Crucial - Offsets/Layout MUST be verified) ---

// Represents C# struct Cinemachine.AxisState
// WARNING: Field order and types are BEST GUESSES - VERIFY WITH IL2CPPINSPECTOR!
#[repr(C)]
struct AxisState {
    value: f32,
    _padding1: [u8; 0x8],
    max_speed: f32,
    accel_time: f32,
    decel_time: f32,
    _padding2: [u8; 0x4],
    min_value: f32,
    max_value: f32,
    wrap: bool,
    _padding_to_end: [u8; 0x2F], // Adjust size based on verification! Example size: 0x50 bytes total
}

// Represents C# class Cinemachine.CinemachineOrbitalTransposer
// WARNING: OFFSET IS A PLACEHOLDER - VERIFY WITH IL2CPPINSPECTOR!
const ORBITAL_TRANSPOSER_XAXIS_OFFSET: usize = 0xA0; // EXAMPLE OFFSET - FIND THE REAL ONE!

// --- Global State for Target Axis ---
static TARGET_COMBAT_XAXIS_ADDRESS: AtomicUsize = AtomicUsize::new(0);

// --- Plugin Entry Point ---
#[skyline::main(name = "prevent_disappearance_and_camera_mod")]
pub fn main() {
    println!("[PreventDisappearance+CameraMod] Initializing...");
    skyline::install_hooks!(
        // --- Visibility Hook ---
        // Re-adding based on previously working script.
        // WARNING: This previously caused a MissingMethod panic in v2.0.0.
        character_builder_set_visible_forced,

        // --- Death State Hook ---
        unit_is_dead, // From previous script

        // --- Camera Hooks ---
        // Hook to find the target AxisState instance using MutateCameraState
        orbital_transposer_mutate_camera_state,
        // Hook to modify the found AxisState instance
        axis_state_update
    );
    // If execution reaches here, all hooks were installed without panic
    println!("[PreventDisappearance+CameraMod] Hooks installed successfully!");
}

// --- Hooks ---

// --- Visibility Hook ---
// Re-added exactly from the previously working script.
// If this panics, the method signature/path is incorrect for v2.0.0.
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced")]
pub fn character_builder_set_visible_forced(
    this: &mut c_void, // Note: Type is just c_void here, skyline::libc::c_void is the same
    value: bool,
    method_info: Option<&c_void>
) {
    println!("[PreventDisappearance+CameraMod] SetVisibleForced called with value: {}. Forcing visibility.", value);

    // Always set to visible, regardless of the input value
    call_original!(this, true, method_info)
}


// --- Death State Hook ---
// From previously working script. Allows the game to mark units as dead internally.
#[unity::hook("App", "Unit", "IsDead")]
pub fn unit_is_dead(this: &Unit, method_info: Option<&c_void>) -> bool {
    let is_dead = call_original!(this, method_info);
    if is_dead {
        // Corrected: Cast 'this' to a pointer and use {:p} formatter
        println!("[PreventDisappearance+CameraMod] Unit {:p} marked dead, but SetVisibleForced hook should keep it visible.", this as *const Unit);
    }
    is_dead
}


// --- Camera Hooks ---

// Hook MutateCameraState on the transposer to identify its instance and find m_XAxis.
// Signature assumption: fn(this, &mut CameraState, f32)
// WARNING: May cause MissingMethod panic if incorrect for v2.0.0.
#[unity::hook("Cinemachine", "CinemachineOrbitalTransposer", "MutateCameraState")]
fn orbital_transposer_mutate_camera_state(
    this: &mut c_void, // Pointer to the CinemachineOrbitalTransposer instance
    state: &mut c_void, // Pointer to the CameraState struct being modified
    delta_time: f32,    // Delta time
    method_info: Option<&c_void>
) {
    // Calculate the potential address of the m_XAxis field within this transposer.
    // REQUIRES THE CORRECT ORBITAL_TRANSPOSER_XAXIS_OFFSET!
    let potential_axis_address = (this as *const c_void as usize) + ORBITAL_TRANSPOSER_XAXIS_OFFSET;

    // Store this address globally. This should be called for the active transposer.
    let old_address = TARGET_COMBAT_XAXIS_ADDRESS.swap(potential_axis_address, Ordering::Relaxed);

    // Log only if the address changes to avoid spamming the log
    if old_address != potential_axis_address && potential_axis_address != 0 {
         println!("[CameraMod] Identified potential combat X-Axis state via MutateCameraState at address: {:#X}", potential_axis_address);
    }

    // Call the original MutateCameraState function with the correct arguments
    call_original!(this, state, delta_time, method_info);
}


// Hook the core AxisState update method. This is where we modify rotation limits.
#[unity::hook("Cinemachine", "AxisState", "Update")]
fn axis_state_update(this: &mut AxisState, delta_time: f32, method_info: Option<&c_void>) {
    // Get the address of the current AxisState instance being updated.
    let current_address = this as *mut AxisState as usize;

    // Load the target address we hopefully found via MutateCameraState.
    let target_address = TARGET_COMBAT_XAXIS_ADDRESS.load(Ordering::Relaxed);

    // Check if the current AxisState is the one we want to modify.
    if target_address != 0 && current_address == target_address {
        // It's our target axis! Modify its properties before calling the original update.
         println!("[CameraMod] Modifying AxisState at {:#X}", current_address); // Debug logging

        // Store original values
        let original_min = this.min_value;
        let original_max = this.max_value;
        let original_wrap = this.wrap;

        // Apply desired settings for 360 rotation
        this.min_value = -180.0; // Allow full circle
        this.max_value = 180.0;  // Allow full circle
        this.wrap = true;        // Enable wrapping

        // Call the original AxisState.Update function
        // This will now use our modified min/max/wrap values for its calculations.
        call_original!(this, delta_time, method_info);

        // Restore original values *after* the call.
        this.min_value = original_min;
        this.max_value = original_max;
        this.wrap = original_wrap;

    } else {
        // Not our target axis, just call the original function without modifications.
        call_original!(this, delta_time, method_info);
    }
}
#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

use engage::gamedata::unit::Unit;
use skyline::libc::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

// --- Struct Definitions (Crucial - Offsets/Layout MUST be verified) ---

// Represents C# struct Cinemachine.AxisState
// WARNING: Field order and types are BEST GUESSES based on common Unity/C# patterns.
//          Verify using tools like Il2CppInspector on the game's specific version!
#[repr(C)]
struct AxisState {
    // --- Fields likely involved in calculation ---
    value: f32,             // The current value of the axis
    _padding1: [u8; 0x8],   // Padding? Or other fields like speed, accel, decel?
                            // Offsets need verification. Let's assume some exist.
    max_speed: f32,         // Max speed of axis change?
    accel_time: f32,        // Acceleration time?
    decel_time: f32,        // Deceleration time?

    // --- Fields controlling clamping and wrapping ---
    _padding2: [u8; 0x4],   // More padding/unknown fields
    min_value: f32,         // Minimum allowed value (clamp)
    max_value: f32,         // Maximum allowed value (clamp)
    wrap: bool,             // Wrap value around min/max?

    // --- Other potential fields ---
    // There might be more fields related to input, recentering etc.
    // We only *strictly* need min_value, max_value, and wrap for this mod.
    // But the *offsets* depend on the full structure.
    // Add more fields based on Il2CppInspector output if needed.
    _padding_to_end: [u8; 0x2F], // Adjust size to match actual total size if known
                                 // Example: If total size is 0x50, padding needed here.
                                 // Needs confirmation.
}

// Represents C# class Cinemachine.CinemachineOrbitalTransposer
// We only need the offset to m_XAxis.
// WARNING: OFFSET IS A PLACEHOLDER - VERIFY!
const ORBITAL_TRANSPOSER_XAXIS_OFFSET: usize = 0xA0; // EXAMPLE OFFSET - FIND THE REAL ONE

// --- Global State for Target Axis ---

// Stores the memory address of the AxisState instance we want to modify (OrbitalTransposer's m_XAxis).
// Use AtomicUsize for basic thread safety, initialized to 0 (invalid address).
static TARGET_COMBAT_XAXIS_ADDRESS: AtomicUsize = AtomicUsize::new(0);

// --- Plugin Entry Point ---

#[skyline::main(name = "prevent_disappearance_and_camera_mod")]
pub fn main() {
    println!("[PreventDisappearance+CameraMod] Initializing...");
    skyline::install_hooks!(
        // Existing Hooks
        character_builder_set_visible_forced,
        unit_is_dead,
        // New Hooks for Camera Mod
        orbital_transposer_on_validate, // Hook to find the target AxisState instance
        axis_state_update             // Hook to modify the AxisState behavior
    );
    println!("[PreventDisappearance+CameraMod] Hooks installed!");
}

// --- Existing Hooks (Unit Visibility) ---

#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced")]
pub fn character_builder_set_visible_forced(
    this: &mut c_void,
    value: bool,
    method_info: Option<&c_void>,
) {
    // Original functionality preserved for context if needed, but always force true
    // println!("[PreventDisappearance+CameraMod] SetVisibleForced called with value: {}. Forcing visibility.", value);
    call_original!(this, true, method_info)
}

#[unity::hook("App", "Unit", "IsDead")]
pub fn unit_is_dead(this: &Unit, method_info: Option<&c_void>) -> bool {
    let is_dead = call_original!(this, method_info);
    // Optional logging
    // if is_dead {
    //     println!("[PreventDisappearance+CameraMod] Unit {:?} marked as dead, but will remain visible.", this.pointer);
    // }
    is_dead
}

// --- New Hooks (Camera Rotation) ---

// Hook a method on CinemachineOrbitalTransposer to identify its m_XAxis address.
// OnValidate is often called when the component is enabled or values change in editor,
// but might also be called at runtime initialization. Update or LateUpdate could also work,
// but might be called more often than needed. Let's try OnValidate first.
// If this doesn't work reliably, try hooking Update or LateUpdate instead.
#[unity::hook("Cinemachine", "CinemachineOrbitalTransposer", "OnValidate")]
fn orbital_transposer_on_validate(this: &c_void, method_info: Option<&c_void>) {
    // Calculate the address of the m_XAxis field within this OrbitalTransposer instance.
    // THIS REQUIRES THE CORRECT ORBITAL_TRANSPOSER_XAXIS_OFFSET!
    let axis_state_address = (this as usize) + ORBITAL_TRANSPOSER_XAXIS_OFFSET;

    // Store this address globally. We assume the relevant combat camera's
    // OrbitalTransposer will call this method at some point.
    // Using Relaxed ordering is likely sufficient here.
    let old_address = TARGET_COMBAT_XAXIS_ADDRESS.swap(axis_state_address, Ordering::Relaxed);

    if old_address != axis_state_address && axis_state_address != 0 {
         println!("[CameraMod] Identified potential combat X-Axis state at address: {:#X}", axis_state_address);
    }

    // Call the original OnValidate function.
    call_original!(this, method_info);
}

// Hook the core AxisState update method. This is called for *all* axes.
#[unity::hook("Cinemachine", "AxisState", "Update")]
fn axis_state_update(this: &mut AxisState, delta_time: f32, method_info: Option<&c_void>) {
    // Get the address of the current AxisState instance being updated.
    let current_address = this as *mut AxisState as usize;

    // Load the target address we found earlier.
    let target_address = TARGET_COMBAT_XAXIS_ADDRESS.load(Ordering::Relaxed);

    // Check if the current AxisState is the one we want to modify.
    if target_address != 0 && current_address == target_address {
        // It's our target axis! Modify its properties before calling the original update.
        // println!("[CameraMod] Modifying AxisState at {:#X}", current_address); // Debug logging

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
        // This is important to potentially avoid breaking other logic
        // that might read these values *after* the Update call.
        this.min_value = original_min;
        this.max_value = original_max;
        this.wrap = original_wrap;

    } else {
        // Not our target axis, just call the original function without modifications.
        call_original!(this, delta_time, method_info);
    }
}
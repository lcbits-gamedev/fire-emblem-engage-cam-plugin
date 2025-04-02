#![feature(ptr_sub_ptr)] // Keep this if needed for other code, or remove if not used

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
    // Install hooks - Temporarily disabling the problematic one
    skyline::install_hooks!(
        // Temporarily disabled due to MissingMethod panic
        // character_builder_set_visible_forced,
        unit_is_dead,
        orbital_transposer_on_validate,
        axis_state_update
    );
    // If execution reaches here, hooks were installed without panic
    println!("[PreventDisappearance+CameraMod] Hooks installed successfully!");
}

// --- Hooks ---

/* TEMPORARILY DISABLED - Caused panic: Failed to find method Combat.CharacterBuilder(SetVisibleForced)
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced")]
pub fn character_builder_set_visible_forced(
    this: &mut c_void,
    // Original signature based on previously working script:
    value: bool,
    method_info: Option<&c_void>,
) {
    println!("[PreventDisappearance+CameraMod] SetVisibleForced called with value: {}. Forcing visibility.", value);

    // Always set to visible, regardless of the input value
    call_original!(this, true, method_info)
}
*/


// Death State Hook (Unchanged)
#[unity::hook("App", "Unit", "IsDead")]
pub fn unit_is_dead(this: &Unit, method_info: Option<&c_void>) -> bool {
    let is_dead = call_original!(this, method_info);
    // Optional logging
    // if is_dead {
    //     println!("[PreventDisappearance+CameraMod] Unit {:?} marked as dead, remains visible.", this.pointer);
    // }
    is_dead
}

// Camera Axis Identification Hook (Unchanged)
#[unity::hook("Cinemachine", "CinemachineOrbitalTransposer", "OnValidate")]
fn orbital_transposer_on_validate(this: &c_void, method_info: Option<&c_void>) {
    let axis_state_address = (this as *const c_void as usize) + ORBITAL_TRANSPOSER_XAXIS_OFFSET;
    let old_address = TARGET_COMBAT_XAXIS_ADDRESS.swap(axis_state_address, Ordering::Relaxed);

    if old_address != axis_state_address && axis_state_address != 0 {
         println!("[CameraMod] Identified potential combat X-Axis state at address: {:#X}", axis_state_address);
    }
    call_original!(this, method_info);
}

// Camera Axis Update Hook (Unchanged)
#[unity::hook("Cinemachine", "AxisState", "Update")]
fn axis_state_update(this: &mut AxisState, delta_time: f32, method_info: Option<&c_void>) {
    let current_address = this as *mut AxisState as usize;
    let target_address = TARGET_COMBAT_XAXIS_ADDRESS.load(Ordering::Relaxed);

    if target_address != 0 && current_address == target_address {
        // println!("[CameraMod] Modifying AxisState at {:#X}", current_address); // Debug logging
        let original_min = this.min_value;
        let original_max = this.max_value;
        let original_wrap = this.wrap;

        this.min_value = -180.0;
        this.max_value = 180.0;
        this.wrap = true;

        call_original!(this, delta_time, method_info);

        this.min_value = original_min;
        this.max_value = original_max;
        this.wrap = original_wrap;
    } else {
        call_original!(this, delta_time, method_info);
    }
}
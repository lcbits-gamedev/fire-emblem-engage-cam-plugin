// --- Plugin Configuration & Features ---
#![feature(ptr_sub_ptr)]

// --- Imports ---
use engage::gamedata::unit::Unit;
use skyline::libc::c_void;
use unity::prelude::*;
use unity::il2cpp::method::OptionalMethod;

// --- Plugin Entry Point ---
#[skyline::main(name = "prevent_disappearance_mod")]
pub fn main() {
    println!("[PreventDisappearance] Initializing...");
    
    // Install only the visibility hooks, which are known to work
    match skyline::install_hooks!(
        character_builder_set_visible_forced,
        unit_is_dead
    ) {
        Ok(_) => println!("[PreventDisappearance] Hooks installed successfully!"),
        Err(e) => println!("[PreventDisappearance] Failed to install hooks: {:?}", e),
    }
}

// --- Hooks ---

/// Hook: `Combat.CharacterBuilder.SetVisibleForced(bool value)` - Forces units visible.
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced", 1)]
pub fn character_builder_set_visible_forced(
    this: &mut c_void,
    value: bool,
    method_info: OptionalMethod
) {
    println!("[PreventDisappearance] SetVisibleForced called with value: {}", value);
    
    // Always override to true
    call_original!(this, true, method_info);
}

/// Hook: `App.Unit.IsDead() -> bool` - Allows internal death state checking. Passthrough.
#[unity::hook("App", "Unit", "IsDead", 0)]
pub fn unit_is_dead(this: &Unit, method_info: OptionalMethod) -> bool {
    // Just pass through - don't modify the internal state
    call_original!(this, method_info)
}
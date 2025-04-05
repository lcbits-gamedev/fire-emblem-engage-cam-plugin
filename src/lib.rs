// --- Imports ---
use skyline::libc::c_void;
use unity::prelude::*; 

// --- Plugin Entry Point ---
#[skyline::main(name = "prevent_disappearance")]
pub fn main() {
    println!("[PreventDisappearance] Initializing...");

    // Install only the necessary hook
    skyline::install_hooks!(
        character_builder_set_visible_forced
    );

    println!("[PreventDisappearance] Hook installed successfully!");
}

// --- Hooks ---
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced", 1)]
fn character_builder_set_visible_forced(
    this: &mut c_void,    // Instance pointer
    _value: bool,         // Included to avoid build errors, but not used
    method_info: OptionalMethod // Metadata about the hooked method
) {
    // Call the original SetVisibleForced function, but always pass 'true'
    call_original!(this, true, method_info);
}
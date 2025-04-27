#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]   

use skyline::libc::c_void;
use unity::prelude::*;

// --- Plugin Entry Point ---
#[skyline::main(name = "prevent_disappearance")]
pub fn main() {
    println!("[PreventDisappearance] Initializing...");

    skyline::install_hooks!(
        character_builder_set_visible_forced
    );

    println!("[PreventDisappearance] Hook installed successfully!");
}

// --- Hooks ---
#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced", 1)]
fn character_builder_set_visible_forced(
    this: &mut c_void,
    _value: bool,          
    method_info: OptionalMethod
) {
    // Force-show the character!
    call_original!(this, true, method_info);
}

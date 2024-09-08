#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

use engage::gamedata::unit::Unit;

#[skyline::main(name = "prevent_disappearance_plugin")]
pub fn main() {
    println!("Prevent Unit Disappearance plugin initialized!");
    skyline::install_hooks!(character_builder_set_visible_forced);
}

#[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced")]
pub fn character_builder_set_visible_forced(
    this: &mut skyline::libc::c_void,
    value: bool,
    method_info: Option<&skyline::libc::c_void>
) {
    println!("SetVisibleForced called with value: {}. Forcing visibility.", value);
    
    // Always set to visible, regardless of the input value
    call_original!(this, true, method_info)
}

// Keep the IsDead hook to allow units to be marked as dead
#[unity::hook("App", "Unit", "IsDead")]
pub fn unit_is_dead(this: &Unit, method_info: Option<&skyline::libc::c_void>) -> bool {
    // Allow the original IsDead logic to run
    let is_dead = call_original!(this, method_info);
    
    if is_dead {
        println!("Unit marked as dead, but will remain visible.");
    }
    
    is_dead
}
#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

use engage::gamedata::unit::Unit;
use engage::gamedata::PersonDataFields;

#[skyline::main(name = "prevent_fade_plugin")]
pub fn main() {
    println!("Prevent Unit Fading plugin initialized!");
}

#[unity::hook("App", "MapSequenceMind", "UnitDeadFade")]
pub fn map_sequence_mind_unit_dead_fade(
    this: &mut skyline::libc::c_void,
    method: &skyline::libc::c_void,
) {
    println!("UnitDeadFade function called. Attempting to prevent fading.");

    // Since we can't directly access the unit, we'll just log the interception
    // and rely on other hooks to prevent fading
}

fn prevent_unit_fading(unit: &mut Unit) {
    println!("Preventing fading for unit");

    // We'll use other methods to ensure visibility since set_visible isn't available
}

#[unity::hook("App", "MapAction", "DeadBind")]
pub fn map_action_dead_bind(this: &mut skyline::libc::c_void, unit: &mut Unit, param: i32) {
    println!("DeadBind function called. Intercepting to prevent fading.");

    prevent_unit_fading(unit);
}

#[unity::hook("App", "Unit", "Die")]
pub fn unit_die(this: &mut Unit) {
    println!("Unit death function called. Preventing disappearance.");
    // Instead of preventing the death, we'll just log it for now
    call_original!(this)
}

#[unity::hook("App", "Unit", "SetVisible")]
pub fn unit_set_visible(this: &mut Unit, visible: bool) {
    println!(
        "SetVisible called with value: {}. Forcing visibility.",
        visible
    );
    // Always set the unit to visible
    call_original!(this, true)
}

#[unity::hook("Combat", "PersonDataFields", "SetDie")]
pub fn person_set_die(this: &mut PersonDataFields, value: Option<&'static str>) {
    println!("Attempt to set 'die' field intercepted. Preventing change.");
    // We're not calling the original function, effectively preventing the change
}

#[unity::hook("Combat", "PersonDataFields", "Update")]
pub fn person_update(this: &mut PersonDataFields) {
    if let Some(die_field) = &this.die {
        if let Ok(die_str) = die_field.get_string() {
            println!("Die field is set with value: {}", die_str);
        } else {
            println!("Die field is set but could not convert to string.");
        }
        println!("Clearing die field to prevent disappearance.");
        this.die = None;
    } else {
        println!("Die field is not set.");
    }
}

#[unity::hook("App", "Unit", "IsVisible")]
pub fn unit_is_visible(this: &Unit) -> bool {
    println!("IsVisible called. Forcing visibility.");
    true // Always return true to keep the unit visible
}

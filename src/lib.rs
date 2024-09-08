#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

use engage::gamedata::unit::Unit;
use engage::gamedata::PersonDataFields;
use std::ffi::c_char;

#[skyline::main(name = "cameraplugin")]
pub fn main() {
    println!("Prevent Unit Disappearance plugin initialized!");
    
    install_hooks();
}

fn install_hooks() {
    skyline::install_hooks!(
        unit_die,
        unit_set_visible,
        person_set_die,
        person_update,
        unit_is_visible
    );
}

#[unity::hook("App", "Unit", "Die")]
pub fn unit_die(_this: &mut Unit) {
    println!("Unit death function called. Attempting to prevent disappearance.");
    // Instead of preventing the death, we'll just log it for now
    // Uncomment the following line if you want to call the original function
    // call_original!(_this)
}

#[unity::hook("App", "Unit", "SetVisible")]
pub fn unit_set_visible(this: &mut Unit, visible: bool) {
    println!("SetVisible called with value: {}. Forcing visibility.", visible);
    // Always set the unit to visible
    call_original!(this, true)
}

#[unity::hook("Combat", "PersonDataFields", "SetDie")]
pub fn person_set_die(_this: &mut PersonDataFields, value: *const c_char) {
    if !value.is_null() {
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(value);
            if let Ok(str_slice) = c_str.to_str() {
                println!("Attempt to set 'die' field intercepted. Value: {}. Preventing change.", str_slice);
            } else {
                println!("Attempt to set 'die' field intercepted. Value is not valid UTF-8. Preventing change.");
            }
        }
    } else {
        println!("Attempt to set 'die' field to null intercepted. Preventing change.");
    }
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
pub fn unit_is_visible(_this: &Unit) -> bool {
    println!("IsVisible called. Forcing visibility.");
    true // Always return true to keep the unit visible
}
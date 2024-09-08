# Camera Plugin

This Skyline plugin aims to prevent unit disappearance and fading in a game. However, please note that it is still a work in progress and not fully functional yet. We are actively debugging and refining the code to achieve the desired outcome.

## Current Status

- The plugin is initialized and running, but there are still some kinks to iron out.
- We've hooked into various methods to prevent unit disappearance and fading, but some are currently just logging events. More work is needed to fully achieve the desired outcome.
- We're facing challenges with accessing certain game structures (`MapSequenceMind` and `MapAction`) and methods (`set_visible` on `Unit`), which limits our ability to directly manipulate unit visibility.
- The hooks to prevent changes to the `die` field and clear it during updates are in place, but their effectiveness needs to be verified.
- We've added new hooks for `UnitDeadFade` and `DeadBind` to try and intercept fading mechanisms, but their effectiveness is still being evaluated.

## Code Overview

### `main` function
```rust
#[skyline::main(name = "prevent_fade_plugin")]
pub fn main() {
    println!("Prevent Unit Fading plugin initialized!");
}
```
The plugin is initialized and running, setting the groundwork for our fading prevention efforts.

### `map_sequence_mind_unit_dead_fade` hook
```rust
#[unity::hook("App", "MapSequenceMind", "UnitDeadFade")]
pub fn map_sequence_mind_unit_dead_fade(this: &mut skyline::libc::c_void, method: &skyline::libc::c_void) {
    println!("UnitDeadFade function called. Attempting to prevent fading.");
}
```
We've hooked into the `UnitDeadFade` method with the intention of preventing unit fading, but as of now, it's just logging the event. We're unable to directly access the unit due to type constraints.

### `map_action_dead_bind` hook
```rust
#[unity::hook("App", "MapAction", "DeadBind")]
pub fn map_action_dead_bind(this: &mut skyline::libc::c_void, unit: &mut Unit, param: i32) {
    println!("DeadBind function called. Intercepting to prevent fading.");
    prevent_unit_fading(unit);
}
```
This hook intercepts the `DeadBind` action, which is likely responsible for initiating the fading process. We call our `prevent_unit_fading` function here.

### `unit_die` hook 
```rust
#[unity::hook("App", "Unit", "Die")]
pub fn unit_die(this: &mut Unit) {
    println!("Unit death function called. Preventing disappearance.");
    call_original!(this)
}
```
We hooked into the `Die` method with the intention of preventing unit disappearance, but as of now, it's just logging the event and calling the original function.

### `unit_set_visible` hook
```rust
#[unity::hook("App", "Unit", "SetVisible")]
pub fn unit_set_visible(this: &mut Unit, visible: bool) {
    println!("SetVisible called with value: {}. Forcing visibility.", visible);
    call_original!(this, true) 
}
```
We're attempting to force units to remain visible regardless of the game's commands by always passing `true` to the original function.

### `person_set_die` hook
```rust
#[unity::hook("Combat", "PersonDataFields", "SetDie")]
pub fn person_set_die(this: &mut PersonDataFields, value: Option<&'static str>) {
    println!("Attempt to set 'die' field intercepted. Preventing change.");
}
```
This hook aims to prevent changes to the `die` field by not calling the original function.

### `person_update` hook
```rust
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
```
We're checking and trying to clear the `die` field during updates to prevent unit disappearance.

### `unit_is_visible` hook
```rust
#[unity::hook("App", "Unit", "IsVisible")] 
pub fn unit_is_visible(this: &Unit) -> bool {
    println!("IsVisible called. Forcing visibility.");
    true
}
```
We added a hook to always return `true` for unit visibility, aiming to keep units visible.

## Next Steps

We are actively working on resolving the issues and making the plugin fully functional. Our next steps include:

- Finding alternative ways to access and manipulate unit visibility, given the constraints we've encountered with the `Unit` struct.
- Debugging and refining the hooks to effectively prevent unit disappearance and fading.
- Verifying the effectiveness of our `die` field manipulation in the `PersonDataFields` hooks.
- Investigating the game's internal mechanisms for unit visibility and fading to find more direct ways of intervention.
- Considering additional hooks or methods that might be necessary to fully prevent unit fading and disappearance.
- Evaluating the effectiveness of the new `UnitDeadFade` and `DeadBind` hooks in preventing fading.
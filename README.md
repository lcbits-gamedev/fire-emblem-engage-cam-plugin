# Camera Plugin

This Skyline plugin aims to prevent unit disappearance in a game. However, please note that it is still a work in progress and not fully functional yet. We are actively debugging and refining the code to achieve the desired outcome.

## Current Status

- The plugin is initialized and running, but there are still some kinks to iron out.
- We've hooked into the `Die` method to prevent unit disappearance, but it's currently just logging the event. More work is needed to fully achieve the desired outcome.
- We attempted to force units to remain visible regardless of the game's commands by hooking into the `SetVisible` method, but it's not having the intended effect yet. We're troubleshooting this issue.
- The hook to prevent changes to the `die` field was added, but it's not fully operational. We're working on making sure it blocks those unwanted changes effectively. 
- We're checking and trying to clear the `die` field during updates in the `Update` method, but it's not quite working as intended. The units are still disappearing, and this part needs more refinement.
- We added a hook to always return `true` for unit visibility in the `IsVisible` method, aiming to keep units visible, but this too hasn't resolved the issue. More debugging is necessary.

## Code Overview

### `main` function
```rust
#[skyline::main(name = "cameraplugin")]
pub fn main() {
    println!("Prevent Unit Disappearance plugin initialized!");
}
```
The plugin is initialized and running, but the groundwork is still being laid.

### `unit_die` hook 
```rust
#[unity::hook("App", "Unit", "Die")]
pub fn unit_die(this: &mut Unit) {
    println!("Unit death function called. Preventing disappearance.");
    call_original!(this)
}
```
We hooked into the `Die` method with the intention of preventing unit disappearance, but as of now, it's just logging the event. This part still needs more work to fully achieve the desired outcome.

### `unit_set_visible` hook
```rust
#[unity::hook("App", "Unit", "SetVisible")]
pub fn unit_set_visible(this: &mut Unit, visible: bool) {
    println!("SetVisible called with value: {}. Forcing visibility.", visible);
    call_original!(this, true) 
}
```
We attempted to force units to remain visible regardless of the game's commands, but it seems like this isn't having the intended effect yet. We're continuing to troubleshoot this issue.

### `person_set_die` hook
```rust
#[unity::hook("Combat", "PersonDataFields", "SetDie")]
pub fn person_set_die(this: &mut PersonDataFields, value: Option<&'static str>) {
    println!("Attempt to set 'die' field intercepted. Preventing change.");
}  
```
The hook to prevent changes to the `die` field was added, but unfortunately, it's not fully operational. We're still working on making sure this blocks those unwanted changes effectively.

### `person_update` hook
```rust
#[unity::hook("Combat", "PersonDataFields", "Update")]
pub fn person_update(this: &mut PersonDataFields) {
    if let Some(die_field) = &this.die {
        println!("Die field is set with value: {}", die_str);
        println!("Clearing die field to prevent disappearance.");
        this.die = None;
    } else {
        println!("Die field is not set.");
    }
}
```
We're checking and trying to clear the `die` field during updates, but it's not quite working as intended yet. The units are still disappearing, and this part needs more refinement.

### `unit_is_visible` hook
```rust
#[unity::hook("App", "Unit", "IsVisible")] 
pub fn unit_is_visible(this: &Unit) -> bool {
    println!("IsVisible called. Forcing visibility.");
    true
}
```
We added a hook to always return `true` for unit visibility, aiming to keep units visible, but this too hasn't resolved the issue. More debugging is necessary.

## Next Steps

We are actively working on resolving the issues and making the plugin fully functional. Our next steps include:

- Debugging and fixing the hooks to prevent unit disappearance effectively.
- Ensuring that the `die` field is properly blocked from unwanted changes.
- Refining the `Update` method to correctly handle the `die` field and prevent unit disappearance. 
- Investigating and resolving the issue with the `IsVisible` hook to keep units visible as intended.

We appreciate your patience as we continue to improve and optimize this plugin. Stay tuned for updates!
# Prevent Unit Disappearance Plugin for Fire Emblem Engage

## Overview

This Skyline plugin aims to prevent unit disappearance in Fire Emblem Engage while still allowing units to be marked as dead. It achieves this by intercepting and modifying the game's visibility control mechanisms.

## Current Status

- The plugin is functional and successfully hooks into key game functions.
- It prevents units from becoming invisible after death, while still allowing them to be marked as dead.
- The plugin has been updated based on new discoveries about the game's internal functions.

## New Discoveries

We have identified a key function in the game's code:

- `Combat.CharacterBuilder$$SetVisibleForced`: This function appears to be responsible for controlling unit visibility in the game.

## Features

1. **Forced Visibility**: Ensures units remain visible at all times by intercepting the `SetVisibleForced` function.
2. **Death State Preservation**: Allows units to be marked as dead without affecting their visibility.

## Code Overview

### Main Function
```rust
#[skyline::main(name = "prevent_disappearance_plugin")]
pub fn main() {
    println!("Prevent Unit Disappearance plugin initialized!");
    skyline::install_hooks!(character_builder_set_visible_forced, unit_is_dead);
}
```
Initializes the plugin and installs all necessary hooks.

### Key Hooks

1. **SetVisibleForced Hook**
   ```rust
   #[unity::hook("Combat", "CharacterBuilder", "SetVisibleForced")]
   pub fn character_builder_set_visible_forced(
       this: &mut skyline::libc::c_void,
       value: bool,
       method_info: Option<&skyline::libc::c_void>
   ) {
       println!("SetVisibleForced called with value: {}. Forcing visibility.", value);
       call_original!(this, true, method_info)
   }
   ```
   This hook intercepts attempts to change unit visibility and forces units to always be visible.

2. **IsDead Hook**
   ```rust
   #[unity::hook("App", "Unit", "IsDead")]
   pub fn unit_is_dead(this: &Unit, method_info: Option<&skyline::libc::c_void>) -> bool {
       let is_dead = call_original!(this, method_info);
       if is_dead {
           println!("Unit marked as dead, but will remain visible.");
       }
       is_dead
   }
   ```
   This hook allows units to be marked as dead without affecting their visibility.

## Changes Based on New Discoveries

1. We now target the `Combat.CharacterBuilder$$SetVisibleForced` function instead of `App.Unit$$SetVisible`. This change was made because `SetVisibleForced` appears to have more direct control over unit visibility.

2. The `SetVisibleForced` hook always calls the original function with `true`, ensuring units remain visible regardless of the game's attempts to hide them.

3. We've kept the `IsDead` hook to allow the game's death logic to function normally, but we ensure this doesn't affect visibility.

4. Removed hooks for `SetDead` and other functions that are no longer necessary with our new approach.

## Installation

1. Ensure you have the latest version of Skyline installed.
2. Copy the `libprevent_disappearance_plugin.nro` file to the `/atmosphere/contents/[TitleID]/romfs/skyline/plugins/` directory on your Switch SD card.
3. Launch Fire Emblem Engage with Skyline enabled.

## Important Notes

- This plugin significantly alters the game's visibility mechanics. While it shouldn't affect core gameplay, it may impact visual aspects of the game, particularly those related to unit death.
- Thoroughly test the plugin in various game scenarios to ensure it doesn't cause unintended side effects.
- The plugin is still in development, and some features may need adjustment based on user feedback and further testing.

## Troubleshooting

If you encounter any issues or unexpected behavior:
1. Check the Skyline logs for any error messages related to the plugin.
2. Ensure you're using the latest version of both Skyline and this plugin.
3. Try disabling other plugins to check for conflicts.

## Contributing

Contributions to improve the plugin are welcome. Please submit pull requests or open issues on the project's GitHub repository.

## License

[Insert appropriate license information here]

## Disclaimer

This plugin is for educational and experimental purposes only. Use at your own risk. The developers are not responsible for any damage to your game saves or console.
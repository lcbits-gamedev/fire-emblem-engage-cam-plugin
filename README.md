# Camera Plugin for Fire Emblem Engage

## Overview

This Skyline plugin aims to prevent unit disappearance and manipulate visibility in Fire Emblem Engage. It's designed to intercept and modify various game functions related to unit visibility and death mechanics.

## Current Status

- The plugin is functional and successfully hooks into several key game functions.
- It prevents units from disappearing by manipulating visibility settings and intercepting death-related function calls.
- The plugin is still in development, and some features may need fine-tuning based on gameplay testing.

## Features

1. **Unit Death Interception**: Logs when a unit death is called, with the option to prevent death entirely.
2. **Forced Unit Visibility**: Ensures units remain visible at all times.
3. **Death Field Manipulation**: Intercepts and prevents changes to the 'die' field in `PersonDataFields`.
4. **Visibility Override**: Always returns true for unit visibility checks.

## Code Overview

### Main Function
```rust
#[skyline::main(name = "cameraplugin")]
pub fn main() {
    println!("Prevent Unit Disappearance plugin initialized!");
    install_hooks();
}
```
Initializes the plugin and installs all hooks.

### Key Hooks

1. **Unit Die Hook**
   ```rust
   #[unity::hook("App", "Unit", "Die")]
   pub fn unit_die(_this: &mut Unit) {
       println!("Unit death function called. Attempting to prevent disappearance.");
       // call_original!(_this) // Uncomment to allow deaths
   }
   ```
   Logs unit death calls. Death prevention is optional.

2. **Set Visible Hook**
   ```rust
   #[unity::hook("App", "Unit", "SetVisible")]
   pub fn unit_set_visible(this: &mut Unit, visible: bool) {
       println!("SetVisible called with value: {}. Forcing visibility.", visible);
       call_original!(this, true)
   }
   ```
   Forces units to always be visible.

3. **Person Set Die Hook**
   ```rust
   #[unity::hook("Combat", "PersonDataFields", "SetDie")]
   pub fn person_set_die(_this: &mut PersonDataFields, value: *const c_char) {
       // Logs and prevents 'die' field changes
   }
   ```
   Intercepts and prevents 'die' field changes.

4. **Person Update Hook**
   ```rust
   #[unity::hook("Combat", "PersonDataFields", "Update")]
   pub fn person_update(this: &mut PersonDataFields) {
       // Clears 'die' field if set
   }
   ```
   Clears the 'die' field during updates.

5. **Is Visible Hook**
   ```rust
   #[unity::hook("App", "Unit", "IsVisible")]
   pub fn unit_is_visible(_this: &Unit) -> bool {
       println!("IsVisible called. Forcing visibility.");
       true
   }
   ```
   Always returns true for visibility checks.

## Installation

1. Ensure you have the latest version of Skyline installed.
2. Copy the `libcameraplugin.nro` file to the `/atmosphere/contents/[TitleID]/romfs/skyline/plugins/` directory on your Switch SD card.
3. Launch Fire Emblem Engage with Skyline enabled.

## Important Notes

- This plugin significantly alters game mechanics related to unit visibility and death. It may affect gameplay balance and progression.
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
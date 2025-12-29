/*
 * Figure out what will let me use files for Lua 
 * Generate files in /usr/bin/ and $HOME/.config
 * * The directory will be named mulbrowser
 * Have the $HOME/.config be the user configuration
 * Have /usr/bin/ be the core/default configuration (minus keybinds)
 * * In $HOME/.config/mulbrowser/keybinds.lua 
 * * * Uses the Neovim way of setting keybinds `vim.keymap.set("MODE", "KEY", "COMMAND")`
 */
/*
 * Directories
 * src/ is for the core functionality of the browser
 * src/core-plugins is for the core mulbrowser plugins. 
 * * This contains all the vim definitions
 * * This is written in Lua 
 * src/ui is for the UI of mulbrowser. 
 * * This is written in Rust with the Leptos and Stylus crates 
 */

fn main() {
    println!("Hello, world!");
}

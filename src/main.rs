/*
 * Figure out what will let me use files for Lua 
 * Generate files in /usr/bin/ and $HOME/.config
 * * The directory will be named mulbrowser
 * Have the $HOME/.config be the user configuration
 * Have /usr/bin/ be the core/default configuration (minus keybinds)
 * * In $HOME/.config/mulbrowser/keybinds.lua 
 * * * Uses the Neovim way of setting keybinds `vim.keymap.set("MODE", "KEY", "COMMAND")`
 */
fn main() {
    println!("Hello, world!");
}

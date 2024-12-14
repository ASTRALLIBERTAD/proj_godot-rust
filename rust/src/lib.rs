mod player;
mod rustplayer;
mod save_manager_rusts;
use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
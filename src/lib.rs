use godot::prelude::*;

mod player;
mod tabuleiro;

struct BatalhaNavalExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BatalhaNavalExtension {}

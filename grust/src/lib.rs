use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod core;
mod ecs;
mod gdscript;
mod rendering;
mod map;
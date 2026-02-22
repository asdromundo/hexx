use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct SingletonRegisterPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for SingletonRegisterPlugin {
    fn enter_tree(&mut self) {
        self.base_mut()
            .add_autoload_singleton("GlobalBevySingleton", "res://scenes/ecs/bevy_singleton.tscn");

        // Perform typical plugin operations here.
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin operations here.
    }
}

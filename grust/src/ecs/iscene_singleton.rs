use godot::classes::Engine;
use godot::obj::{Bounds, WithBaseField, bounds};
use godot::prelude::*;

pub(crate) trait ISceneSingleton:
    GodotClass + Bounds<Declarer = bounds::DeclUser> + Inherits<Object> + WithBaseField
{
    #[allow(dead_code)]
    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(&Self::class_id().to_string_name())
            .unwrap()
            .cast::<Self>()
    }

    fn register(&self) {
        Engine::singleton().register_singleton(&Self::class_id().to_string_name(), &self.to_gd())
    }

    fn unregister(&self) {
        Engine::singleton().unregister_singleton(&Self::class_id().to_string_name())
    }
}

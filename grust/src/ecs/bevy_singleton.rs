use crate::ecs::ISceneSingleton;
use bevy_ecs::prelude::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct BevySingleton {
    pub(crate) world: World,
    pub(crate) schedule: Schedule,
    pub(crate) physics_schedule: Schedule,
    base: Base<Node>,
}

impl ISceneSingleton for BevySingleton {}

#[godot_api]
impl INode for BevySingleton {
    fn init(base: Base<Node>) -> Self {
        BevySingleton {
            world: World::new(),
            schedule: Schedule::default(),
            physics_schedule: Schedule::default(),
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.register();
        godot_print!("BevySingleton registered");
    }

    fn exit_tree(&mut self) {
        self.unregister();
        godot_print!("BevySingleton unregistered");
    }

    fn process(&mut self, _delta: f64) {
        self.schedule.run(&mut self.world);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.physics_schedule.run(&mut self.world);
    }
}

#[godot_api]
impl BevySingleton {
    #[func]
    pub fn foo(&mut self) {
        godot_print!("Hello BevySingleton!");
    }
}

/*
Use this to access
let bevy_instance = Engine::singleton()
        .get_singleton("SingletonRegisterPlugin")
        .unwrap()
        .cast::<bevy_plugin::SingletonRegisterPlugin>();
    bevy_instance.bind().test();
 */

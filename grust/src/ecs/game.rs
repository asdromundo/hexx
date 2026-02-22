use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use godot::classes::{INode, Node, Node2D, Texture2D};
use godot::prelude::*;

use crate::ecs::BevySingleton;
use crate::ecs::ISceneSingleton;
use crate::player::Player;
// use crate::ecs::hex_grid::spawn_board;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Game {
    base: Base<Node>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        Game { base }
    }

    fn ready(&mut self) {
        godot_print!("Rust ready");
        let mut bevy_node = BevySingleton::singleton();
        let mut bevy = bevy_node.bind_mut();
        bevy.foo();
        let world_2d = Node2D::new_alloc();
        self.base_mut().add_child(&world_2d);

        bevy.world
            .insert_non_send_resource(World2D { node: world_2d });
        bevy.world
            .insert_resource(Time { elapsed_seconds: 0.0, delta: 0.0 });

        bevy.world.run_system_once(_setup).ok();
        bevy.world.run_system_once(_list_entities).ok();
        // bevy.world.run_system_once(spawn_board).ok();

        bevy.physics_schedule.add_systems(_move_players );
        
    }

    fn enter_tree(&mut self) {}

    fn exit_tree(&mut self) {}

    fn physics_process(&mut self, delta: f64) {
        let mut bevy_node = BevySingleton::singleton();
        let mut bevy = bevy_node.bind_mut();
        if let Some(mut time) = bevy.world.get_resource_mut::<Time>() {
            time.delta = delta;
            time.elapsed_seconds += delta;
        }
    }
}

#[derive(Component)]
struct CPlayer {
    id: InstanceId,
}

// #[derive(Resource)]
// Not in use for it's a NonSend resource,
// but it is a resource in the sense that it is a singleton that can be accessed from systems.
struct World2D {
    node: Gd<Node2D>,
}

#[derive(Resource)]
struct Time {
    elapsed_seconds: f64,
    delta: f64,
}

fn _setup(mut commands: Commands, mut world_2d: NonSendMut<World2D>) {
    let mut player = Player::new_alloc();
    player.set_texture(&load::<Texture2D>("res://icon.svg"));
    player.set_scale(Vector2 { x: 3.0, y: 3.0 });
    player.set_position(Vector2 { x: 567.0, y: 304.0 });
    world_2d.node.add_child(&player);
    commands.spawn(CPlayer {
        id: player.instance_id(),
    });

}

fn _list_entities(query: Query<&CPlayer>) {
    for player in query.iter() {
        godot_print!("Player with ID: {:?}", player.id);
    }
}

fn _move_players(query: Query<&CPlayer>, time: Res<Time>) {
 // Configuración de la onda
    let frequency = 10.0; // Qué tan rápido oscila
    let amplitude = 150.0; // Qué tan ancha es la onda

    for cplayer in query.iter() { // No necesitas iter_mut si solo lees el ID
        // 1. Recuperar el nodo de Godot
        let mut player: Gd<Player> = Gd::from_instance_id(cplayer.id);
        
        // 2. Calcular el tiempo y el delta
        let t = time.elapsed_seconds as f32;
        let dt = time.delta as f32;

        // 3. Calcular el desplazamiento lateral
        // Usamos COSENO porque estamos aplicando velocidad (cambio por frame), 
        // y la integral del coseno es el seno (la forma que queremos).
        let sway_amount = (t * frequency).cos() * amplitude * dt;

        // 4. Aplicar movimiento local
        // Asumiendo que tu "Frente" es Y (Vector2::UP), tu "Lado" es X.
        // translate mueve el objeto relativo a su rotación actual.
        player.translate(Vector2::new(sway_amount, 0.0));
    
    }
}



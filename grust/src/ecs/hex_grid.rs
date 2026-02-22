use bevy_ecs::prelude::*;
use hexx::{shapes, Hex}; // Importamos shapes y Hex


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilePos(pub Hex);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainType {
    Land,
    Water,
    Mountain,
    Obstacle,
}
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainZone {
    Swamp,
    Tundra,
    Forest,
    Jungle,
    Sea,
    Lake,
}

// Asegúrate de tener tus componentes definidos arriba o importados
// use crate::{TilePos, TerrainType, TerrainZone}; 

pub fn spawn_board(mut commands: Commands) {
    // Definimos los límites del rectángulo.
    // Esto generará un mapa de 10 de ancho (0 a 10) y 10 de alto (0 a 10).
    let left = 0;
    let right = 10;
    let top = 0;
    let bottom = 10;

    // Generamos el iterador de hexágonos
    let hex_grid = shapes::pointy_rectangle([left, right, top, bottom]);

    // Iteramos y creamos las entidades
    for hex in hex_grid {
        commands.spawn((
            TilePos(hex),       // El componente wrapper que creamos
            TerrainType::Land,  // Un terreno por defecto
        ));
    }
    
    // Debug: Confirmación en consola
    println!("Mapa rectangular generado: {} a {} horizontal, {} a {} vertical", left, right, top, bottom);
}
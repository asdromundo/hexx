class_name MockHex

enum Terrain {
	BASE,
	GRASS,
	WATER
}

enum Direction {
	N = 0,
	NE = 1,
	SE = 2,
	S = 3,
	SW = 4,
	NW = 5
}

const TERRAIN_MESHES : Dictionary[Terrain, PackedScene] = {
	Terrain.BASE: preload("res://assets/Base_Triangle.glb"),
	Terrain.GRASS: preload("res://assets/Grass.glb"),
	Terrain.WATER: preload("res://assets/Water.glb")
}

const _PI_SIXTHS := PI / 6

func _init() -> void:
	assert(TERRAIN_MESHES.size() == Terrain.size())

static func render_triangle(p_world: Node3D, p_pointy_orientation: bool, p_origin: Vector2, p_direction: Direction, p_terrain_type: Terrain) -> void:
	var triangle_offset := _PI_SIXTHS * (2*int(p_direction) + int(p_pointy_orientation))
	var gltf_scene : Node3D = TERRAIN_MESHES[p_terrain_type].instantiate()
	p_world.add_child(gltf_scene)
	gltf_scene.global_position = Vector3(p_origin.x, 0.0, p_origin.y) 
	gltf_scene.rotate_y(triangle_offset)

static func render_map(p_world: Node3D, p_pointy_orientation: bool, p_map: TriangleMap) -> void:
	var origin: Vector2
	var type: Terrain
	for idx in p_map.expected_size:
		origin = p_map.origins[idx]
		type = p_map.types[idx] as Terrain
		for direction in Direction.values():
			render_triangle(p_world, p_pointy_orientation, origin, direction, type)

class TriangleMap:
	# Both MUST have the same lenght
	var types: PackedInt32Array
	var origins: PackedVector2Array
	var expected_size: int

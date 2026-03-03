class_name Grid3DMap
extends Node3D

@export var pointy_orientation: bool = true
@export var origin: Vector2 = Vector2.ZERO
@export var map_scale: Vector2 = Vector2(32.0,32.0)
@export var limits: Vector4i = Vector4i(-1, 1, -1, 1)
@onready var map: HexMap = HexMap.new_from(self.pointy_orientation, self.origin, self.map_scale, self.limits)
const RNG_SEED := 221100

func _ready() -> void:
	#_render_hex_map()
	var t_map := MockHex.TriangleMap.new()
	t_map.origins = map.map_to_vector2_array()
	t_map.expected_size = t_map.origins.size()
	t_map.types.resize(t_map.expected_size)
	
	var rng := RandomNumberGenerator.new()
	rng.seed = RNG_SEED
	
	for idx in t_map.expected_size:
		t_map.types[idx] = rng.randi_range(0, MockHex.Terrain.size()-1)
	MockHex.render_map(self,self.pointy_orientation,t_map)	
	#for coord in grid_origins:
		#_render_hex(coord)

func _render_hex_map() -> void:
	var map_arr	:= map.map_to_column_mesh()

	var map_mesh := ArrayMesh.new()
	map_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, map_arr)
	
	var mat := StandardMaterial3D.new()
	mat.albedo_color = Color.REBECCA_PURPLE
	map_mesh.surface_set_material(0, mat)
	
	var mesh_instance := MeshInstance3D.new()
	mesh_instance.mesh = map_mesh
	mesh_instance.scale = Vector3.ONE
	
	add_child(mesh_instance)

func _render_hex(p_origin: Vector2) -> void:
	for direction in MockHex.Direction.values():
		MockHex.render_triangle(self, self.pointy_orientation, p_origin, direction, MockHex.Terrain.BASE)

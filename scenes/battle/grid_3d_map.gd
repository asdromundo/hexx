class_name Grid3DMap
extends Node3D

@export var pointy_orientation: bool = true
@export var origin: Vector2 = Vector2.ZERO
@export var map_scale: Vector2 = Vector2.ONE
@export var limits: Vector4i = Vector4i(-10, 10, -10, 10)
@onready var map: HexMap = HexMap.new_from(self.pointy_orientation, self.origin, self.map_scale, self.limits)

func _ready() -> void:

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

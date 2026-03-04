## HexRenderer.gd
## Responsible for taking HexRenderDataFfi objects (emitted by Rust after map
## generation) and instantiating the correct mesh scenes under each hex node.
##
## This is the ONLY place in GDScript that touches asset instantiation for
## terrain. All decisions about *which* mesh to use were made in Rust.

class_name HexRenderer
extends Node

## Parent node that holds all hex visual nodes in the scene tree.
@export var hex_root: Node3D
## Must match the Grid3DMap
@export var pointy_orientation: bool = true

# World-space Y units per elevation step.
# Adjust to taste; Rust only knows the logical step integer.
const ELEVATION_STEP: float = 0.5

# Wedge meshes are authored with tip at origin, flat edge toward -Z (tip points +Z).
#
# hexx flat:   dir 0 = East (+X). Tip pointing +Z needs +90° to face +X.
# hexx pointy: dir 0 = NE.  NE is 30° from East, so base offset = 90° - 30° = 60°.
#
# Godot rotation_degrees.y positive = CCW from above, so each hexx step
# (CW from above) = -60° per direction.
const BASE_OFFSET_FLAT:   float =  -120.0
const BASE_OFFSET_POINTY: float =  -90.0
const DIRECTION_STEP:     float = -60.0

# ---------------------------------------------------------------------------
# Called by Grid3DMap._on_map_ready().
# Each HexRenderDataFfi already carries its own world_pos — no parallel
# position array needed.
# ---------------------------------------------------------------------------
func render_map(render_data_array: Array) -> void:
	for child in hex_root.get_children():
		child.queue_free()

	for data: HexRenderDataFfi in render_data_array:
		_spawn_hex(data)


# ---------------------------------------------------------------------------
# Spawns all mesh nodes for a single hex.
# ---------------------------------------------------------------------------
func _spawn_hex(data: HexRenderDataFfi) -> void:
	var hex_node := Node3D.new()
	hex_node.position = Vector3(
		data.world_pos.x,
		data.elevation * ELEVATION_STEP,
		data.world_pos.y
	)
	hex_root.add_child(hex_node)

	var base_offset: float = BASE_OFFSET_POINTY if pointy_orientation else BASE_OFFSET_FLAT

	# Spawn 6 wedge meshes.
	for d in 6:
		var key: String = data.wedge_key(d)
		if key.is_empty():
			push_warning("HexRenderer: empty wedge key at direction %d, skipping." % d)
			continue

		var wedge_node: Node = MeshAssetRegistry.instantiate(key)
		if wedge_node == null:
			continue

		# Wedges are authored pre-rotated for direction 0 (East).
		# Rotate each by 60° * direction around Y to align to the correct edge.
		wedge_node.rotation_degrees.y = base_offset + d * DIRECTION_STEP

		hex_node.add_child(wedge_node)

	# Spawn center feature if present.
	if data.has_center_feature():
		var feature_node: Node = MeshAssetRegistry.instantiate(data.center_feature_key)
		if feature_node != null:
			hex_node.add_child(feature_node)

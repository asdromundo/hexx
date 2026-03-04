## HexDebugOverlay.gd
## Spawns Label3D nodes over each hex showing its coord, biome, and
## a colored wedge-direction indicator so you can verify mesh rotation.
##
## Add as a child of Grid3DMap. Disable in production by hiding the node.

class_name HexDebugOverlay
extends Node3D

@export var enabled: bool = true
## How high above the hex surface to float the labels.
@export var label_y_offset: float = 0.5
## Scale of the direction arrow markers.
@export var marker_scale: float = 0.08

## Biome index → human label. Extend if you add biomes.
const BIOME_NAMES: Array[String] = ["Grass", "Water"]

## Direction index → short name (hexx flat convention).
const DIR_NAMES_FLAT:   Array[String] = ["E", "NE", "NW", "W", "SW", "SE"]
const DIR_NAMES_POINTY: Array[String] = ["NE", "N", "NW", "SW", "S", "SE"]

## One color per direction so wedge 0 is always the same color regardless of hex.
const DIR_COLORS: Array[Color] = [
	Color.RED,
	Color.ORANGE,
	Color.YELLOW,
	Color.GREEN,
	Color.CYAN,
	Color.MAGENTA,
]


func draw(
	map_data: MapDataRef,
	render_data_array: Array,
	pointy: bool,
	elevation_step: float,
) -> void:
	if not enabled:
		return

	for child in get_children():
		child.queue_free()

	var dir_names: Array[String] = DIR_NAMES_POINTY if pointy else DIR_NAMES_FLAT

	for i in render_data_array.size():
		var data: HexRenderDataFfi = render_data_array[i]
		var world_y: float = data.elevation * elevation_step + label_y_offset
		var centre := Vector3(data.world_pos.x, world_y, data.world_pos.y)

		# --- Coord + biome label ---
		var coord: Vector2i = map_data.hex_coord_at(i)
		var biome_idx: int  = map_data.biome_at(i)
		var biome_name: String = BIOME_NAMES[biome_idx] if biome_idx < BIOME_NAMES.size() else str(biome_idx)

		var label := Label3D.new()
		label.text = "(%d,%d)\n%s" % [coord.x, coord.y, biome_name]
		label.font_size = 64
		label.modulate = Color.WHITE
		label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		label.position = centre
		label.pixel_size = 0.002
		add_child(label)

		# --- Per-direction wedge markers ---
		# A small colored sphere sits along the direction vector so you can
		# confirm direction 0 (red) points where the mesh's tip points.
		for d in 6:
			var angle_deg: float
			if pointy:
				angle_deg = 90.0 - d * 60.0   # NE=60°, N=90°... in standard math
			else:
				angle_deg = 0.0  - d * 60.0   # E=0°, NE=60°...

			var angle_rad := deg_to_rad(angle_deg)
			# hexx world pos uses X right, Y forward → maps to Godot X, Z
			var dir_offset := Vector3(cos(angle_rad), 0.0, -sin(angle_rad)) * marker_scale * 4.0

			var marker := MeshInstance3D.new()
			var sphere := SphereMesh.new()
			sphere.radius = marker_scale
			sphere.height = marker_scale * 2.0
			marker.mesh = sphere

			var mat := StandardMaterial3D.new()
			mat.albedo_color = DIR_COLORS[d]
			mat.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
			marker.material_override = mat

			marker.position = Vector3(centre.x, world_y, centre.z) + dir_offset

			# Label the direction
			var dir_label := Label3D.new()
			dir_label.text = "%s(%d)" % [dir_names[d], d]
			dir_label.font_size = 48
			dir_label.modulate = DIR_COLORS[d]
			dir_label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
			dir_label.pixel_size = 0.002
			dir_label.position = marker.position + Vector3(0, marker_scale * 2.0, 0)
			add_child(dir_label)
			add_child(marker)

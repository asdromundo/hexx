## Grid3DMap.gd
## View-only coordinator. Owns no game state.
##
## Responsibilities:
##   - Hold map configuration as @export vars (for the editor).
##   - Tell MapGeneratorNode to run generation.
##   - Receive the map_ready signal and forward data to HexRenderer.
##
## NOT responsible for:
##   - Biome assignment (Rust / MapGeneratorNode).
##   - Mesh selection (Rust / MeshRegistry).
##   - Asset loading (MeshAssetRegistry autoload).

class_name Grid3DMap
extends Node3D

# ---------------------------------------------------------------------------
# Map configuration — edit in the Inspector.
# These are forwarded verbatim to MapGeneratorNode.generate_map().
# ---------------------------------------------------------------------------
@export var pointy_orientation: bool = true
@export var origin: Vector2 = Vector2.ZERO
@export var map_scale: Vector2 = Vector2(1.0, 1.0)
@export var limits: Vector4i = Vector4i(-1, 1, -1, 1)
@export var rng_seed: int = 221100


# ---------------------------------------------------------------------------
# Child nodes — add MapGeneratorNode and HexRenderer as children in the
# scene tree, then assign them here via @onready or @export.
# ---------------------------------------------------------------------------
@onready var generator: MapGeneratorNode = $MapGeneratorNode
@onready var renderer: HexRenderer = $HexRenderer   # HexRenderer.gd node
@onready var debug_overlay: HexDebugOverlay = $HexDebugOverlay

# Held as a reference after generation — read-only, for editor tools / UI.
var map_data: MapDataRef = null
var _last_render_data: Array = []


func _ready() -> void:
	renderer.pointy_orientation = self.pointy_orientation
	generator.map_ready.connect(_on_map_ready)
	generator.generate_map(
		rng_seed,
		pointy_orientation,
		origin,
		map_scale,
		limits,
	)


# ---------------------------------------------------------------------------
# Signal handler — runs on the main thread, safe to touch Godot nodes.
# ---------------------------------------------------------------------------
func _on_map_ready(p_map_data: MapDataRef, render_data: Array) -> void:
	map_data = p_map_data
	_last_render_data = render_data

	renderer.render_map(render_data)
	
	if OS.is_debug_build() and debug_overlay:
		debug_overlay.draw(
			map_data,
			render_data,
			pointy_orientation,
			HexRenderer.ELEVATION_STEP,
		)

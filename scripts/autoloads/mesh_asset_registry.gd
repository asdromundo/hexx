## MeshAssetRegistry.gd
## Autoload singleton — add as "MeshAssetRegistry" in Project > Autoloads.
##
## RESPONSIBILITIES (Godot side only):
##   - Map namespaced string keys  →  preloaded PackedScene handles.
##   - Support data-pack / mod overrides loaded at runtime.
##   - Instantiate scenes on demand for the renderer.
##
## NOT RESPONSIBLE FOR:
##   - Which key belongs to which biome pair (Rust owns that).
##   - Spatial variant selection (Rust owns that).
##   - Any game logic (Rust owns that).
##
## USAGE:
##   var scene = MeshAssetRegistry.get_scene("base:grass_flat_a")
##   var node  = MeshAssetRegistry.instantiate("base:grass_flat_a")
extends Node

# ---------------------------------------------------------------------------
# Internal registry: key -> PackedScene
# Populated by _build_base_registry() and overridden by load_pack().
# ---------------------------------------------------------------------------
var _registry: Dictionary = {}

# ---------------------------------------------------------------------------
# Fallback scene shown when a key has no registered asset.
# Replace with your actual missing-mesh placeholder.
# ---------------------------------------------------------------------------
const FALLBACK_KEY := "base:missing"


func _ready() -> void:
	_build_base_registry()


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

## Returns the PackedScene for the given namespaced key.
## Returns the fallback scene if the key is not registered.
func get_scene(key: String) -> PackedScene:
	if _registry.has(key):
		return _registry[key]
	push_warning("MeshAssetRegistry: unknown key '%s', using fallback." % key)
	return _registry.get(FALLBACK_KEY)


## Instantiates the scene for the given key and returns the root Node.
## Caller is responsible for adding the node to the scene tree.
func instantiate(key: String) -> Node:
	var scene := get_scene(key)
	if scene == null:
		push_error("MeshAssetRegistry: fallback scene is also null for key '%s'." % key)
		return null
	return scene.instantiate()


## Returns true if the key has a registered asset (excluding fallback logic).
func has_key(key: String) -> bool:
	return _registry.has(key)


## Returns all currently registered keys.
## Useful for pre-warming, debugging, or exposing to a mod inspector UI.
func registered_keys() -> Array:
	return _registry.keys()


# ---------------------------------------------------------------------------
# Data-pack / mod support
#
# To ship a data pack:
#   1. Export a .pck file from Godot containing replacement .glb scenes.
#   2. Call load_pack("res://mods/volcanic_pack.pck") at runtime.
#   3. The pack registers its assets under its own namespace (e.g. "volcanic:lava_flat_a").
#      Any key matching an existing base key will OVERRIDE it.
#
# Packs are layered: last loaded pack with a given key wins.
# ---------------------------------------------------------------------------

## Loads a .pck data pack and merges its asset manifest into the registry.
## `manifest_path` is a path inside the pack to a JSON manifest file.
##
## Manifest format:
##   { "base:grass_flat_a": "res://mods/volcanic/grass_flat_a.glb", ... }
func load_pack(pck_path: String, manifest_path: String) -> bool:
	if not ProjectSettings.load_resource_pack(pck_path):
		push_error("MeshAssetRegistry: failed to load pack '%s'." % pck_path)
		return false

	var manifest := _load_json_manifest(manifest_path)
	if manifest.is_empty():
		push_warning("MeshAssetRegistry: pack loaded but manifest is empty: %s" % manifest_path)
		return true

	var overrides := 0
	var additions := 0

	for key in manifest.keys():
		var path: String = manifest[key]
		if not ResourceLoader.exists(path):
			push_warning("MeshAssetRegistry: asset not found for key '%s': %s" % [key, path])
			continue

		var scene := load(path) as PackedScene
		if scene == null:
			push_warning("MeshAssetRegistry: could not load PackedScene at '%s'." % path)
			continue

		if _registry.has(key):
			overrides += 1
		else:
			additions += 1

		_registry[key] = scene

	print("MeshAssetRegistry: pack '%s' loaded — %d overrides, %d new keys." \
		% [pck_path.get_file(), overrides, additions])
	return true


## Removes all overrides and resets to the built-in base registry.
func reset_to_base() -> void:
	_registry.clear()
	_build_base_registry()


# ---------------------------------------------------------------------------
# Base registry — edit this to match your project's actual .glb paths.
# Keys MUST match the string keys emitted by the Rust MeshRegistry.
# ---------------------------------------------------------------------------

func _build_base_registry() -> void:
	# ---- Grass variants ----
	_register("base:grass_flat_a",  "res://assets/meshes/terrain/grass_flat_a.glb")
	_register("base:grass_flat_b",  "res://assets/meshes/terrain/grass_flat_b.glb")

	# ---- Water variants ----
	_register("base:water_flat_a",  "res://assets/meshes/terrain/water_flat_a.glb")
	_register("base:water_flat_b",  "res://assets/meshes/terrain/water_flat_b.glb")

	# ---- Transition: Grass → Water ----
	_register("base:grass_water_a", "res://assets/meshes/terrain/grass_water_a.glb")
	_register("base:grass_water_b", "res://assets/meshes/terrain/grass_water_b.glb")

	# ---- Transition: Water → Grass ----
	_register("base:water_grass_a", "res://assets/meshes/terrain/water_grass_a.glb")
	_register("base:water_grass_b", "res://assets/meshes/terrain/water_grass_b.glb")

	# ---- Fallback (placeholder / error mesh) ----
	_register(FALLBACK_KEY,         "res://assets/meshes/terrain/missing.glb")


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

func _register(key: String, path: String) -> void:
	if not ResourceLoader.exists(path):
		# In editor / development builds this is expected for placeholder paths.
		push_warning("MeshAssetRegistry: asset path not found for key '%s': %s" % [key, path])
		return
	_registry[key] = load(path)


func _load_json_manifest(path: String) -> Dictionary:
	if not FileAccess.file_exists(path):
		push_error("MeshAssetRegistry: manifest not found at '%s'." % path)
		return {}

	var file := FileAccess.open(path, FileAccess.READ)
	if file == null:
		push_error("MeshAssetRegistry: could not open manifest at '%s'." % path)
		return {}

	var json := JSON.new()
	var err := json.parse(file.get_as_text())
	file.close()

	if err != OK:
		push_error("MeshAssetRegistry: JSON parse error in manifest '%s': %s" \
			% [path, json.get_error_message()])
		return {}

	var result = json.get_data()
	if not result is Dictionary:
		push_error("MeshAssetRegistry: manifest root must be a JSON object.")
		return {}

	return result

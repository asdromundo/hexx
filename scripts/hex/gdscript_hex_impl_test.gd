extends Node2D

var hex_grid : Array[Hex]
var hex_grid_corners : Dictionary[int, PackedVector2Array]
var hex_grid_mesh : PackedVector2Array
var hex_layout = HexLayout.new(HexOrientation.POINTY, Vector2(50.0, 50.0), Vector2(555,324))
func _ready() -> void:
	var hex1 : Hex = Hex.new(0,0)
	hex_grid.append(hex1)
	hex_grid.append_array(HexMath.neighbors(hex1))
	for i in range(len(hex_grid)):
		var corners := HexMath.hex_corners(hex_layout, hex_grid[i])
		hex_grid_corners[i] = corners
		hex_grid_mesh.append_array(corners)
	print(hex_grid_corners)

func _draw() -> void:
	#draw_polygon(hex_grid_mesh, [Color.DARK_ORANGE])
	#draw_polyline(hex_grid_mesh, Color.DARK_GREEN)
	for i in range(len(hex_grid)):
		_draw_hex(i)

func _draw_hex(idx: int) -> void:
	var corner := hex_grid_corners[idx]
	draw_polygon(corner.slice(0,6), [Color.DARK_CYAN])
	draw_polyline(corner, Color.AQUA)

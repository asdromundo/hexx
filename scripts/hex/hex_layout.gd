## Hexagonal grid layout.
## Represents the layout of a hexagonal grid, including the orientation of the hexagons, their size, and the origin point for coordinate conversions.
## Aims to represent a struct with const fields, but GDScript does not support this directly. Instead, we use getter methods to provide read-only access to the properties.
class_name HexLayout
extends RefCounted

var _orientation : HexOrientation
## Orientation of the hexagons in the grid, defined by the `HexOrientation` class.
var orientation : HexOrientation:
	get:
		return _orientation

var _size: Vector2
## Size of the hexagons, represented as a `Vector2` where `x` is the width and `y` is the height.
var size: Vector2:
	get:
		return _size

var _origin: Vector2
## Origin point for coordinate conversions, represented as a `Vector2`. This is the point in pixel space that corresponds to the hexagonal coordinate (0, 0).
var origin: Vector2:
	get:
		return _origin

## Initialize the `HexLayout` with the given orientation, size, and origin.
func _init(h_orientation: HexOrientation, h_size: Vector2, h_origin: Vector2 = Vector2.ZERO):
	_orientation = h_orientation
	_size = h_size
	_origin = h_origin

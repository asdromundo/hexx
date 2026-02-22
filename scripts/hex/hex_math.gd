## Mathematical utilities for hexagonal grid operations.
##
## Provides static methods for coordinate conversions, neighbor calculations, and geometric operations
## on hexagonal grids using axial coordinate systems.
class_name HexMath

## Get the direction vector for a given direction index (0-5).
##
## Direction indexing follows the standard hex grid convention:
## - 0: East
## - 1: Southeast
## - 2: Southwest
## - 3: West
## - 4: Northwest
## - 5: Northeast
##
## [param dir] must be in range [0, 5].
## [returns] A [Hex] direction vector.
static func direction(dir: int) -> Hex:
	assert(0 <= dir && dir <= 6)
	return Hex.DIRECTIONS[dir]

## Get all six neighbors of a hexagon.
##
## [param hex] The hexagon to get neighbors for.
## [returns] An [Array] of [Hex] coordinates for all adjacent hexagons.
static func neighbors(hex: Hex) -> Array[Hex]:
	var result: Array[Hex] = []
	for dir in Hex.DIRECTIONS:
		result.append(hex.add(dir))
	return result

## Convert hexagonal coordinates to pixel coordinates.
##
## Uses the orientation and size information from the [HexLayout] to transform
## axial hex coordinates into a 2D point in pixel space.
##
## [param layout] The [HexLayout] containing orientation, size, and origin info.
## [param hex] The hexagonal coordinate to convert.
## [returns] A [Vector2] representing the pixel position of the hex center.
static func hex_to_point2d(layout: HexLayout, hex: Hex) -> Vector2:
	var M := layout.orientation
	var x = (M.f0 * hex.q + M.f1 * hex.r) * layout.size.x
	var y = (M.f2 * hex.q + M.f3 * hex.r) * layout.size.y
	return Vector2(x + layout.origin.x, y + layout.origin.y)

## Round floating-point axial coordinates to the nearest valid hex.
##
## Uses the axial rounding algorithm to find the nearest hex coordinate
## from potentially fractional values.
##
## [param x] The fractional q-coordinate.
## [param y] The fractional r-coordinate.
## [returns] A [Vector2i] with rounded integer hex coordinates.
static func axial_round(x: float, y: float) -> Vector2i:
	var xgrid := roundi(x)
	var ygrid := roundi(y)

	x -= xgrid
	y -= ygrid

	if abs(x) >= abs(y):
		return Vector2i(
			xgrid + roundi(x + 0.5 * y),
			ygrid
		)
	else:
		return Vector2i(
			xgrid,
			ygrid + roundi(y + 0.5 * x)
		)

## Convert pixel coordinates to the nearest hexagonal coordinate.
##
## Transforms a 2D point in pixel space back into axial hex coordinates,
## rounding to the nearest valid hex.
##
## [param layout] The [HexLayout] containing orientation, size, and origin info.
## [param p] The pixel position to convert.
## [returns] A [Hex] representing the nearest hexagonal coordinate.
static func point2d_to_hex(layout: HexLayout, p: Vector2) -> Hex:
	var M := layout.orientation
	var pt = Vector2((p.x - layout.origin.x) / layout.size.x, (p.y - layout.origin.y) / layout.size.y)
	var q = M.b0 * pt.x + M.b1 * pt.y
	var r = M.b2 * pt.x + M.b3 * pt.y
	var hex_coords := RHexMath.axial_round(q, r)
	return Hex.new(hex_coords.x, hex_coords.y)

## Calculate the pixel offset for a corner of a hexagon.
##
## Computes the offset vector from the hex center to a specific corner,
## taking into account the layout's orientation.
##
## [param layout] The [HexLayout] containing orientation and size info.
## [param corner] The corner index (0-5), where 0 is the first corner.
## [returns] A [Vector2] offset from the hex center to the specified corner.
static func hex_corner_offset(layout: HexLayout, corner: int) -> Vector2:
	var angle : float = 2.0 * PI * (layout.orientation.start_angle - corner) / 6.0
	return Vector2(layout.size.x * cos(angle), layout.size.y * sin(angle))

## Get all corners of a hexagon as pixel coordinates.
##
## Generates the 6 corner positions for a hexagon in pixel space.
## The array includes a 7th element that repeats the first corner to close the polygon.
##
## [param layout] The [HexLayout] containing orientation, size, and origin info.
## [param hex] The hexagon to get corners for.
## [returns] A [PackedVector2Array] with 7 corner positions (6 unique + closing point).
static func hex_corners(layout: HexLayout, hex: Hex) -> PackedVector2Array:
	var corners: PackedVector2Array = []
	corners.resize(7) # 6 corners + 1 to close the loop
	var center = hex_to_point2d(layout, hex)
	for i in range(6):
		var offset = hex_corner_offset(layout, i)
		corners[i] = center + offset
	corners[6] = corners[0] # Close the loop for drawing
	return corners

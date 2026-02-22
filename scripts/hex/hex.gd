## Hex (axial coordinates)
##
## Implements axial storage `(q, r)` with a computed cube coordinate
## `s()` when needed. Follows the Red Blob Games reference:
## https://www.redblobgames.com/grids/hexagons/implementation.html#hex
##
## Using axial storage keeps state minimal and matches the public
## API used across the project. Methods return new `Hex` instances
## to preserve value-like semantics.
class_name Hex
extends RefCounted

## Axial coordinate q.
var q: int

## Axial coordinate r.
var r: int

## Compute the cube-coordinate `s`.
## Returns: s = -q - r
var s: int:
	get:
		return -q-r

## Initialize axial coordinates (`q`, `r`).
func _init(_q: int, _r: int):
	q = _q
	r = _r

## Component-wise equality.
func equals(other : Hex) -> bool:
	return q == other.q && r == other.r

## Component-wise addition.
##
## Returns a new `Hex` equal to `self + other`.
func add(other: Hex) -> Hex:
	return Hex.new(q + other.q, r + other.r)

## Component-wise subtraction.
##
## Returns a new `Hex` equal to `self - other`.
func subtract(other: Hex) -> Hex:
	return Hex.new(q - other.q, r - other.r)

## Scale both coordinates by integer `k`.
##
## Returns a new `Hex`.
func scale(k: int) -> Hex:
	return Hex.new(q * k, r * k)

## Length (distance from origin) using cube coordinates formula:
## (|q| + |r| + |s|) / 2
func length() -> int:
	return int((abs(q) + abs(r) + abs(s)) / 2)

## Hex distance measured as the length of the difference vector.
##
## Returns an integer distance between `self` and `other`.
func distance(other: Hex) -> int:
	return subtract(other).length()

static var DIRECTIONS = [
	Hex.new(1, 0),
	Hex.new(1, -1),
	Hex.new(0, -1),
	Hex.new(-1, 0),
	Hex.new(-1, 1),
	Hex.new(0, 1)
]

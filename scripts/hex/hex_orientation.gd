class_name HexOrientation

var _f0: float
var _f1: float
var _f2: float
var _f3: float
var _b0: float
var _b1: float
var _b2: float
var _b3: float
var _start_angle: float

var f0: float:
	get:
		return _f0
var f1: float:
	get:
		return _f1
var f2: float:
	get:
		return _f2
var f3: float:
	get:
		return _f3
var b0: float:
	get:
		return _b0
var b1: float:
	get:
		return _b1
var b2: float:
	get:
		return _b2
var b3: float:
	get:
		return _b3
var start_angle: float:
	get:
		return _start_angle

static var POINTY := HexOrientation.new(
	sqrt(3.0), sqrt(3.0)/2.0, 0.0, 3.0/2.0,
	sqrt(3.0)/3.0, -1.0/3.0, 0.0, 2.0/3.0,
	0.5
)

static var FLAT := HexOrientation.new(
	3.0/2.0, 0.0, sqrt(3.0)/2.0, sqrt(3.0),
	2.0/3.0, 0.0, -1.0/3.0, sqrt(3.0)/3.0,
	0.0
)

func _init(
	forward_f0: float, forward_f1: float, forward_f2: float, forward_f3: float,
	backward_b0: float, backward_b1: float, backward_b2: float, backward_b3: float,
	orientation_start_angle: float
):
	_f0 = forward_f0
	_f1 = forward_f1
	_f2 = forward_f2
	_f3 = forward_f3
	_b0 = backward_b0
	_b1 = backward_b1
	_b2 = backward_b2
	_b3 = backward_b3
	_start_angle = orientation_start_angle

extends Game

func _ready() -> void:
	print("GdScript Ready")
	var world_2d : Node2D = Node2D.new()
	add_child(world_2d)
	var player: Player = Player.new()
	player.texture = load("res://icon.svg")
	player.scale = Vector2(3,3)
	player.position = Vector2(567,304)
	world_2d.add_child(player)

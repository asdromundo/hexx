# Project roadmap

```
tactical-rpg/
│
├── godot/
│   ├── scenes/
│   │   ├── battle/
│   │   │   ├── BattleRoot.tscn
│   │   │   ├── HexGrid.tscn
│   │   │   ├── Units.tscn
│   │   │   └── CameraRig.tscn
│   │   │
│   │   └── ui/
│   │       ├── ActionMenu.tscn
│   │       └── HUD.tscn
│   │
│   ├── scripts/
│   │   ├── grid/
│   │   │   ├── hex_coord.gd
│   │   │   ├── hex_layout.gd
│   │   │   ├── hex_grid_logic.gd
│   │   │   └── pathfinding.gd
│   │   │
│   │   ├── units/
│   │   │   ├── unit.gd
│   │   │   └── unit_visual.gd
│   │   │
│   │   └── battle/
│   │       ├── battle_controller.gd
│   │       └── input_controller.gd
│   │
│   └── addons/
│       └── rust_core/   (GDExtension cuando migres)
│
└── grust/
	├── core/
	│   ├── grid/
	│   │   ├── coord.rs
	│   │   ├── layout.rs
	│   │   ├── neighbors.rs
	│   │   └── pathfinding.rs
	│   │
	│   ├── units/
	│   │   ├── components.rs
	│   │   └── stats.rs
	│   │
	│   └── battle/
	│       └── movement.rs
	│
	└── gdextension/
		└── lib.rs

```

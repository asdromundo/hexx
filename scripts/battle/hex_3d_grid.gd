extends Hex3DGrid

func _ready():
	# 1. Create a MeshInstance3D node
	var mesh_instance = MeshInstance3D.new()
	add_child(mesh_instance) # Add it to the scene tree

	# 2. Create a Primitive Mesh resource (e.g., a BoxMesh)
	var box_mesh = BoxMesh.new()
	# Optional: configure the mesh properties (size, material, etc.)
	# box_mesh.size = Vector3(2, 1, 0.5)

	# 3. Assign the mesh resource to the MeshInstance3D
	mesh_instance.mesh = box_mesh
	
	# Ensure the scale is not zero, as it is by default for new nodes
	mesh_instance.scale = Vector3.ONE 

	# Optional: create a simple material
	var material = StandardMaterial3D.new()
	material.albedo_color = Color(0.9, 0.1, 0.1) # Red color
	box_mesh.surface_set_material(0, material) # Assign material to the first surface

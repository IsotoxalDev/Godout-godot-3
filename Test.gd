extends Node

# Run test cases
func _ready():
	var dir = Directory.new()
	dir.open("res://Tests")
	dir.list_dir_begin(true)
	var file_name = dir.get_next()
	while file_name != "":
		if dir.current_is_dir():
			print("Testing %s Codecs" % file_name)
		file_name = dir.get_next()

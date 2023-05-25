extends Node

const TEST_DIR : = "res://Tests/"

# Run test cases
func _ready():
	var dir = Directory.new()
	dir.open(TEST_DIR)
	dir.list_dir_begin(true)
	var file_name = dir.get_next()
	while file_name != "":
		if dir.current_is_dir():
			print("Testing %s Codecs" % file_name)
			load(TEST_DIR + file_name + "/Test.gd")._test()
		file_name = dir.get_next()

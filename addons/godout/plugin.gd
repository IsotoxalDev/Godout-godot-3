tool
extends EditorPlugin

func _enter_tree():
	var godout_script = "extends Node\n"
	var dir = Directory.new()
	dir.open("res://addons/godout/")
	dir.list_dir_begin(true)
	var file_name = dir.get_next()
	while file_name != "":
		if dir.current_is_dir():
			var sub_dir = Directory.new()
			var curr_dir = dir.get_current_dir()+file_name
			sub_dir.open(curr_dir)
			sub_dir.list_dir_begin(true)
			var enter = "func _enter_tree():"
			var out = sub_dir.get_next()
			while out != "":
				godout_script += "const %s_%s = preload(\"%s/%s/%s.gdns\")\n" % [file_name, out, curr_dir, out, out]
				enter += "\n\t%s[\"%s\"] = %s_%s.new()" % [file_name, out, file_name, out]
#				enter += "\n\tnode = Node.new()\n\tnode.set_script(%s_%s)\n\tadd_child(node)\n\t%s.%s = node" % [file_name, out, file_name, out]
				out = sub_dir.get_next()
			godout_script += "var %s = {}\n%s" % [file_name, enter]
			var script = File.new()
			script.open("res://addons/godout/godout.gd", File.WRITE)
			script.store_string(godout_script)
			script.close()
		file_name = dir.get_next()
	add_autoload_singleton("Godout", "res://addons/godout/godout.gd")

func _exit_tree():
	remove_autoload_singleton("Godout")

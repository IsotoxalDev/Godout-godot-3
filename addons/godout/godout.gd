extends Node
var animation = {}
func _enter_tree():
	animation["gif"] = load("res://addons/godout/animation/gif/gif.gdns").new()
	animation["webp"] = load("res://addons/godout/animation/webp/webp.gdns").new()

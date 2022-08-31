extends Node
const animation_gif = preload("res://addons/godout/animation/gif/gif.gdns")
const animation_webp = preload("res://addons/godout/animation/webp/webp.gdns")
var animation = {}
func _enter_tree():
	animation["gif"] = animation_gif.new()
	animation["webp"] = animation_webp.new()

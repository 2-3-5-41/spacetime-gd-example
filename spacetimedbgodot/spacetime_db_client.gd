extends SpacetimeDbClient

@onready var input: TextEdit = $Panel/HBoxContainer/TextEdit

func _on_button_pressed() -> void:
	update_status(input.text)

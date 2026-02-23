use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Player inicializado!");
        
        Self {
            base,
        }
    }

}



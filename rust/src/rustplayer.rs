use std::str::FromStr;
use godot::classes::{ character_body_2d, CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;




#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Rustplayer{
    base: Base<CharacterBody2D>,
    
}


#[godot_api]
impl ICharacterBody2D for Rustplayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("hi");
        Self {base, }
    }

    
    fn process(&mut self, _delta:f64){
        let speed: f32 = 100.0;

        let input = Input::singleton();

        let direction = Input::get_vector(&input, &StringName::from_str("left").unwrap(), &StringName::from_str("right").unwrap(), &StringName::from_str("up").unwrap(), &StringName::from_str("down").unwrap());
       
        let velocity = direction * speed;
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        




        
    }
    

    
}



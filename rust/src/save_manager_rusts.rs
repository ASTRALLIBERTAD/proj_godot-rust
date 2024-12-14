use std::borrow::Borrow;

use bincode::serialize;
use godot::classes::file_access::ModeFlags;
use godot::classes::{ node_2d, CharacterBody2D, DirAccess, FileAccess, Node};
use godot::prelude::*;

use serde::{Deserialize, Serialize};


use crate::rustplayer::Rustplayer;

#[derive(Serialize, Deserialize)]
struct PlayerPosition {
    x: f32,
    y: f32,
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct SaveManagerRust {
    #[base]
    base: Base<Node>,
    player_node_rust: Option<Gd<Rustplayer>>,
}

#[godot_api]
impl SaveManagerRust {



    #[func]
    fn set_player_node(&mut self, player: Gd<Rustplayer>) {
        self.player_node_rust = Some(player);
    }

    #[func]
    fn save_game_rust(&self, name: String) {
        let base_path = "user://";
        let folder = "games";
        let file_saver = "user://games";
        let name = name;
        let games_path = format!("{}/{}/{}", base_path, folder, name);
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);
        
        let file  = FileAccess::open(&save_path, ModeFlags::WRITE);     
           
        let mut dir = DirAccess::open(base_path).expect("okkk"); 
 
        if !dir.dir_exists(folder) {
                dir.make_dir(folder);
            } 
        
        dir = DirAccess::open(file_saver).expect("not opened");

        if !dir.dir_exists(&name){
            dir.make_dir(&name);
        }

        if dir != DirAccess::open(&games_path).expect("failed to open"){
            return;
        }

        if file.is_none() {
            godot_error!("Failed to open file at {}", save_path);
            return;
        }

        if let Some(mut file) = file {
            if let Some(player) = &self.player_node_rust {
                let position = player.get_global_position();
                let player_position = PlayerPosition {
                    x: position.x,
                    y: position.y,
                };
    
                // Serialize the position
                if let Ok(serialized_data) = bincode::serialize(&player_position) {
                    // Write the serialized data to the file
                    file.store_buffer(serialized_data.as_slice());
                } else {
                    godot_error!("Failed to serialize player position");
                }
            } else {
                godot_error!("Player node is not set");
            }
        } else {
            godot_error!("Failed to open file at {}", save_path);
        }
    
    }
}
                    
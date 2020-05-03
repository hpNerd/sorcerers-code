use amethyst::core::transform::Transform;

pub struct Player {
    pub player_type: String,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Player {
    pub fn new(player_type: String) -> Player {
        Player {
            player_type,
            pos_x: 50.0,
            pos_y: 50.0,
        }
    }
}

use crate::player::Player;

#[path ="player.rs"] mod player;

pub struct Team {
  name: String,
  players: Vec<Player>,
}
impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            players: Vec::new(),
        }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_players(&mut self) ->  &mut Vec<Player> {
        &mut self.players
    }
}
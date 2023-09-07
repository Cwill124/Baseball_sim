#[path ="model/player.rs"]
mod player;

#[path="model/team.rs"]
mod team;

use rand::prelude::*;

fn main() {
  let mut atlanta_braves = populate_atlanta_braves(); // Make atlanta_braves mutable
  for player in atlanta_braves.get_players() {
    simulate_season(player,700);
  }
}

fn simulate_season(_player: &mut player::Player, number_of_at_bats: i32) {
    for _i in 0..number_of_at_bats {
        let result = calculate_at_bat(_player);
        if result == 0 {
            _player.increament_at_bats();
            _player.increament_strike_out();
        } else if result == 1 {
            _player.increament_walk();
        } else if result == 3 {
            _player.increament_at_bats();
            _player.increament_hits();
            _player.increament_home_runs();
        } else if result == 2 {
            _player.increament_at_bats();
            _player.increament_hits();
        } else {
            _player.increament_at_bats();
        }
        _player.calculate_batting_average();
    }
    println!("Stats for a number of at bats {}", number_of_at_bats);
    println!(
        "{} had {} at bats, {} strikeouts, {} hits, {} walks, {} home runs, and a batting average of {}",
        _player.get_name(),
        _player.get_at_bats(),
        _player.get_strike_outs(),
        _player.get_hits(),
        _player.get_walks(),
        _player.get_home_runs(),
        _player.get_batting_average()
    );
    println!("-----------------------------------------------------------------------");
}

fn calculate_at_bat(_player: &mut player::Player) -> u8 {
    let mut rng = rand::thread_rng();
    let random = rng.gen::<f32>();
    let walk_threshold = _player.get_walk_percentage() + _player.get_strike_out_percentage();

    if random <= _player.get_strike_out_percentage() {
        return 0;
    } else if random <= walk_threshold {
        return 1;
    } else if random > 0.99 {
        return 3;
    } else {
        return calculate_if_hit_is_an_out(_player);
    }
}

fn calculate_if_hit_is_an_out(_player: &mut player::Player) -> u8 {
  let mut rng = rand::thread_rng();
  let mut random = rng.gen::<f32>();
  let hard_hit_threshold = _player.get_hard_hit_percentage();
  if random <= hard_hit_threshold {
    random = rng.gen::<f32>();
    if random <= _player.get_home_run_percentage(){
      return 3;
    } else if random > _player.get_home_run_percentage() && random <= 0.57 {
      return 2;
    } else {
      return 4;
    }
  } else {
    random = rng.gen::<f32>();
    if random < 0.07 {
      return 2;
    } else {
      return 4;
    }
  }

}

fn populate_atlanta_braves() -> team::Team {
    let mut _atlanta_braves = team::Team::new(String::from("Atlanta Braves"));
    let mut _ronald_acuna = player::Player::new(String::from("Ronald Acuna Jr."), 0.118, 0.0563, 0.11,0.451);
    let mut _matt_olson = player::Player::new(String::from("Matt Olson"), 0.248, 0.063, 0.143,0.571);
    let mut _sean_murphy = player::Player::new(String::from("Sean Murphy"), 0.225, 0.051, 0.105,0.463);
    let mut _ozzie_albies = player::Player::new(String::from("Ozzie Albies"), 0.164, 0.053, 0.068,0.377);
    let mut _orlando_arica = player::Player::new(String::from("Orlando Arcia"), 0.186, 0.039, 0.070,0.437);
    let mut _austin_riley = player::Player::new(String::from("Austin Riley"), 0.234, 0.054, 0.079,0.484);
    let mut _eddie_rosario = player::Player::new(String::from("Eddie Rosario"), 0.237, 0.046, 0.078,0.36);
    let mut _micheal_harris = player::Player::new(String::from("Micheal Harris II"), 0.187, 0.03, 0.057,0.476);
    let mut _marcell_ozuna = player::Player::new(String::from("Marcell Ozuna"), 0.231, 0.067, 0.10,0.489);

    _atlanta_braves.add_player(_ronald_acuna);
    _atlanta_braves.add_player(_matt_olson);
    _atlanta_braves.add_player(_sean_murphy);
    _atlanta_braves.add_player(_ozzie_albies);
    _atlanta_braves.add_player(_orlando_arica);
    _atlanta_braves.add_player(_austin_riley);
    _atlanta_braves.add_player(_eddie_rosario);
    _atlanta_braves.add_player(_micheal_harris);
    _atlanta_braves.add_player(_marcell_ozuna);

    return _atlanta_braves;
}

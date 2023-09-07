pub struct Player {
  name: String,
  at_bats: f32,
  hits: f32,
  batting_average: f32,
  walk: f32,
  home_runs: f32,
  strike_outs: f32,
  strike_out_percentage: f32,
  home_run_percentage: f32,
  walk_percentage: f32,
  hard_hit_percentage: f32,
}
impl Player {
  pub fn new(name: String,strike_percent:f32,home_run_percent:f32,walk_percent:f32,hard_hit_percentage:f32) -> Player {
    Player {
      name,
      at_bats: 0.0,
      hits: 0.0,
      batting_average: 0.0,
      walk: 0.0,
      home_runs: 0.0,
      strike_outs: 0.0,
      strike_out_percentage: strike_percent,
      home_run_percentage: home_run_percent,
      walk_percentage: walk_percent,
      hard_hit_percentage: hard_hit_percentage,
    }
  }
  pub fn get_name(&self) -> &String {
    &self.name
  }
  pub fn get_at_bats(&self) -> f32 {
    self.at_bats
  }
  pub fn get_hits(&self) -> f32 {
    self.hits
  }
  pub fn get_batting_average(&self) -> f32 {
    self.batting_average
  }
  pub fn increament_at_bats(&mut self) {
    self.at_bats += 1.0;
  }
  pub fn increament_hits(&mut self) {
    self.hits += 1.0;
  }
  pub fn increament_home_runs(&mut self) {
    self.home_runs += 1.0;
  }
  pub fn get_strike_out_percentage(&self) -> f32 {
    self.strike_out_percentage
  }
  pub fn get_home_run_percentage(&self) -> f32 {
    self.home_run_percentage
  }
  pub fn get_walk_percentage(&self) -> f32 {
    self.walk_percentage
  }
  pub fn get_walks(&self) -> f32 {
    self.walk
  }
  pub fn get_home_runs(&self) -> f32 {
    self.home_runs
  }
  pub fn get_strike_outs(&self) -> f32 {
    self.strike_outs
  }
  pub fn increament_walk(&mut self) {
    self.walk += 1.0;
  }
  pub fn increament_strike_out(&mut self) {
    self.strike_outs += 1.0;
  }
  pub fn get_hard_hit_percentage(&self) -> f32 {
    self.hard_hit_percentage
  }
  pub fn calculate_batting_average(&mut self) -> f32 {
    if self.at_bats == 0.0 {
      self.batting_average = 0.0;
      return 0.0;
    }
    let batting_average = self.hits / self.at_bats;
    self.batting_average = batting_average;
    return batting_average;
  }
}
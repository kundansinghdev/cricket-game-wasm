use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use rand::Rng;
use web_sys::{Audio, HtmlElement, HtmlButtonElement, Document, Window};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    game_started: bool,
    toss_completed: bool,
    current_player: u8,
    scores: [u32; 2],
    balls: [u32; 2],
    wickets: [u32; 2],
    over: u32,
    ball: u32,
    is_batting: bool,
    game_over: bool,
    winner: Option<u8>,
    events: Vec<GameEvent>,
    target_score: u32,
    required_runs: u32,
    player_names: [String; 2],
    match_type: MatchType,
    difficulty: Difficulty,
    power_plays: [u32; 2],
    player_stats: [PlayerStats; 2],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStats {
    runs: u32,
    balls_faced: u32,
    fours: u32,
    sixes: u32,
    strike_rate: f32,
    wickets_taken: u32,
    economy_rate: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum MatchType {
    T20,
    ODI,
    Test,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameEvent {
    message: String,
    event_type: String,
    timestamp: u64,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        GameState {
            game_started: false,
            toss_completed: false,
            current_player: 1,
            scores: [0, 0],
            balls: [0, 0],
            wickets: [0, 0],
            over: 0,
            ball: 0,
            is_batting: true,
            game_over: false,
            winner: None,
            events: Vec::new(),
            target_score: 0,
            required_runs: 0,
            player_names: ["Player 1".to_string(), "Player 2".to_string()],
            match_type: MatchType::T20,
            difficulty: Difficulty::Medium,
            power_plays: [0, 0],
            player_stats: [
                PlayerStats {
                    runs: 0,
                    balls_faced: 0,
                    fours: 0,
                    sixes: 0,
                    strike_rate: 0.0,
                    wickets_taken: 0,
                    economy_rate: 0.0,
                },
                PlayerStats {
                    runs: 0,
                    balls_faced: 0,
                    balls_faced: 0,
                    fours: 0,
                    sixes: 0,
                    strike_rate: 0.0,
                    wickets_taken: 0,
                    economy_rate: 0.0,
                },
            ],
        }
    }

    pub fn set_match_type(&mut self, match_type: String) {
        match match_type.as_str() {
            "T20" => self.match_type = MatchType::T20,
            "ODI" => self.match_type = MatchType::ODI,
            "Test" => self.match_type = MatchType::Test,
            _ => {}
        }
    }

    pub fn set_difficulty(&mut self, difficulty: String) {
        match difficulty.as_str() {
            "Easy" => self.difficulty = Difficulty::Easy,
            "Medium" => self.difficulty = Difficulty::Medium,
            "Hard" => self.difficulty = Difficulty::Hard,
            _ => {}
        }
    }

    pub fn set_player_names(&mut self, player1: String, player2: String) {
        self.player_names[0] = player1;
        self.player_names[1] = player2;
    }

    pub fn handle_toss(&mut self) {
        if !self.toss_completed {
            let toss = if rand::thread_rng().gen_bool(0.5) { "Heads" } else { "Tails" };
            let winner = if rand::thread_rng().gen_bool(0.5) { 1 } else { 2 };
            
            self.play_sound("toss");
            self.toss_completed = true;
            self.current_player = winner;
            
            self.add_event(&format!("Toss: {} - {} wins the toss!", toss, self.player_names[(winner - 1) as usize]));
        }
    }

    pub fn handle_start(&mut self) {
        if !self.game_started && self.toss_completed {
            self.game_started = true;
            self.add_event(&format!("Game started! {} starts batting!", self.player_names[(self.current_player - 1) as usize]));
            
            // Set target based on match type
            match self.match_type {
                MatchType::T20 => self.target_score = 160,
                MatchType::ODI => self.target_score = 250,
                MatchType::Test => self.target_score = 300,
            }
            
            self.add_event(&format!("Target score: {}", self.target_score));
        }
    }

    fn get_random_outcome(&self) -> (i32, &'static str) {
        let (outcomes, probabilities) = match self.difficulty {
            Difficulty::Easy => (
                vec![
                    (0, "Dot ball"),
                    (1, "Single"),
                    (2, "Double"),
                    (4, "Four!"),
                    (6, "Six!"),
                    (-1, "Wicket!"),
                ],
                vec![0.2, 0.2, 0.15, 0.25, 0.15, 0.05],
            ),
            Difficulty::Medium => (
                vec![
                    (0, "Dot ball"),
                    (1, "Single"),
                    (2, "Double"),
                    (4, "Four!"),
                    (6, "Six!"),
                    (-1, "Wicket!"),
                ],
                vec![0.25, 0.2, 0.15, 0.2, 0.1, 0.1],
            ),
            Difficulty::Hard => (
                vec![
                    (0, "Dot ball"),
                    (1, "Single"),
                    (2, "Double"),
                    (4, "Four!"),
                    (6, "Six!"),
                    (-1, "Wicket!"),
                ],
                vec![0.3, 0.2, 0.15, 0.15, 0.1, 0.1],
            ),
        };

        let mut rng = rand::thread_rng();
        let random = rng.gen::<f64>();
        let mut cumulative_probability = 0.0;

        for ((runs, description), probability) in outcomes.iter().zip(probabilities.iter()) {
            cumulative_probability += probability;
            if random <= cumulative_probability {
                return (*runs, *description);
            }
        }

        (0, "Dot ball")
    }

    pub fn handle_bat(&mut self) {
        if self.game_started && self.is_batting {
            let (runs, description) = self.get_random_outcome();
            let player_idx = (self.current_player - 1) as usize;
            
            self.balls[player_idx] += 1;
            self.player_stats[player_idx].balls_faced += 1;

            if runs == -1 {
                self.play_sound("wicket");
                self.wickets[player_idx] += 1;
                self.is_batting = false;
                self.current_player = if self.current_player == 1 { 2 } else { 1 };
                self.add_event(&format!("{} is out! {}", self.player_names[player_idx], description));
            } else {
                self.scores[player_idx] += runs as u32;
                self.player_stats[player_idx].runs += runs as u32;
                
                match runs {
                    4 => {
                        self.play_sound("boundary");
                        self.player_stats[player_idx].fours += 1;
                    },
                    6 => {
                        self.play_sound("six");
                        self.player_stats[player_idx].sixes += 1;
                    },
                    _ => self.play_sound("bat"),
                }
                
                self.player_stats[player_idx].strike_rate = 
                    (self.player_stats[player_idx].runs as f32 / self.player_stats[player_idx].balls_faced as f32) * 100.0;
                
                self.add_event(&format!(
                    "{} scores {} run{}! {}",
                    self.player_names[player_idx],
                    runs,
                    if runs != 1 { "s" } else { "" },
                    description
                ));
            }

            self.update_ball_count();
        }
    }

    pub fn handle_bowl(&mut self) {
        if self.game_started && !self.is_batting {
            let player_idx = (self.current_player - 1) as usize;
            self.balls[player_idx] += 1;
            
            // Calculate economy rate
            let overs = self.balls[player_idx] as f32 / 6.0;
            self.player_stats[player_idx].economy_rate = 
                self.scores[if player_idx == 0 { 1 } else { 0 }] as f32 / overs;
            
            self.add_event(&format!("{} bowls a delivery", self.player_names[player_idx]));
            self.update_ball_count();
        }
    }

    fn update_ball_count(&mut self) {
        self.ball += 1;
        if self.ball >= 6 {
            self.ball = 0;
            self.over += 1;
            self.add_event(&format!("Over {} completed!", self.over));
            
            if self.is_batting {
                self.is_batting = false;
                self.current_player = if self.current_player == 1 { 2 } else { 1 };
                self.add_event(&format!("{} starts bowling", self.player_names[(self.current_player - 1) as usize]));
            } else {
                self.is_batting = true;
                self.current_player = if self.current_player == 1 { 2 } else { 1 };
                self.add_event(&format!("{} starts batting", self.player_names[(self.current_player - 1) as usize]));
                
                // Check if game is over based on match type
                let max_overs = match self.match_type {
                    MatchType::T20 => 20,
                    MatchType::ODI => 50,
                    MatchType::Test => 90,
                };
                
                if self.over >= max_overs {
                    self.game_over = true;
                    if self.scores[0] > self.scores[1] {
                        self.winner = Some(1);
                    } else if self.scores[1] > self.scores[0] {
                        self.winner = Some(2);
                    }
                    self.play_sound("win");
                }
            }
        }
    }

    fn play_sound(&self, sound_name: &str) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let audio = document.create_element("audio").unwrap();
        audio.set_attribute("src", &format!("/sounds/{}.mp3", sound_name)).unwrap();
        let audio: Audio = audio.dyn_into().unwrap();
        let _ = audio.play();
    }

    fn add_event(&mut self, message: &str) {
        self.events.insert(0, GameEvent {
            message: message.to_string(),
            event_type: String::new(),
            timestamp: js_sys::Date::now() as u64,
        });
    }

    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }

    pub fn reset(&mut self) {
        *self = GameState::new();
        self.add_event("New game started! Complete the toss to begin.");
    }
}

#[wasm_bindgen]
pub fn init_game() -> GameState {
    GameState::new()
} 
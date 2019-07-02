use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::moves::MoveType;


/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub suggestion: usize,
    pub player_1_successful_suggestions: usize,
    pub player_1_successful_predictions: usize,
    pub player_1_suggestion_attempts: usize,
    pub player_1_prediction_attempts: usize,
    pub player_2_successful_suggestions: usize,
    pub player_2_successful_predictions: usize,
    pub player_2_suggestion_attempts: usize,
    pub player_2_prediction_attempts: usize,
    pub player_1_suggests: bool,
}

impl GameState {
    pub fn initial() -> Self {
        // <<DEVCAMP>> return an initial state of a game
        Self{
            moves: Vec::new(),
            suggestion: 0,
            player_1_successful_suggestions: 0,
            player_1_successful_predictions: 0,
            player_1_suggestion_attempts: 0,
            player_1_prediction_attempts: 0,
            player_2_successful_suggestions: 0,
            player_2_successful_predictions: 0,
            player_2_suggestion_attempts: 0,
            player_2_prediction_attempts: 0,
            player_1_suggests: true,
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP>> return a pretty formatting string representation
        let game_string;
        if self.suggestion == 0 {
            game_string = "Waiting for suggestion."
        } else {
            game_string = "Waiting for prediction."
        }
        format!("{} \n\nplayer 1 record: \n\tsuggestions: {}/{}\n\tpredictions: {}/{}\n player 2 record: \n\tsuggestions: {}/{}\n\tpredictions: {}/{}\n", game_string, self.player_1_successful_suggestions, self.player_1_suggestion_attempts, self.player_1_successful_predictions, self.player_1_prediction_attempts, self.player_2_successful_suggestions, self.player_2_suggestion_attempts, self.player_2_successful_predictions, self.player_2_prediction_attempts)
    }

    pub fn evolve(&self, _game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid
        
        let mut moves = self.moves.clone();
        let mut new_suggestion = self.suggestion;
        let mut p1_suggestion_attempts = self.player_1_suggestion_attempts;
        let mut p1_prediction_attempts = self.player_1_prediction_attempts;
        let mut p1_successful_suggestions = self.player_1_successful_suggestions;
        let mut p1_successful_predictions = self.player_1_successful_predictions;

        let mut p2_successful_suggestions = self.player_2_successful_suggestions;
        let mut p2_successful_predictions = self.player_2_successful_predictions;
        let mut p2_suggestion_attempts = self.player_2_suggestion_attempts;
        let mut p2_prediction_attempts = self.player_2_prediction_attempts;
        
        let mut p1_suggests = self.player_1_suggests;

        moves.push(next_move.clone());

        match next_move.move_type {
            MoveType::Suggest{suggestion} => {
                new_suggestion = suggestion;
            }, 
            MoveType::Swap{} => {
                p1_suggests = !p1_suggests;
            },
            MoveType::Predict{prediction} => {
                let mut success = 0;
                if prediction == self.suggestion {
                    success = 1;
                }

                if p1_suggests {
                    p1_successful_suggestions += success;    
                    p2_successful_predictions += success;
                    p1_suggestion_attempts += 1;
                    p2_prediction_attempts += 1;
                } else {
                    p1_successful_predictions += success;
                    p2_successful_suggestions += success;
                    p1_prediction_attempts += 1;
                    p2_suggestion_attempts += 1;
                }
                new_suggestion = 0;
            }
        }

        GameState {
            moves: moves,
            suggestion: new_suggestion,
            player_1_successful_suggestions: p1_successful_suggestions,
            player_1_successful_predictions: p1_successful_predictions,
            player_1_suggestion_attempts: p1_suggestion_attempts,
            player_1_prediction_attempts: p1_prediction_attempts,
            player_2_successful_suggestions: p2_successful_suggestions,
            player_2_successful_predictions: p2_successful_predictions,
            player_2_suggestion_attempts: p2_suggestion_attempts,
            player_2_prediction_attempts: p2_prediction_attempts,
            player_1_suggests: p1_suggests,
        }
    }

}

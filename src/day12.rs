use std::cmp::PartialEq;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};


#[derive(Debug)]
enum GameState {
    Playing,
    CookieWon,
    MilkWon,
    EndedNoWinner,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameState::Playing, GameState::Playing) => true,
            (GameState::CookieWon, GameState::CookieWon) => true,
            (GameState::MilkWon, GameState::MilkWon) => true,
            (GameState::EndedNoWinner, GameState::EndedNoWinner) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Cookie,
    Milk,
}

impl Tile {
    fn to_emoji(self) -> char {
        match self {
            Tile::Empty => '⬛',
            Tile::Cookie => '🍪',
            Tile::Milk => '🥛'
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tile::Empty, Tile::Empty) => true,
            (Tile::Cookie, Tile::Cookie) => true,
            (Tile::Milk, Tile::Milk) => true,
            _ => false,
        }
    }
}

pub struct Grid {
    tiles: [[Tile; 4]; 4],
}

impl Grid {
    fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 4]; 4],
        }
    }
}

#[derive(Clone)]
pub struct Board {
    grid: Arc<Mutex<Grid>>,
    state: Arc<Mutex<GameState>>,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: Arc::new(Mutex::new(Grid::new())),
            state: Arc::new(Mutex::new(GameState::Playing)),
        }
    }

    fn to_string(&self) -> String {
        let grid = self.grid.lock().unwrap();
        let mut result = String::new();
        for row in &grid.tiles {
            result.push('⬜');
            for &tile in row {
                result.push(tile.to_emoji());
            }
            result.push('⬜');
            result.push('\n');
        }
        result.push_str("⬜⬜⬜⬜⬜⬜\n");

        match self.state.lock().unwrap().deref() {
            GameState::CookieWon => result.push_str("🍪 wins!\n"),
            GameState::MilkWon => result.push_str("🥛 wins!\n"),
            GameState::EndedNoWinner => result.push_str("No winner!\n"),
            GameState::Playing => {},
        };

        result
    }
}

fn get_game_state(grid: &[[Tile; 4]; 4]) -> GameState {
    // check diagonals
    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[2][2] == grid[3][3] && grid[0][0] != Tile::Empty {
         return if grid[0][0] == Tile::Cookie {
             GameState::CookieWon
         } else {
             GameState::MilkWon
         };
    };

    // check rows
    for row in grid.iter() {
        if row[0] == row[1] && row[1] == row[2] && row[2] == row[3] && row[0] != Tile::Empty {
            return if row[0] == Tile::Cookie {
                GameState::CookieWon
            } else {
                GameState::MilkWon
            };
        };
    };

    // check columns
    for i in 0..4 {
        if grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] && grid[2][i] == grid[3][i] && grid[0][i] != Tile::Empty {
            return if grid[0][i] == Tile::Cookie {
                GameState::CookieWon
            } else {
                GameState::MilkWon
            };
        };
    };

    // check if the board is full
    return if grid.iter().all(|row| row.iter().all(|&tile| tile != Tile::Empty)) {
        GameState::EndedNoWinner
    } else {
        GameState::Playing
    }
}

pub async fn get_board(State(board): State<Arc<Board>>) -> impl IntoResponse {
    (StatusCode::OK, board.to_string())
}

pub async fn reset_board(State(board): State<Arc<Board>>) -> impl IntoResponse {
    {
        let mut grid = board.grid.lock().unwrap();
        *grid = Grid::new();
        let mut state = board.state.lock().unwrap();
        *state = GameState::Playing;
    }
    (StatusCode::OK, board.to_string())
}

pub async fn place_item(
    State(board): State<Arc<Board>>,
    Path((team, column)): Path<(String, usize)>, // team: "cookie" or "milk", column: from 1 to 4
) -> impl IntoResponse {

    if *board.state.lock().unwrap().deref() != GameState::Playing {
        return (StatusCode::SERVICE_UNAVAILABLE, "Game is over".to_string());
    }

    let tile_to_add = match team.as_str() {
        "cookie" => Tile::Cookie,
        "milk" => Tile::Milk,
        _ => return (StatusCode::BAD_REQUEST, "Invalid team".to_string()),
    };

    let column_index = column - 1;
    if column_index > 3 {
        return (StatusCode::BAD_REQUEST, "Invalid column".to_string());
    }
    {
        let mut item_was_placed = false;
        {
            let mut grid = board.grid.lock().unwrap();

            for row in grid.tiles.iter_mut().rev() {
                if row[column_index] == Tile::Empty {
                    row[column_index] = tile_to_add;
                    item_was_placed = true;
                    let mut state = board.state.lock().unwrap();
                    *state = get_game_state(&grid.tiles);
                    // println!("{:?}", state.deref());
                    break;
                }
            }
        }
        if item_was_placed {
            return (StatusCode::OK, board.to_string());
        }
    }

    (StatusCode::SERVICE_UNAVAILABLE, "Column is full".to_string())
}


pub fn day12_routes() -> Router {
    let state = Arc::new(Board::new());

    Router::new()
        .route("/12/board", get(get_board))
        .route("/12/reset", post(reset_board))
        .route("/12/place/:team/:column", post(place_item))
        .with_state(state)
}
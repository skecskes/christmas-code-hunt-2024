use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};

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
}

impl Board {
    fn new() -> Self {
        Self {
            grid: Arc::new(Mutex::new(Grid::new())),
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
        result
    }
}

pub async fn get_board(State(board): State<Arc<Board>>) -> impl IntoResponse {
    (StatusCode::OK, board.to_string())
}

pub async fn reset_board(State(board): State<Arc<Board>>) -> impl IntoResponse {
    {
        let mut grid = board.grid.lock().unwrap();
        *grid = Grid::new();
    }
    (StatusCode::OK, board.to_string())
}


pub fn day12_routes() -> Router {
    let state = Arc::new(Board::new());

    Router::new()
        .route("/12/board", get(get_board))
        .route("/12/reset", post(reset_board))
        .with_state(state)
}
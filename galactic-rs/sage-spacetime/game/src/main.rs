use instant::{Duration, Instant};
use macroquad::prelude::*;
use spacetimedb_sdk::{DbContext, Error, Identity, credentials};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod module_bindings;
use module_bindings::*;

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "sage-stdb";

const FPS: f64 = 30.0;

struct GameState {
    slot: u64,
    fleet_positions: HashMap<String, (f32, f32)>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            slot: 0,
            fleet_positions: HashMap::new(),
        }
    }
}

#[macroquad::main("sage spacetimedb")]
async fn main() -> anyhow::Result<()> {
    let game_state = Arc::new(RwLock::new(GameState::new()));

    let ctx = DbConnection::builder()
        .on_connect(|_ctx: &DbConnection, _identity: Identity, token: &str| {
            let _ = credentials::File::new(DB_NAME).save(token);
        })
        .on_connect_error(|_ctx: &ErrorContext, _error: Error| {
            std::process::exit(1);
        })
        .on_disconnect(|_ctx: &ErrorContext, _error: Option<Error>| {
            std::process::exit(0);
        })
        .with_token(credentials::File::new(DB_NAME).load()?)
        .with_module_name(DB_NAME)
        .with_uri(HOST)
        .build()?;

    ctx.subscription_builder()
        .on_applied(|_ctx: &SubscriptionEventContext| {
            // TODO
        })
        .on_error(|_ctx: &ErrorContext, error: Error| {
            println!("Subscription error: {:?}", error);
        })
        .subscribe(["SELECT * FROM config", "SELECT * FROM fleet_pos"]);

    let game_state_clone = game_state.clone();
    ctx.reducers
        .on_update_config_slot(move |_ctx: &ReducerEventContext, slot: &u64| {
            let mut state = game_state_clone.write().unwrap();
            state.slot = *slot;
        });

    let game_state_clone = game_state.clone();
    ctx.reducers
        .on_update_fleet_pos(move |_ctx: &ReducerEventContext, pos: &SageFleetPos| {
            let mut state = game_state_clone.write().unwrap();
            state
                .fleet_positions
                .insert(pos.pubkey.to_string(), (pos.x as f32, pos.y as f32));
        });

    ctx.run_threaded();

    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    let mut slot = 0;
    let mut fleet_positions = HashMap::new();

    loop {
        clear_background(WHITE);

        let cell_size = 10.0;
        let cols = (screen_width() / cell_size) as i32;
        let rows = (screen_height() / cell_size) as i32;
        let (center_col, center_row) = (cols / 2, rows / 2);

        for col in 0..=cols {
            let x = col as f32 * cell_size;
            draw_line(x, 0.0, x, screen_height(), 1.0, LIGHTGRAY);
        }
        for row in 0..=rows {
            let y = row as f32 * cell_size;
            draw_line(0.0, y, screen_width(), y, 1.0, LIGHTGRAY);
        }

        let delta = Instant::now().duration_since(last_update);
        accumulator = accumulator.saturating_add(delta);
        last_update = Instant::now();

        let fps_delta = 1. / FPS;
        while accumulator.as_secs_f64() > fps_delta {
            accumulator = accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

            (slot, fleet_positions) = {
                let state = game_state.read().unwrap();
                (state.slot, state.fleet_positions.clone())
            };
        }

        draw_text(&format!("Slot: {}", slot), 20.0, 20.0, 25.0, DARKPURPLE);
        draw_text(
            &format!("Fleet (x, y): {}", fleet_positions.len()),
            20.0,
            40.0,
            25.0,
            DARKPURPLE,
        );

        for (_fleet_id, (grid_x, grid_y)) in fleet_positions.iter() {
            let screen_x = (center_col + *grid_x as i32) as f32 * cell_size + cell_size / 2.0;
            let screen_y = (center_row + *grid_y as i32) as f32 * cell_size + cell_size / 2.0;
            draw_circle(screen_x, screen_y, 4.0, BLUE);
        }

        next_frame().await;
    }
}

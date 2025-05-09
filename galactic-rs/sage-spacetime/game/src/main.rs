use instant::{Duration, Instant};
use macroquad::prelude::*;
use spacetimedb_sdk::{DbContext, Error, Identity, Table, TableWithPrimaryKey, credentials};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod module_bindings;
use module_bindings::*;

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "sage-stdb";

const FPS: f64 = 30.0;

struct GameState {
    identity: String,
    slot: u64,
    fleet_positions: HashMap<String, (f32, f32)>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            identity: String::new(),
            slot: 0,
            fleet_positions: HashMap::new(),
        }
    }
}

struct Viewport {
    x: i32,
    y: i32,
    size: i32,
}

impl Viewport {
    fn new(x: i32, y: i32, size: i32) -> Self {
        Viewport { x, y, size }
    }
}

#[macroquad::main("sage spacetimedb")]
async fn main() -> anyhow::Result<()> {
    let game_state = Arc::new(RwLock::new(GameState::new()));

    let game_state_clone = game_state.clone();
    let ctx = DbConnection::builder()
        .on_connect(
            move |_ctx: &DbConnection, identity: Identity, token: &str| {
                let mut state = game_state_clone.write().unwrap();
                state.identity = format!("{}", identity);
                dbg!(&state.identity);
                let _ = credentials::File::new(DB_NAME).save(token);
            },
        )
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
    ctx.db
        .fleet_pos()
        .on_insert(move |_ctx: &EventContext, pos: &SageFleetPos| {
            let mut state = game_state_clone.write().unwrap();
            state
                .fleet_positions
                .insert(pos.pubkey.to_string(), (pos.x as f32, pos.y as f32));
        });

    let game_state_clone = game_state.clone();
    ctx.db.fleet_pos().on_update(
        move |_ctx: &EventContext, _old: &SageFleetPos, new: &SageFleetPos| {
            let mut state = game_state_clone.write().unwrap();
            state
                .fleet_positions
                .insert(new.pubkey.to_string(), (new.x as f32, new.y as f32));
        },
    );

    ctx.db.player_viewport().on_update(
        |_ctx: &EventContext, _old: &PlayerViewport, new: &PlayerViewport| {
            dbg!(new);
        },
    );

    ctx.run_threaded();

    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    let mut identeity = {
        let state = game_state.read().unwrap();
        state.identity.clone()
    };
    let mut slot = 0;
    let mut fleet_positions = HashMap::new();

    let mut viewport = Viewport::new(0, 0, 16); // 16x16 grid cells
    ctx.reducers
        .set_player_viewport(viewport.x, viewport.y, viewport.size)?;

    loop {
        if identeity.is_empty() {
            identeity = {
                let state = game_state.read().unwrap();
                state.identity.clone()
            }
        }

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

        if is_key_down(KeyCode::Left) {
            viewport.x -= 1;
        }
        if is_key_down(KeyCode::Right) {
            viewport.x += 1;
        }
        if is_key_down(KeyCode::Up) {
            viewport.y -= 1;
        }
        if is_key_down(KeyCode::Down) {
            viewport.y += 1;
        }

        viewport.x = viewport.x.clamp(-center_col, center_col - viewport.size);
        viewport.y = viewport.y.clamp(-center_row, center_row - viewport.size);

        let vp_screen_x = (center_col + viewport.x) as f32 * cell_size;
        let vp_screen_y = (center_row + viewport.y) as f32 * cell_size;
        let vp_size = viewport.size as f32 * cell_size;
        draw_rectangle_lines(vp_screen_x, vp_screen_y, vp_size, vp_size, 2.0, RED);

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

        draw_text(&format!("ID: {}", identeity), 20.0, 20.0, 25.0, DARKPURPLE);
        draw_text(&format!("Slot: {}", slot), 20.0, 40.0, 25.0, DARKPURPLE);
        draw_text(
            &format!("Fleet (x, y): {}", fleet_positions.len()),
            20.0,
            60.0,
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

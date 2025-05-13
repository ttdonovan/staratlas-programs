use instant::{Duration, Instant};
use macroquad::prelude::*;
use shipyard::{Component, EntityId, IntoIter, View, World};
use spacetimedb_sdk::{DbContext, Error, Identity, Table, TableWithPrimaryKey, credentials};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod module_bindings;
use module_bindings::*;

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "sage-stdb";

const FPS: f64 = 30.0;

#[derive(Component)]
struct Pubkey(String);

#[derive(Component)]
struct Pos(f32, f32);

#[derive(Component)]
struct Warp {
    from_sector: [i64; 2],
    to_sector: [i64; 2],
    slot_start: u64,
    slot_finish: u64,
}

struct GameState {
    identity: String,
    slot: u64,
    world: World,
    pubkey_to_entity: HashMap<String, EntityId>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            identity: String::new(),
            slot: 0,
            world: World::new(),
            pubkey_to_entity: HashMap::new(),
        }
    }

    fn world_entity(&mut self, pubkey: &str) -> EntityId {
        if let Some(&entity) = self.pubkey_to_entity.get(pubkey) {
            entity
        } else {
            let entity = self.world.add_entity((Pubkey(pubkey.to_string()),));
            self.pubkey_to_entity.insert(pubkey.to_string(), entity);
            entity
        }
    }
}

struct Viewport {
    x: i32,
    y: i32,
    size: i32,
    margin: i32,
}

impl Viewport {
    fn new(x: i32, y: i32, size: i32, margin: i32) -> Self {
        Viewport { x, y, size, margin }
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
        .subscribe([
            "SELECT * FROM config",
            "SELECT * FROM player_viewport WHERE id = :sender",
            "SELECT * FROM fleet_pos",
            "SELECT * FROM fleet_warping",
        ]);

    let game_state_clone = game_state.clone();
    ctx.reducers
        .on_update_config_slot(move |_ctx: &ReducerEventContext, slot: &u64| {
            let mut state = game_state_clone.write().unwrap();
            state.slot = *slot;
        });

    // static fleet positions
    let game_state_clone = game_state.clone();
    ctx.db
        .fleet_pos()
        .on_insert(move |_ctx: &EventContext, pos: &SageFleetPos| {
            let mut state = game_state_clone.write().unwrap();
            let id = state.world_entity(&pos.pubkey);
            state
                .world
                .add_component(id, Pos(pos.x as f32, pos.y as f32));
        });

    let game_state_clone = game_state.clone();
    ctx.db
        .fleet_pos()
        .on_delete(move |_ctx: &EventContext, pos: &SageFleetPos| {
            let mut state = game_state_clone.write().unwrap();
            let id = state.world_entity(&pos.pubkey);
            state.world.remove::<Pos>(id);
        });

    // kinetic fleet position
    let game_state_clone = game_state.clone();
    ctx.db
        .fleet_warping()
        .on_insert(move |_ctx: &EventContext, warping: &SageFleetWarping| {
            let mut state = game_state_clone.write().unwrap();
            let id = state.world_entity(&warping.pubkey);
            state.world.add_component(
                id,
                Warp {
                    from_sector: [warping.from_sector_x, warping.from_sector_y],
                    to_sector: [warping.to_sector_x, warping.to_sector_y],
                    slot_start: warping.slot_start,
                    slot_finish: warping.slot_finish,
                },
            );
        });

    let game_state_clone = game_state.clone();
    ctx.db
        .fleet_warping()
        .on_delete(move |_ctx: &EventContext, warping: &SageFleetWarping| {
            let mut state = game_state_clone.write().unwrap();
            let id = state.world_entity(&warping.pubkey);
            state.world.remove::<Warp>(id);
        });

    // ctx.db.player_viewport().on_update(
    //     |_ctx: &EventContext, _old: &PlayerViewport, new: &PlayerViewport| {
    //         dbg!(new);
    //     },
    // );

    ctx.run_threaded();

    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    let mut identeity = {
        let state = game_state.read().unwrap();
        state.identity.clone()
    };
    let mut slot = 0;

    let mut viewport = Viewport::new(0, 0, 16, 6); // 16x16 grid cells
    let mut vp_margin_x = viewport.x - viewport.margin;
    let mut vp_margin_y = viewport.y - viewport.margin;
    let vp_margin_size = viewport.size + viewport.margin * 2;

    ctx.reducers.set_player_viewport(
        viewport.x as i64,
        viewport.y as i64,
        viewport.size as i64,
        viewport.margin as i64,
    )?;

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

        let crossed_left = viewport.x < vp_margin_x;
        let crossed_right = viewport.x + viewport.size > vp_margin_x + vp_margin_size;
        let crossed_top = viewport.y < vp_margin_y;
        let crossed_bottom = viewport.y + viewport.size > vp_margin_y + vp_margin_size;
        if crossed_left || crossed_right || crossed_top || crossed_bottom {
            vp_margin_x = viewport.x - viewport.margin;
            vp_margin_y = viewport.y - viewport.margin;

            ctx.reducers.set_player_viewport(
                viewport.x as i64,
                viewport.y as i64,
                viewport.size as i64,
                viewport.margin as i64,
            )?;
        }

        viewport.x = viewport.x.clamp(-center_col, center_col - viewport.size);
        viewport.y = viewport.y.clamp(-center_row, center_row - viewport.size);

        let vpm_screen_x = (center_col + vp_margin_x) as f32 * cell_size;
        let vpm_screen_y = (center_row + vp_margin_y) as f32 * cell_size;
        let vpm_size = vp_margin_size as f32 * cell_size;
        draw_rectangle_lines(vpm_screen_x, vpm_screen_y, vpm_size, vpm_size, 2.0, GREEN);

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

            slot = {
                let state = game_state.read().unwrap();
                state.slot
            };
        }

        draw_text(
            &format!("ClientID: {}", identeity),
            20.0,
            20.0,
            25.0,
            DARKPURPLE,
        );
        draw_text(&format!("Slot: {}", slot), 20.0, 40.0, 25.0, DARKPURPLE);

        let fleet_count = {
            let state = game_state.read().unwrap();
            state.world.run(|positions: View<Pos>| positions.len())
        };
        draw_text(
            &format!("Fleet (x, y): {}", fleet_count),
            20.0,
            60.0,
            25.0,
            DARKPURPLE,
        );

        let warping_count = {
            let state = game_state.read().unwrap();
            state.world.run(|warping: View<Warp>| warping.len())
        };
        draw_text(
            &format!("Fleet (warping): {}", warping_count),
            20.0,
            80.0,
            25.0,
            DARKPURPLE,
        );

        {
            let state = game_state.read().unwrap();
            state.world.run(|v_pos: View<Pos>| {
                for pos in (&v_pos).iter() {
                    let screen_x = (center_col + pos.0 as i32) as f32 * cell_size + cell_size / 2.0;
                    let screen_y = (center_row + pos.1 as i32) as f32 * cell_size + cell_size / 2.0;
                    draw_circle(screen_x, screen_y, 4.0, BLUE);
                }
            });
        }

        next_frame().await;
    }
}

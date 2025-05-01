use anyhow::Result;
use dotenv::dotenv;
use instant::{Duration, Instant};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

use sage_game::host::{db_pool, staratlas_sage as sage};

const FPS: f64 = 30.0;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    host: bool,
}

struct GameContext {
    pub fleets: Arc<Mutex<Vec<sage::ui::UiFleet>>>,
}

impl GameContext {
    fn new() -> Self {
        GameContext {
            fleets: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[macroquad::main("Sage Game Peer-2-Peer")]
async fn main() -> Result<()> {
    dotenv().ok();

    let opts = Opt::from_args();
    assert!(opts.host, "Host flag is required at this time...");

    // Create game context outside the runtime
    let game_ctx = GameContext::new();

    if opts.host {
        let fleets_arc = game_ctx.fleets.clone();
        let rt = tokio::runtime::Runtime::new()?;

        rt.block_on(async {
            let database_url = dotenv::var("DATABASE_URL").unwrap();
            let pool = db_pool(&database_url).await.unwrap();

            let rows = sqlx::query!("SELECT * FROM sage_ui_fleets")
                .fetch_all(&pool)
                .await
                .unwrap();

            // Create a temporary vector for processing
            let mut loaded_fleets = Vec::new();

            for r in rows {
                let _pubkey = r.pubkey;
                let bytes = r.data.as_bytes();
                let fleet = serde_json::from_slice::<sage::ui::UiFleet>(bytes).unwrap();

                // Add fleet to our temporary vector
                loaded_fleets.push(fleet);
            }

            // Update game context with loaded fleets
            {
                let mut fleets_guard = fleets_arc.lock().unwrap();
                *fleets_guard = loaded_fleets;
                println!("Loaded {} fleets into game context", fleets_guard.len());
            }
        });
    }

    // Time variables for tick rates
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    loop {
        // Get delta time from last iteration and accumulate it
        let delta = Instant::now().duration_since(last_update);
        accumulator = accumulator.saturating_add(delta);
        last_update = Instant::now();

        let fps_delta = 1. / FPS;

        // If enough time is accumulated, run frame
        while accumulator.as_secs_f64() > fps_delta {
            // Decrease accumulator by fps_delta
            accumulator = accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

            // println!("Sage Game Peer-2-Peer: {:?}", delta);
        }

        // Wait for next loop
        next_frame().await;
    }
}

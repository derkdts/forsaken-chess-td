#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
mod generator;

use game::GameState;
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, Manager, State};
use tokio::sync::Mutex;

struct SharedState(Arc<Mutex<GameState>>);

#[tauri::command]
async fn summon_or_upgrade(
    gx: Option<usize>,
    gy: Option<usize>,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let mut g = state.0.lock().await;
    g.summon_or_upgrade(gx, gy);
    Ok(())
}

#[tauri::command]
async fn next_stage(stage: u32, state: State<'_, SharedState>) -> Result<(), String> {
    let mut g = state.0.lock().await;
    *g = GameState::new(stage);
    Ok(())
}

#[tauri::command]
async fn toggle_speed(state: State<'_, SharedState>) -> Result<u32, String> {
    let mut g = state.0.lock().await;
    g.speed = if g.speed == 1 { 2 } else { 1 };
    Ok(g.speed)
}

fn main() {
    let game_state = Arc::new(Mutex::new(GameState::new(1)));
    let loop_state = game_state.clone();

    tauri::Builder::default()
        .manage(SharedState(game_state))
        .invoke_handler(tauri::generate_handler![
            summon_or_upgrade,
            next_stage,
            toggle_speed
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(33));
                loop {
                    interval.tick().await;
                    let snapshot = {
                        let mut g = loop_state.lock().await;
                        g.tick(0.033);
                        g.clone()
                    };
                    let _ = app_handle.emit("game_state", &snapshot);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

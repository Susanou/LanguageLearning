use bevy::prelude::*;
use bevy::log;

use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState{
    InGame,
    Out,
    Paused,
}

fn main() {
    let mut app = App::new();
    
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins);
    app.insert_resource(BoardOptions{
        map_size: (20, 20),
        bomb_count: (40),
        tile_padding: 3.0,
        safe_start: true,
        ..Default::default()
    })
    .add_state(AppState::InGame)
    //.add_state(AppState::Paused)
    .add_plugin(BoardPlugin{
        running_state: AppState::InGame,
        paused_state: AppState::Paused,
    })
    .add_system(state_handler);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        /* if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        } */

        state.restart().unwrap(); // generates the board again without having to clear
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("Pausing detected");
        if state.current() == &AppState::InGame {
            log::info!("Game is Paused");
            state.push(AppState::Paused).unwrap();
        } else {
            log::info!("Game is playing");
            state.pop().unwrap();
        }
    }
}
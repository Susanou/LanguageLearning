use bevy::prelude::*;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

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
    .add_plugin(BoardPlugin);
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
use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;
mod board;
use board::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
    // Spot light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    
}


fn main() {

    let mut app = App::new();
    // Set antialiasing to use 4 samples
    app.insert_resource(Msaa { samples: 4 });
        // Set WindowDescriptor Resource to change title and size
    app.insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        });
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DefaultPickingPlugins);
    
    app.add_plugin(BoardPlugin);
    #[cfg(feature = "debug")]
    {
        // Debug hierarchy inspector
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<PieceType>()
            .register_inspectable::<Piece>()
            .register_inspectable::<Square>();
        app.add_plugin(DebugCursorPickingPlugin);
        app.add_plugin(DebugEventsPickingPlugin);
    }
    
    app.add_startup_system(setup);
    app.add_startup_system(create_pieces);
    app.run();
}
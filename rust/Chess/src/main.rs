use bevy::prelude::*;

mod pieces;
use pieces::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

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
        });
    // Spot light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Create meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    let white_mat_tmp = StandardMaterial {
        base_color: Color::rgb(1., 0.9, 0.9),
        unlit: true,
        ..Default::default()
    };

    let black_mat_tmp = StandardMaterial {
        base_color: Color::rgb(0., 0.1, 0.1),
        unlit: true,
        ..Default::default()
    };


    let white_material = materials.add(white_mat_tmp);
    let black_material = materials.add(black_mat_tmp);

    // Spawn all squares of the board 
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                // material according to coordinates in alternating parttern
                material: if(i+j+1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }

}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    //spawn white pieces

    // rook
    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        rook_handle.clone(), 
        Vec3::new(-0.1, 0., 1.8), 
        Vec3::new(0., 0., 0.)
    );

    // king
    spawn_piece_parts(
        &mut commands, 
        white_material.clone(), 
        king_handle.clone(), 
        king_cross_handle.clone(), 
        Vec3::new(-0.2, 0., -1.9), 
        Vec3::new(0.0,0.0,4.)
    );


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
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(setup);
    app.add_startup_system(create_board);
    app.add_startup_system(create_pieces);
    app.run();
}
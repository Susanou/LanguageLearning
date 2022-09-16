use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};

#[derive(Clone, Copy, PartialEq, Component)]
#[cfg_attr(feature="debug", derive(Inspectable))]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Component)]
#[cfg_attr(feature="debug", derive(Inspectable))]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Component)]
#[cfg_attr(feature="debug", derive(Inspectable))]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

fn spawn_piece_parts(
    mut commands:&mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    piece_type: PieceType,
    mesh: Handle<Mesh>,
    mesh_head: Handle<Mesh>,
    offset: Vec3,
    position: (u8, u8),
) {
    commands.spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(position.0 as f32, 0.0, position.1 as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: piece_type,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(offset);
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });

            parent.spawn_bundle(PbrBundle {
                mesh: mesh_head.clone(),
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(offset);
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_piece(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    piece_type: PieceType,
    mesh: Handle<Mesh>,
    offset: Vec3,
    position: (u8, u8),
) {
    commands.spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(position.0 as f32, 0.0, position.1 as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: piece_type,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(offset);
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn create_pieces(
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
    let white_mat_tmp = StandardMaterial {
        base_color: Color::rgb(1., 0.9, 0.9),
        reflectance: 0.,
        ..Default::default()
    };

    let black_mat_tmp = StandardMaterial {
        base_color: Color::rgb(0., 0.1, 0.1),
        reflectance: 0.,
        ..Default::default()
    };


    let white_material = materials.add(white_mat_tmp);
    let black_material = materials.add(black_mat_tmp);

    //spawn white pieces
    // rook
    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Rook,
        rook_handle.clone(), 
        Vec3::new(-0.1, 0., 1.8), 
        (0, 0)
    );

    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Rook, 
        rook_handle.clone(), 
        Vec3::new(-0.1, 0., 1.8), 
        (0, 7)
    );

    // knight
    spawn_piece_parts(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Knight,
        knight_1_handle.clone(), 
        knight_2_handle.clone(), 
        Vec3::new(-0.2, 0., 0.9), 
        (0, 1)
    );

    spawn_piece_parts(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Knight, 
        knight_1_handle.clone(), 
        knight_2_handle.clone(), 
        Vec3::new(-0.2, 0., 0.9), 
        (0, 6)
    );

    //spawn bishop
    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Bishop,
        bishop_handle.clone(), 
        Vec3::new(-0.1, 0., 0.), 
        (0, 2)
    );

    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Bishop,
        bishop_handle.clone(), 
        Vec3::new(-0.1, 0., 0.), 
        (0, 5)
    );

    // king
    spawn_piece_parts(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::King,
        king_handle.clone(), 
        king_cross_handle.clone(), 
        Vec3::new(-0.2, 0., -1.9), 
        (0, 4)
    );

    // queen
    spawn_piece(
        &mut commands, 
        white_material.clone(), 
        PieceColor::White,
        PieceType::Queen,
        queen_handle.clone(), 
        Vec3::new(-0.2, 0., -0.95), 
        (0, 3)
    );


    // Spawn black pieces
    // rook
    spawn_piece(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Rook,
        rook_handle.clone(), 
        Vec3::new(-0.1, 0., 1.8), 
        (7, 0)
    );

    spawn_piece(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Rook, 
        rook_handle.clone(), 
        Vec3::new(-0.1, 0., 1.8), 
        (7, 7)
    );

    // knight
    spawn_piece_parts(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Knight, 
        knight_1_handle.clone(), 
        knight_2_handle.clone(), 
        Vec3::new(-0.2, 0., 0.9), 
        (7, 1)
    );

    spawn_piece_parts(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Knight,
        knight_1_handle.clone(), 
        knight_2_handle.clone(), 
        Vec3::new(-0.2, 0., 0.9), 
        (7, 6)
    );

    //spawn bishop
    spawn_piece(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Bishop,
        bishop_handle.clone(), 
        Vec3::new(-0.1, 0., 0.), 
        (7, 2)
    );

    spawn_piece(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Bishop, 
        bishop_handle.clone(), 
        Vec3::new(-0.1, 0., 0.), 
        (7, 5)
    );

    // king
    spawn_piece_parts(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::King,
        king_handle.clone(), 
        king_cross_handle.clone(), 
        Vec3::new(-0.2, 0., -1.9), 
        (7, 4)
    );

    // queen
    spawn_piece(
        &mut commands, 
        black_material.clone(), 
        PieceColor::Black,
        PieceType::Queen,
        queen_handle.clone(), 
        Vec3::new(-0.2, 0., -0.95), 
        (7, 3)
    );

    //spawn pawns on both sides
    for i in 0..8 {
        spawn_piece(
            &mut commands, 
            white_material.clone(), 
            PieceColor::White,
            PieceType::Pawn,
            pawn_handle.clone(), 
            Vec3::new(-0.2, 0., 2.6), 
            (1, i)
        );

        spawn_piece(
            &mut commands, 
            black_material.clone(), 
            PieceColor::Black,
            PieceType::Pawn,
            pawn_handle.clone(), 
            Vec3::new(-0.2, 0., 2.6), 
            (6, i)
        );
    }

}
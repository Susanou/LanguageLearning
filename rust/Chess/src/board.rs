use bevy::prelude::*;
use bevy_mod_picking::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};


#[derive(Component)]
#[cfg_attr(feature="debug", derive(Inspectable))]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

pub fn create_board(
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

    // Spawn all squares of the board 
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                // material according to coordinates in alternating parttern
                material: if(i+j+1) % 2 == 0 {
                    materials.add(white_mat_tmp.clone())
                } else {
                    materials.add(black_mat_tmp.clone())
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Square {
                x: i,
                y: j,
            });
        }
    }

}

fn color_squares(
    mut events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {

    let mut top_entity = None;

    // Get entity under the cursor when there is one
    for event in events.iter() {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => top_entity = Some(*e),
            PickingEvent::Clicked(e) => selected_square.entity = Some(*e),
            _ => {},
        };
    }

    for (entity, square, material_handle) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        //change the color of the material
        material.base_color = if Some(entity) == top_entity {
            Color::rgb(0.5, 0.9, 0.5)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board)
            .add_system(color_squares);
    }
}
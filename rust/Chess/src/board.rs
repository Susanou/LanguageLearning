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
)
{
    // Helpful:
    //  Get entity from Event: https://stackoverflow.com/a/72260406
    //  bevy_mod_picking event example: https://github.com/aevyrie/bevy_mod_picking/blob/main/examples/events.rs
    //  bevy_picking doc: https://docs.rs/bevy_mod_picking/latest/bevy_mod_picking/index.html
    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                /*  Example of USE:
                     if let HoverEvent::JustEntered(ent) = e {
                         info!("ID of entity: {}", ent.id());
                         for (entity, square, handled) in query.iter() {
                             if entity == *ent {
                                 info!("\t=>Found !")
                             }
                         }
                     }
                */
                info!("color_squares:: Event: {:?}", e);

                let (HoverEvent::JustEntered(hovered_entity) |
                HoverEvent::JustLeft(hovered_entity)) = e;

                let (entity, square, material_handle) = query.get(*hovered_entity).unwrap();

                if selected_square.entity == Some(entity) {
                    // I added it first to always see the selected square, in blue
                    let material = materials.get_mut(material_handle).unwrap();
                    material.base_color = Color::rgb(0.9, 0.1, 0.9);


                } else if let HoverEvent::JustEntered(hovered_entity) = e {
                    if *hovered_entity == entity {
                        let material = materials.get_mut(material_handle).unwrap();
                        info!("Just Entered Entity: {}", entity.id());
                        material.base_color = Color::rgb(0.8, 0.3, 0.3);
                    }

                } else if let HoverEvent::JustLeft(hovered_entity) = e {
                    if *hovered_entity == entity {
                        let material = materials.get_mut(material_handle).unwrap();
                        material.base_color = if square.is_white() {
                            info!("Just Left entity: {} which was white", entity.id());
                            Color::rgb(1., 0.9, 0.9)
                        } else {
                            info!("Just Left entity: {} which was black", entity.id());
                            Color::rgb(0., 0.1, 0.1)
                        };
                    }
                }

                info!("\n");
            }
            _ => {}
        }
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
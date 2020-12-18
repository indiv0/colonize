use bevy::{ecs::{Entity, Query, Res, ResMut}, input::Input, math::Vec3, pbr::PbrBundle, prelude::Assets, prelude::{AppBuilder, Color, Commands, IntoSystem, KeyCode, Mesh, MouseButton, Plugin, StandardMaterial, Transform, shape, trace}};
use bevy_mod_picking::{Group, HighlightablePickMesh, InteractableMesh, PickableMesh, SelectablePickMesh};
use bevy_rapier3d::{na::{Vector2, Vector3}, physics::{EventQueue, RigidBodyHandleComponent}, rapier::{
        dynamics::{RigidBody, RigidBodyBuilder, RigidBodySet},
        geometry::{ColliderBuilder, ColliderSet, ContactEvent},
    }};
use rand::{thread_rng, Rng};

use crate::terrain::{Chunk, TerrainResource};

#[derive(Debug, PartialEq)]
enum State {
    Falling,
    Stationary,
}

#[derive(Debug, PartialEq)]
enum Task {
    RandomWalk,
}

// Struct for storing the currently selected dwarf, if any.
struct SelectedDwarf {
    dwarf: Option<Entity>,
}

#[derive(Debug)]
struct Dwarf {
    free_fall: bool,
    task: Option<Task>,
}

impl Dwarf {
    fn set_free_fall(&mut self, free_fall: bool) {
        self.free_fall = free_fall
    }

    fn start_random_walk(&mut self) {
        self.task = Some(Task::RandomWalk);
    }
}

impl Default for Dwarf {
    fn default() -> Self {
        Self { free_fall: true, task: None }
    }
}

#[derive(Debug)]
struct Name(String);

fn add_dwarves(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_res: Res<TerrainResource>,
) {
    let names = [
        "Khustrul Lavablade",
        "Vaddul Cavemantle",
        "Ormat Caskfury",
        "Alfodgror Steelbeard",
        "Krorhunri Mithrilshoulder",
        "Noramilin Blazingsunder",
        "Snastaenelyn Whitbow",
        "Kizneabela Horndelver",
        "Korkkalynn Opalback",
        "Thulgreline Oakview",
    ];

    // Pick random starting positions for each dwarf, on the surface of the terrain, within
    // 30 blocks of origin. Ensure that none of the locations collide.
    let mut spawn_positions = Vec::new();
    while spawn_positions.len() < names.len() {
        let p = random_point_in_circle((0., 0.), 10.);
        // To ensure that dwarves don't spawn inside of each other (which would cause
        // physics problems) we check if the given X & Z coords are already in the list.
        if !spawn_positions.contains(&p) {
            spawn_positions.push(p);
        }
    }
    let mut spawn_positions = spawn_positions
        .into_iter()
        // FIXME: `surface_y` doesn't return the correct value. It returns a value that's way
        //   above the actual surface.
        .map(|(x, z)| (x as f32, terrain_res.surface_y(x, z) as f32, z as f32));

    for name in &names {
        let position = spawn_positions.next().unwrap();
        spawn_dwarf(
            name.to_string(),
            position,
            commands,
            &mut meshes,
            &mut materials,
        );
    }

    commands.insert_resource(SelectedDwarf { dwarf: None });
}

fn spawn_dwarf(
    name: String,
    (px, py, pz): (f32, f32, f32),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    const SIZE: f32 = 1.;

    let entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(px, py, pz)),
            ..Default::default()
        })
        .with(Dwarf::default())
        .with(Name(name.to_string()))
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        .with(HighlightablePickMesh::default())
        .with(SelectablePickMesh::default())
        .current_entity()
        .unwrap();
    // Dynamic rigid-body with cuboid shape as the collision box.
    let rigid_body = RigidBodyBuilder::new_dynamic().translation(px, py, pz);
    let collider = ColliderBuilder::cuboid(SIZE, SIZE, SIZE).user_data(entity.to_bits() as u128);
    commands.insert(entity, (rigid_body, collider));
}

/// Chooses a random point with a circle.
fn random_point_in_circle(origin: (f64, f64), radius: f64) -> (f64, f64) {
    let mut rng = thread_rng();
    let r = radius * rng.gen::<f64>().sqrt();
    let theta = rng.gen::<f64>() * 2. * std::f64::consts::PI;
    let x = origin.0 + r * theta.cos();
    let y = origin.1 + r * theta.sin();
    (x, y)
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    terrain_res: Res<TerrainResource>,
) {
    // If the `T` button is pressed, spawn in 10 more dwarves.
    if keyboard_input.pressed(KeyCode::T) {
        add_dwarves(commands, meshes, materials, terrain_res);
    }
}

fn handle_physics_events(
    events: Res<EventQueue>,
    collider_set: Res<ColliderSet>,
    mut dwarf_query: Query<&mut Dwarf>,
    chunk_query: Query<&Chunk>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        trace!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        trace!("Received contact event: {:?}", contact_event);

        if let ContactEvent::Started(handle_1, handle_2) = contact_event {
            // If a dwarf has collided with the ground, mark him as no longer falling.
            let collider_1 = collider_set.get(handle_1).unwrap();
            let collider_2 = collider_set.get(handle_2).unwrap();

            let entity_1 = Entity::from_bits(collider_1.user_data as u64);
            let entity_2 = Entity::from_bits(collider_2.user_data as u64);

            let mut maybe_dwarf_entity: Option<Entity> = Option::None;
            if let Ok(_) = dwarf_query.get_component::<Dwarf>(entity_1) {
                maybe_dwarf_entity.replace(entity_1);
            } else if let Ok(_) = dwarf_query.get_component::<Dwarf>(entity_2) {
                maybe_dwarf_entity.replace(entity_2);
            }

            let mut maybe_chunk_entity: Option<Entity> = Option::None;
            if let Ok(_) = chunk_query.get_component::<Chunk>(entity_1) {
                maybe_chunk_entity.replace(entity_1);
            } else if let Ok(_) = chunk_query.get_component::<Chunk>(entity_2) {
                maybe_chunk_entity.replace(entity_2);
            }

            if let Some(dwarf_entity) = maybe_dwarf_entity {
                if let Some(_chunk_entity) = maybe_chunk_entity {
                    let mut dwarf = dwarf_query
                        .get_component_mut::<Dwarf>(dwarf_entity)
                        .unwrap();
                    dwarf.set_free_fall(false);
                    trace!("Dwarf is now stationary");
                }
            }
        } else if let ContactEvent::Stopped(handle_1, handle_2) = contact_event {
            // If a dwarf is colliding with the ground, mark him as falling.
            let collider_1 = collider_set.get(handle_1).unwrap();
            let collider_2 = collider_set.get(handle_2).unwrap();

            let entity_1 = Entity::from_bits(collider_1.user_data as u64);
            let entity_2 = Entity::from_bits(collider_2.user_data as u64);

            let mut maybe_dwarf_entity: Option<Entity> = Option::None;
            if let Ok(_) = dwarf_query.get_component::<Dwarf>(entity_1) {
                maybe_dwarf_entity.replace(entity_1);
            } else if let Ok(_) = dwarf_query.get_component::<Dwarf>(entity_2) {
                maybe_dwarf_entity.replace(entity_2);
            }

            let mut maybe_chunk_entity: Option<Entity> = Option::None;
            if let Ok(_) = chunk_query.get_component::<Chunk>(entity_1) {
                maybe_chunk_entity.replace(entity_1);
            } else if let Ok(_) = chunk_query.get_component::<Chunk>(entity_2) {
                maybe_chunk_entity.replace(entity_2);
            }

            if let Some(dwarf_entity) = maybe_dwarf_entity {
                if let Some(_chunk_entity) = maybe_chunk_entity {
                    let mut dwarf = dwarf_query
                        .get_component_mut::<Dwarf>(dwarf_entity)
                        .unwrap();
                    dwarf.set_free_fall(true);
                    trace!("Dwarf is now falling");
                }
            }
        }
    }
}

fn move_around(
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut dwarf_rigid_body_query: Query<(&mut Dwarf, &Name, &RigidBodyHandleComponent)>,
) {
    let mut rng = thread_rng();

    for (mut dwarf, name, rigid_body_handle) in dwarf_rigid_body_query.iter_mut() {
        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        // A dwarf that is falling can't do anything until they stop falling.
        // A dwarf that is in motion stays in motion.
        let is_idle = !dwarf.free_fall && !rigid_body.is_moving();
        if is_idle {
            // If the dwarf is idle, then that means it can start performing an action.
            trace!("Dwarf {:?} is now walking around", name);
            dwarf.start_random_walk();
            // TODO: do we need to wake up the rigidbody?
            let impulse = Vector3::new(rng.gen::<f32>(), 0., rng.gen::<f32>()).normalize() * 1000.;
            rigid_body.apply_impulse(impulse, true);
        }
    }
}

fn select_dwarves(event_query: Query<(&InteractableMesh, Entity)>,
    mut dwarf_query: Query<&mut Dwarf>,
    mut selected_dwarf: ResMut<SelectedDwarf>,
) {
    for (interactable, entity) in &mut event_query.iter() {
        let mouse_down_event = interactable
            .mouse_down_event(&Group::default(), MouseButton::Left)
            .unwrap();
        // If a mouse down event has occurred, select the dwarf for motion.
        if mouse_down_event.is_none() {
            continue;
        }

        // Check if the entity is a dwarf.
        if let Ok(dwarf) = dwarf_query.get_component::<Dwarf>(entity) {
            selected_dwarf.dwarf = Some(entity);
        }
    }
}

fn movement_direction(
    input: &Res<Input<KeyCode>>,
    positive: &[KeyCode],
    negative: &[KeyCode],
) -> i8 {
    let mut direction = 0;
    if positive.iter().any(|k| input.pressed(*k)) {
        direction += 1;
    }
    if negative.iter().any(|k| input.pressed(*k)) {
        direction -= 1;
    }
    direction
}

fn keyboard_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut selected_dwarf: ResMut<SelectedDwarf>,
    mut dwarf_query: Query<&mut RigidBodyHandleComponent>,
) {
    let mut rng = thread_rng();
    if let Some(entity) = selected_dwarf.dwarf {
        let rigid_body_handle = dwarf_query.get_component_mut::<RigidBodyHandleComponent>(entity).unwrap();
        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();

        let axis_backward = movement_direction(&keyboard_input, &[KeyCode::Z], &[KeyCode::X]);
        let axis_right = movement_direction(&keyboard_input, &[KeyCode::C], &[KeyCode::V]);
        let axis_up = movement_direction(&keyboard_input, &[KeyCode::B], &[KeyCode::N]);

        if axis_backward != 0 || axis_right != 0 || axis_up != 0 {
            let impulse = Vector3::new(axis_backward as f32, axis_right as f32, axis_up as f32).normalize() * 10.;
            rigid_body.apply_impulse(impulse, true);
        }
    }
}

pub(crate) struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_dwarves.system())
            .add_system(input_system)
            .add_system(handle_physics_events)
            .add_system(move_around)
            .add_system(select_dwarves)
            .add_system(keyboard_movement_system);
    }
}

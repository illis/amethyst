use Mass;
use MassRes;
use amethyst::core::transform::{Transform, GlobalTransform};
use amethyst::ecs::prelude::{Join, Read, System, WriteStorage, Entities, ReadExpect};
use amethyst::renderer::{MeshHandle, Material};
use amethyst::input::InputHandler;
use amethyst::core::cgmath::{Vector3, Array};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct MassSystem;

impl<'s> System<'s> for MassSystem {
    type SystemData = (
        WriteStorage<'s, Mass>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut masses, mut locals): Self::SystemData) {
        for (mass, local) in (&mut masses, &mut locals).join() {

            local.translation += mass.vel;

            //use std::thread::sleep;
            //use std::time::Duration;
            //sleep(Duration::new(0,20000)); // some intensive calculation
        }
    }
}

pub struct MassControllerSystem;

impl<'s> System<'s> for MassControllerSystem {
    type SystemData = (
        WriteStorage<'s, Mass>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MeshHandle>,
        WriteStorage<'s, Material>,
        WriteStorage<'s, GlobalTransform>,
        ReadExpect<'s, MassRes>,
        Read<'s, InputHandler<String, String>>,
        Entities<'s>,
    );

    fn run(&mut self, (mut masses, mut locals, mut meshes, mut mats, mut global_transforms, mass_res, input, entities): Self::SystemData) {
        if let Some(movement) = input.axis_value("add") {
            if movement != 0.0 {
                for i in 1..8 {
                    // add mass
                    use {ARENA_HEIGHT, ARENA_WIDTH};
                    let mass =  Mass{ vel: Vector3::new(0., -1.0, 0.), pos: Vector3::from_value(0.), mass: 1.};
                    let mut local_transform = Transform::default();
                    local_transform.translation = Vector3::new(ARENA_WIDTH / i as f32, ARENA_HEIGHT, 0.0);

                    let e = entities.create();
                    masses.insert(e, mass);
                    locals.insert(e, local_transform);
                    global_transforms.insert(e, GlobalTransform::default());
                    meshes.insert(e, mass_res.mesh.clone());
                    mats.insert(e, mass_res.mat.clone());
                    println!("asdsd");
                }
            }
        }
    }
}


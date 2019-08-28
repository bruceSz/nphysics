extern crate nalgebra as na;

use na::{Point2, Vector2, Isometry2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, DefaultBodySet, DefaultColliderSet, Ground,
                         BodyPartHandle, PBFFluid, IISPHFluid, LFFluid, DFSPHFluid};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics_testbed2d::Testbed;


pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81));
    let geometrical_world = DefaultGeometricalWorld::new();
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let joint_constraints = DefaultJointConstraintSet::new();
    let force_generators = DefaultForceGeneratorSet::new();

    /*
     * Ground
     */
    let ground_size = 25.0;
    let ground_shape =
        ShapeHandle::new_shared(Cuboid::new(Vector2::new(ground_size, 1.0)));

    let ground_handle = bodies.insert(Ground::new());

    let co = ColliderDesc::new(ground_shape.clone())
        .translation(-Vector2::y())
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(Vector2::x() * 3.0, std::f32::consts::PI / 2.0))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(Vector2::x() * -3.0, std::f32::consts::PI / 2.0))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    use nphysics2d::object::Multibody;
    use nphysics2d::joint::RevoluteJoint;
    for (handle, body) in bodies.iter() {
        if let Some(multibody) = body.downcast_ref::<Multibody<f32>>() {
            for link in multibody.links().filter_map(|l| l.joint().downcast_ref::<RevoluteJoint<f32>>()) {
                // Do something.
            }
        }
    }

    /*
     * Create a cube
     */
    /*
    let rb = RigidBodyDesc::new()
        .translation(Vector2::y() * 13.0)
        .build();
    let rb_handle = bodies.insert(rb);
    let cube = Cuboid::new(Vector2::repeat(0.4));
    let co = ColliderDesc::new(ShapeHandle::new_owned(cube))
        .density(1.0)
        .build(BodyPartHandle(rb_handle, 0));
    colliders.insert(co);
    */

    /*
     * Create the fluid
     */
    let num = 20;
    let particles_radius = 0.1;

    let shift = particles_radius * 2.0;// - 0.1;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0 + 2.0;
    let mut particle_centers = Vec::new();

    for i in 0usize.. num {
        for j in 0..num * 4 {
            let x = i as f32 * shift - centerx + j as f32 * 0.001;
            let y = j as f32 * shift + centery;
            particle_centers.push(Point2::new(x, y));
        }
    }

    // Build the fluid.
    let fluid = DFSPHFluid::new(1.2, particles_radius, particle_centers);
    let fluid_collider_desc = fluid.particles_collider_desc();
    let fluid_handle = bodies.insert(fluid);
    let fluid_collider = fluid_collider_desc.build(fluid_handle);
    colliders.insert(fluid_collider);

    /*
     * Set up the testbed.
     */
    testbed.set_ground_handle(Some(ground_handle));
    testbed.set_world(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators);
    testbed.look_at(Point2::new(0.0, -2.5), 95.0);
}


fn main() {
    let testbed = Testbed::from_builders(0, vec![
        ("Fluids", init_world),
    ]);
    testbed.run()
}
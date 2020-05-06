extern crate nalgebra as na;
extern crate generational_arena;

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nalgebra::Vector3;

use nphysics3d::object::{RigidBodyDesc, BodyPartHandle, ColliderDesc};
use nphysics3d::algebra::Velocity3;
use nphysics3d::material::{MaterialHandle, BasicMaterial};
use na::Matrix3;



use crate::{PhysicObject, process_shape};

pub struct Physics{
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>,
}

impl Default for Physics
{
    fn default() -> Self
    {
        // MechanicalWorld with a gravity vector
        let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();
	
        // Where we store all the RigidBody object
        let bodies = DefaultBodySet::new();

        // Where we store all the Collider object
        let colliders = DefaultColliderSet::<f32>::new();

        Physics::new(
	    mechanical_world,
	    geometrical_world,
	    bodies,
	    colliders,
	    joint_constraints,
	    force_generators)

    }
}

impl Physics
{
    pub fn new(mechanical_world: DefaultMechanicalWorld<f32>,
        geometrical_world: DefaultGeometricalWorld<f32>,
        bodies: DefaultBodySet<f32>,
        colliders: DefaultColliderSet<f32>,
        joint_constraints: DefaultJointConstraintSet<f32>,
        force_generators: DefaultForceGeneratorSet<f32>) -> Physics{

        let physics = Physics
	{
	    mechanical_world: mechanical_world,
            geometrical_world: geometrical_world,
            bodies: bodies,
            colliders: colliders,
            joint_constraints: joint_constraints,
            force_generators: force_generators
	};

        return physics;
    }

    


    /// Runs the universe and ticks it 60 times per second
    pub fn run(&mut self){
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );
        
    }

/// Creates the RigidBody and Collider of the PhysicObject given in parameter, store them in a ColliderSet and a Vector<Collider> and returns it
    pub fn build_rigbd_col(&mut self, physic_object: &PhysicObject) -> generational_arena::Index
    {
        let shape = process_shape(&physic_object.shape); // ShapeHandle object de ncollide
        
        // We create the RigidBody relative to the field rbdata of 'object'
        let mut rb = RigidBodyDesc::new()
        .translation(physic_object.rbdata.translation)
        .rotation(physic_object.rbdata.rotation)
        .gravity_enabled(physic_object.rbdata.gravity_enabled)
	    .status(physic_object.rbdata.bodystatus)
        .velocity(Velocity3::new(physic_object.rbdata.linear_velocity, physic_object.rbdata.angular_velocity))
        .linear_damping(physic_object.rbdata.linear_damping)
        .angular_damping(physic_object.rbdata.angular_damping)
        .max_linear_velocity(physic_object.rbdata.max_linear_velocity)
        .max_angular_velocity(physic_object.rbdata.max_angular_velocity)
        .angular_inertia(Matrix3::from_diagonal_element(physic_object.rbdata.angular_inertia))
        .mass(physic_object.rbdata.mass)
//        .local_center_of_mass(physic_object.rbdata.local_center_of_mass)
        .sleep_threshold(Some(physic_object.rbdata.sleep_threshold))
        .kinematic_translations(physic_object.rbdata.kinematic_translations)
        .kinematic_rotations(physic_object.rbdata.kinematic_rotations)
        .user_data(physic_object.rbdata.user_data)
        .build(); // Build the rigid-body
    
        rb.enable_linear_motion_interpolation(physic_object.rbdata.enable_linear_motion_interpolation);
    
        // We add the RigidBody to the RigidBodySet
        let rb_handle = self.bodies.insert(rb);
    
    
        // We create the Collider relative to the field coldata of 'object'
        let collider = ColliderDesc::new(shape)
        .translation(physic_object.coldata.translation)
        .rotation(physic_object.coldata.rotation)
        .density(physic_object.coldata.density)
        .material(MaterialHandle::new(BasicMaterial::new(physic_object.coldata.restitution, physic_object.coldata.friction)))
        .margin(physic_object.coldata.margin)
        .linear_prediction(physic_object.coldata.linear_prediction)
        .angular_prediction(physic_object.coldata.angular_prediction)
        .sensor(physic_object.coldata.sensor)
        .user_data(physic_object.coldata.user_data)
        .build(BodyPartHandle(rb_handle, 0)); // Build the collider into the world
        
        // We add the Collider to the set of colliders
        let coll_handle = self.colliders.insert(collider);
        
        coll_handle
    }
}

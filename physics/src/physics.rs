extern crate nalgebra as na;
extern crate generational_arena;

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nalgebra::Vector3;

pub struct Physics{
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>,
    pub col_tab: Vec<generational_arena::Index>,
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

        // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
        let col_tab = Vec::new();

        Physics::new(
	    mechanical_world,
	    geometrical_world,
	    bodies,
	    colliders,
	    joint_constraints,
	    force_generators,
	    col_tab)

    }
}

impl Physics
{
    pub fn new(mechanical_world: DefaultMechanicalWorld<f32>,
        geometrical_world: DefaultGeometricalWorld<f32>,
        bodies: DefaultBodySet<f32>,
        colliders: DefaultColliderSet<f32>,
        joint_constraints: DefaultJointConstraintSet<f32>,
        force_generators: DefaultForceGeneratorSet<f32>,
        col_tab: Vec<generational_arena::Index>) -> Physics{

        let physics = Physics{mechanical_world: mechanical_world,
            geometrical_world: geometrical_world,
            bodies: bodies,
            colliders: colliders,
            joint_constraints: joint_constraints,
            force_generators: force_generators,
            col_tab};

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
}

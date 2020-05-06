extern crate nalgebra as na;

use crate::shapes::*;

use nphysics3d::object::{BodyStatus, ActivationStatus};

use ncollide3d::shape::ShapeHandle;
use na::Vector3;
use na::geometry::Point3;


use graphics::{Object, Vertex};
use std::f32::consts::PI;
use std::f32::INFINITY;




// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
/// Different types of shape an PhysicObject can take
pub enum ShapeType {
    Ball(Ball),
    Capsule(Capsule),
    Compound(Compound),
    ConvexHull(ConvexHull),
    Cuboid(Cuboid),
    HeightField(HeightField),
    Plane(Plane),
    Polyline(Polyline),
    Segment(Segment),
    TriMesh(TriMesh),
    Triangle(Triangle)
}

impl ShapeType 
{
    pub fn make_static(
	&self,
	translation: Vector3<f32>,
	rotation: Vector3<f32>,
	scale: f32,
	gravity: bool) -> PhysicObject
    {
	    self.make_object(translation, rotation, scale, gravity, BodyStatus::Static, None, None)
    }

    pub fn make_dynamic(
	&self,
	translation: Vector3<f32>,
	rotation: Vector3<f32>,
	scale: f32,
	gravity: bool) -> PhysicObject
    {
	    self.make_object(translation, rotation, scale, gravity, BodyStatus::Dynamic, None, None)
    }

    pub fn make_kinematic(
	&self,
	translation: Vector3<f32>,
	rotation: Vector3<f32>,
	scale: f32,
    gravity: bool,
    kinematic_rotations : Vector3<bool>,
    kinematic_translations : Vector3<bool>) -> PhysicObject
    {
	    self.make_object(translation, rotation, scale, gravity, BodyStatus::Kinematic, Some(kinematic_rotations), Some(kinematic_translations))
    }

    pub fn make_dynamic_sans_liberte(
        &self,
        translation: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: f32,
        gravity: bool,
        kinematic_rotations : Vector3<bool>,
        kinematic_translations : Vector3<bool>) -> PhysicObject
        {
            self.make_object(translation, rotation, scale, gravity, BodyStatus::Dynamic, Some(kinematic_rotations), Some(kinematic_translations))
        }


    pub fn make_object(
	&self,
	translation: Vector3<f32>,
	rotation: Vector3<f32>,
	scale: f32,
	gravity: bool,
    stat: BodyStatus,
    kinematic_rot : Option<Vector3<bool>>,
    kinematic_trans : Option<Vector3<bool>>) -> PhysicObject
    {
        match self
        {
            ShapeType::TriMesh(trimesh) => {
            
                let shape = ShapeType::TriMesh(trimesh.clone());

		        let center: Point3<f32> = trimesh
		        .points.iter()
		        .fold(Point3::new(0., 0., 0.), |sum, p| sum+p.coords) / (trimesh.points.len() as f32);
        
              //  let kinematic_rotations = kinematic_rot.unwrap_or(Vector3::new(false, false, false));
              //  let kinematic_translations = kinematic_trans.unwrap_or(Vector3::new(false, false, false));
                
                let rb_data = RbData::new(
                    translation,                            // translation
                    rotation,                               // rotation
                    gravity,                                // gravity_enabled
                    stat,                                   // bodystatus
                    Vector3::new(0.0, 0.0, 0.0),            // linear_velocity
                    Vector3::new(0.0, 0.0, 0.0),            // angular_velocity
                    0.8,                                    // linear_damping
                    1.8,                                    // angular_damping
                    INFINITY,                               // max_linear_velocity
                    INFINITY,                               // max_angular_velocity
                    0.0,                                    // angular_inertia
                    2000.0,                                 // mass
                    center                    ,             // local_center_of_mass
                    ActivationStatus::default_threshold(),  // sleep_threshold
                    kinematic_trans.unwrap_or(Vector3::new(false, false, false)),                  // kinematic_translations
                    kinematic_rot.unwrap_or(Vector3::new(false, false, false)),                    // kinematic_rotations
                    0,                                      // user_data
                    true                                    // enable_linear_motion_interpolation
                );

		
                let col_data = ColData::new(
                    Vector3::new(0.0, 0.0, 0.0),            // translation relative to the RigidBody it's attached to
                    Vector3::new(0.0, 0.0, 0.0),            // rotation relative to the RigidBody it's attached to
                    0.0,                                    // density ! Since we use TriMesh objects it needs to be 0.0 or game will crash !
                    0.5,                                    // restitution
                    0.2,                                    // friction
                    0.01,                                   // margin
                    0.002,                                  // linear_prediction
                    PI / 180.0 * 5.0,                       // angular_prediction
                    false,                                  // sensor
                    0                                       // user_data
                );
		
                PhysicObject::new(shape, rb_data, col_data)  
            },
            _ => unimplemented!()
        }
    }
}


/// Data needed to create a 'RigidBody'
pub struct RbData {
    pub translation: Vector3<f32>, // The rigid body translation - Default: zero vector
    pub rotation: Vector3<f32>, // The rigid body rotation - Default: no rotation
    pub gravity_enabled: bool, // Whether or not this rigid body is affected by gravity - Default: true
    pub bodystatus: BodyStatus, // The status of this rigid body. It can be Disabled, Static, Kinematic or Dynamic - Default: BodyStatus::Dynamic
    pub linear_velocity: Vector3<f32>, // The velocity of this body - Default: zero velocity
    pub angular_velocity: Vector3<f32>, // The velocity of this body - Default: zero velocity
    pub linear_damping: f32, // The linear damping applied to this rigid body velocity to slow it down automatically - Default: zero (no damping at all)
    pub angular_damping: f32, // The angular damping applied to this rigid body velocity to slow down its rotation automatically - Default: zero (no damping at all)
    pub max_linear_velocity: f32, // The maximum linear velocity this rigid body can reach - Default: f32::max_value() or f64::max_value() (no limit)
    pub max_angular_velocity: f32, // The maximum angular velocity this rigid body can reach - Default: f32::max_value() or f64::max_value() (no limit)
    pub angular_inertia: f32, // The angular inertia tensor of this rigid body, expressed on its local-space - Default: the zero matrix
    pub mass: f32, // The rigid body mass - Default: 0.0
    pub local_center_of_mass: Point3<f32>, // The center of mass of this rigid body expressed in its local-space - Default: the origin
    pub sleep_threshold: f32, // The threshold for putting this rigid body to sleep - Default: Some(ActivationStatus::default_threshold())
    pub kinematic_translations: Vector3<bool>, // The translations that will be locked for this rigid body - Default: nothing is locked (false everywhere)
    pub kinematic_rotations: Vector3<bool>, // The rotations that will be locked for this rigid body - Default: nothing is locked (false everywhere)
    pub user_data: usize, // Arbitrary user-defined data associated to the rigid body to be built - Default: no associated data
    pub enable_linear_motion_interpolation: bool // Whether this rigid body motion should be interpolated linearly during CCD resolution - Default: false (which implies non-linear interpolation)
}

impl Default for RbData
{
    fn default() -> Self {
	    Self {
	        translation: Vector3::new(0., 0., 0.),
	        rotation: Vector3::new(0., 0., 0.),
	        gravity_enabled: true,
	        bodystatus: BodyStatus::Dynamic,
	        linear_velocity: Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            angular_velocity: Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            linear_damping: 0.,
            angular_damping: 0.,
            max_linear_velocity: std::f32::MAX,
            max_angular_velocity: std::f32::MAX,
            angular_inertia: 0.,
            mass: 0.,
            local_center_of_mass: Point3::new(0., 0., 0.),
            sleep_threshold: 0.,
            kinematic_translations: Vector3::new(false, false, false),
            kinematic_rotations: Vector3::new(false, false, false),
            user_data: 0,
            enable_linear_motion_interpolation: false
	    }
    }
}

impl RbData {
    pub fn new(
        translation: Vector3<f32>, 
        rotation: Vector3<f32>, 
        gravity_enabled: bool, 
        bodystatus: BodyStatus, 
        linear_velocity: Vector3<f32>, 
        angular_velocity: Vector3<f32>, 
        linear_damping: f32, 
        angular_damping: f32, 
        max_linear_velocity: f32, 
        max_angular_velocity: f32, 
        angular_inertia: f32, 
        mass: f32, 
        local_center_of_mass: Point3<f32>, 
        sleep_threshold: f32, 
        kinematic_translations: Vector3<bool>, 
        kinematic_rotations: Vector3<bool>, 
        user_data: usize, 
        enable_linear_motion_interpolation: bool) -> RbData{

        RbData {
            translation: translation, 
            rotation: rotation, 
            gravity_enabled: gravity_enabled, 
            bodystatus: bodystatus, 
            linear_velocity: linear_velocity, 
            angular_velocity: angular_velocity, 
            linear_damping: linear_damping, 
            angular_damping: angular_damping, 
            max_linear_velocity: max_linear_velocity, 
            max_angular_velocity: max_angular_velocity, 
            angular_inertia: angular_inertia, 
            mass: mass, 
            local_center_of_mass: local_center_of_mass, 
            sleep_threshold: sleep_threshold, 
            kinematic_translations: kinematic_translations, 
            kinematic_rotations: kinematic_rotations, 
            user_data: user_data, 
            enable_linear_motion_interpolation: enable_linear_motion_interpolation 
        }
    }
}

/// Data needed to create a 'Collider'
pub struct ColData{
    pub translation: Vector3<f32>, // The collider translation wrt. the body part it is attached to - Default: zero vector
    pub rotation: Vector3<f32>, // The collider rotation wrt. the body part it is attached to - Default: no rotation
    pub density: f32, // If non-zero the collider's mass and angular inertia will be added to the inertial properties of the body part it is attached to - Default: 0.0
    pub restitution: f32, // Restitution of the collider - Default 0.0
    pub friction: f32, // Friction of the collider - Default: 0.5
    pub margin: f32, // Solid margin surrounding the collider (should always be non-zero) - Default: 0.01
    pub linear_prediction: f32, // The distance tolerance for predictive contacts generation - Default: 0.002
    pub angular_prediction: f32, // The angular tolerance for predictive contacts generation - Default: PI / 180.0 * 5.0
    pub sensor: bool, // Whether this collider is a sensor, i.e., generate only proximity events - Default: false
    pub user_data: usize // Arbitrary user-defined data associated to the rigid body to be built - Default: no associated data
}

impl Default for ColData {
    fn default() -> Self {
	    Self {
	        translation: Vector3::new(0., 0., 0.),
	        rotation: Vector3::new(0., 0., 0.),
	        density: 0.,
	        restitution: 0.,
	        friction: 0.5,
	        margin: 0.01,
	        linear_prediction: 0.002,
	        angular_prediction: std::f32::consts::PI/180.*5.,
	        sensor: false,
	        user_data: 0
	    }
    }
}


impl ColData{
    pub fn new(
        translation: Vector3<f32>, 
        rotation: Vector3<f32>,
        density: f32,
        restitution: f32,
        friction: f32,
        margin: f32,
        linear_prediction: f32,
        angular_prediction: f32,
        sensor: bool,
        user_data: usize) -> ColData{

        ColData{
            translation: translation, 
            rotation: rotation,
            density: density,
            restitution: restitution,
            friction: friction,
            margin: margin,
            linear_prediction: linear_prediction,
            angular_prediction: angular_prediction,
            sensor: sensor,
            user_data: user_data
        }
    }
}

/// An PhysicObject with different features
pub struct PhysicObject {
    pub shape: ShapeType,
    pub rbdata: RbData,
    pub coldata: ColData
}

impl PhysicObject {
    pub fn new(shape: ShapeType, rbdata: RbData, coldata: ColData) -> PhysicObject {
        PhysicObject {
            shape: shape, 
            rbdata: rbdata, 
            coldata: coldata
        }
    }
}



/// Creates and returns a RigidBody corresponding to the PhysicObject's shape
pub fn process_shape(event: &ShapeType) -> ShapeHandle<f32>{
    match event {
        ShapeType::Ball(ball) => return Ball::process_ball(ball.clone()),
        ShapeType::Capsule(capsule) => return Capsule::process_capsule(capsule.clone()),
        ShapeType::Compound(compound) => return Compound::process_compound(compound.clone()),
        ShapeType::ConvexHull(convexhull) => return ConvexHull::process_convexhull(convexhull.clone()),
        ShapeType::Cuboid(cuboid) => return Cuboid::process_cuboid(cuboid.clone()),
        ShapeType::HeightField(heightfield) => return HeightField::process_heightfield(heightfield.clone()),
        ShapeType::Plane(plane) => return Plane::process_plane(plane.clone()),
        ShapeType::Polyline(polyline) => return Polyline::process_polyline(polyline.clone()),
        ShapeType::Segment(segment) => return Segment::process_segment(segment.clone()),
        ShapeType::TriMesh(trimesh) => return TriMesh::process_trimesh(trimesh.clone()),
        ShapeType::Triangle(triangle) => return Triangle::process_triangle(triangle.clone()),
    }
}



// Create the ShapeType::TriMesh associated to the object and return it
pub fn make_trimesh(object: &Object) -> ShapeType
{
    let all_vertex = object.data.iter()
	.map(|(group, _)|  
	{
		(*group.vertexes)
		.read()
		.iter()
		.flatten()
		.map(|vertex: &Vertex|
		{
			Point3::new(vertex.position[0], vertex.position[1], vertex.position[2])
		})
		.collect::<Vec<_>>()
    }
    ).flatten()
    .collect::<Vec<_>>() ;

    let indices = (0..(all_vertex.len()/3)).map(|i| {Point3::new(3*i, 3*i+1, 3*i+2)}).collect::<Vec<_>>() ;

    ShapeType::TriMesh(TriMesh::new(all_vertex, indices, None))  // We might need the scale
}

/* 
########################################################################
 EVERYTHING BETWEEN "### FOR TESTING PURPOSE ONLY ###" and 
 "### END OF TESTING ###" must be utlimately removed
########################################################################
*/
extern crate nalgebra as na;

use std::vec::Vec;

use na::Vector3;
use na::base::{Unit, DMatrix};
use na::geometry::{Point2, Point3};

use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc, RigidBody, BodyPartHandle, ColliderDesc, BodyStatus};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::material::{MaterialHandle, BasicMaterial};

use ncollide3d::shape::ShapeHandle;
use ncollide3d::shape;
use ncollide3d::math::{Point, Isometry};



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Ball
{
    radius: f32
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Capsule
{
    half_height: f32,
    radius: f32
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Compound
{
    shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct ConvexHull
{
    points: Vec<Point<f32>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Cuboid
{
    vector: Vector3<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct HeightField
{
    heights: DMatrix<f32>,
    scale: Vector3<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Plane
{
    normal: Unit<Vector3<f32>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Polyline
{
    points: Vec<Point<f32>>,
    indices: Option<Vec<Point2<usize>>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Segment
{
    a: Point<f32>,
    b: Point<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct TriMesh
{
    points: Vec<Point<f32>>,
    indices: Vec<Point3<usize>>,
    uvs: Option<Vec<Point2<f32>>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Triangle
{
    a: Point<f32>,
    b: Point<f32>,
    c: Point<f32>
}

// We create an enum with all the shapes of mesh we can create with ncollide3d
#[derive(Clone)]
/// Different types of shape an object can take
pub enum MeshType {
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

// We implement the Copy trait to the structure
#[derive(Copy, Clone)]
pub struct Coordinates{
    x: f32,
    y: f32,
    z: f32
}

/// An object with different features
pub struct Object {
    position: Coordinates,
    //speed: f32,
    //mass: f32,
    //can_move: bool,
    mesh: MeshType,
    density: f32,
    restitution: f32,
    friction: f32,
}

/// A set that contains many 'Object'
pub struct ObjSet{
    tab: Vec<Object>
}



/// Creates an empty Vec that can store 'Object'
pub fn build_object_table() -> Vec<Object>{
    let tab = Vec::new();
    return tab;
}

/// Creates an 'ObjSet' with the tab given as parameter
pub fn build_obj_set(tab: Vec<Object>,) -> ObjSet{
    ObjSet {
        tab
    }
}

/// Creates and returns a RigidBody corresponding to the 'Ball' type
pub fn process_ball(ball: Ball, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and radius of the Ball
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let radius = ball.radius;

    // Creation of a Ball
    let bal = ShapeHandle::new(shape::Ball::new(radius));

    // Creation of the Ball's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, bal);
}

/// Creates and returns a RigidBody corresponding to the 'Capsule' type
pub fn process_capsule(capsule: Capsule, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, half-height and radius of the Capsule
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let half_height = capsule.half_height;
    let radius = capsule.radius;

    // Creation of a Capsule
    let caps = ShapeHandle::new(shape::Capsule::new(half_height, radius));

    // Creation of the Capsule's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, caps);
}

/// Creates and returns a RigidBody corresponding to the 'Compound' type
pub fn process_compound(compound: Compound, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and shapes of the Compound
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let shapes = compound.shapes; 

    // Creation of a Compound
    let comp = ShapeHandle::new(shape::Compound::new(shapes));

    // Creation of the Compound's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, comp);
}

/// Creates and returns a RigidBody corresponding to the 'ConvexHull' type
pub fn process_convexhull(convexhull: ConvexHull, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordonnées and points of the ConvexHull
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = convexhull.points;

    // Creation of a ConvexHull
    let convexh = ShapeHandle::new(shape::ConvexHull::try_from_points(&points).unwrap());

    // Creation of the ConvexHull's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, convexh);
}

/// Creates and returns a RigidBody corresponding to the 'Cuboid' type
pub fn process_cuboid(cuboid: Cuboid, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordonnées and vector of the Cuboid
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let vector = cuboid.vector;

    // Creation of a Cuboid
    let cub = ShapeHandle::new(shape::Cuboid::new(vector));

    // Creation of the Cuboid's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, cub);
}

/// Creates and returns a RigidBody corresponding to the 'HeightField' type
pub fn process_heightfield(heightfield: HeightField, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, height and scale of the HeightField
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let heights = heightfield.heights;
    let scale = heightfield.scale;

    // Creation of a HeightField
    let heightf = ShapeHandle::new(shape::HeightField::new(heights, scale));

    // Creation of the HeightField's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, heightf);
}

/// Creates and returns a RigidBody corresponding to the 'Plane' type
pub fn process_plane(plane: Plane, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and normal of the Plane
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let normal = plane.normal;

    // Creation of a Plane
    let pla = ShapeHandle::new(shape::Plane::new(normal));

    // Creation of the Plane's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, pla);
}

/// Creates and returns a RigidBody corresponding to the 'Polyline' type
pub fn process_polyline(polyline: Polyline, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, points and indices of the Polyline
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = polyline.points;
    let indices = polyline.indices;

    // Creation of a Polyline
    let polyl = ShapeHandle::new(shape::Polyline::new(points, indices));

    // Creation of the Polyline's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, polyl);
}
 
/// Creates and returns a RigidBody corresponding to the 'Segment' type
pub fn process_segment(segment: Segment, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and points of the Segment
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = segment.a;
    let b = segment.b; 

    // Creation of a Segment
    let seg = ShapeHandle::new(shape::Segment::new(a, b));

    // Creation of the Segment's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, seg);
}

/// Creates and returns a RigidBody corresponding to the 'TriMesh' type
pub fn process_trimesh(trimesh: TriMesh, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, points, indices and uvs of the TriMesh
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = trimesh.points;
    let indices = trimesh.indices;
    let uvs = trimesh.uvs;

    // Creation of a TriMesh
    let trim = ShapeHandle::new(shape::TriMesh::new(points, indices, uvs));

    // Creation of the TriMesh's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, trim);
}

/// Creates and returns a RigidBody corresponding to the 'Triangle' type
pub fn process_triangle(triangle: Triangle, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and points of the Triangle
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    // Creation of a Triangle
    let tri = ShapeHandle::new(shape::Triangle::new(a, b, c));

    // Creation of the Triangle's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, tri);
}

/// Creates and returns a Tuple containing: 1.A RigidBody corresponding to the object's shape (mesh); 2.The position of the RigidBody
pub fn process_mesh(event: MeshType, object: &Object) -> (RigidBody<f32>, ShapeHandle<f32>) {
    match event {
        MeshType::Ball(ball) => return process_ball(ball, object.position),
        MeshType::Capsule(capsule) => return process_capsule(capsule, object.position),
        MeshType::Compound(compound) => return process_compound(compound, object.position),
        MeshType::ConvexHull(convexhull) => return process_convexhull(convexhull, object.position),
        MeshType::Cuboid(cuboid) => return process_cuboid(cuboid, object.position),
        MeshType::HeightField(heightfield) => return process_heightfield(heightfield, object.position),
        MeshType::Plane(plane) => return process_plane(plane, object.position),
        MeshType::Polyline(polyline) => return process_polyline(polyline, object.position),
        MeshType::Segment(segment) => return process_segment(segment, object.position),
        MeshType::TriMesh(trimesh) => return process_trimesh(trimesh, object.position),
        MeshType::Triangle(triangle) => return process_triangle(triangle, object.position),
    }
}

/// Creates the world, colliders and ticks the world. Some of these tasks will be ensured by other functions in a later release.
pub fn main() {
    // MechanicalWorld with a gravity vector
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let mut geometrical_world = DefaultGeometricalWorld::<f32>::new();

    // Where we store all the RigidBody object
    let mut bodies = DefaultBodySet::new();
    // Where we store all the Collider object
    let mut colliders = DefaultColliderSet::<f32>::new();
    let mut joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let mut force_generators = DefaultForceGeneratorSet::<f32>::new();

    // We create the tab of the Obj_set
    let mut obj_tab = build_object_table();

    /* 
     We create a tab to store the handle of every collider so we can
     get their position and material.
    */
    let mut coll_tab = Vec::new();



    // ### FOR TESTING PURPOSE ONLY ###
    // BALL
    let coords_ball = Coordinates{
        x: 0 as f32,
        y: 500 as f32,
        z: 0 as f32
    };

    let mesh_ball = MeshType::Ball(Ball{ radius: 1.0 as f32});

    let ball = Object {
        position: coords_ball,
        mesh: mesh_ball,
        density: 1.0 as f32,
        restitution: 0.8 as f32,
        friction: 0.8 as f32,
    };

    obj_tab.push(ball);

    // GROUND

    let coords_ground = Coordinates{
        x: 0 as f32,
        y: 0 as f32,
        z: 0 as f32
    };

    let vec_ground = Vector3::new(3.0, 0.2, 3.0);

    let mesh_ground = MeshType::Cuboid(Cuboid{vector: vec_ground});

    let ground = Object {
        position: coords_ground,
        mesh: mesh_ground,
        density: 1.0 as f32,
        restitution: 0.0 as f32,
        friction: 0.5 as f32,
    };

    obj_tab.push(ground);
    // ### END OF TESTING ###



    // We create the Obj_set
    let obj_set = build_obj_set(obj_tab); 

    // For every object in obj_set
    for object in &obj_set.tab{
        let tuple = process_mesh(object.mesh.clone(), object);
        // The RigidBody associated to the object is at position 0 of the tuple
        let rb = tuple.0; 
        // We add the RigidBody to the RigidBodySet
        let rb_handle = bodies.insert(rb);

        // ### FOR TESTING PURPOSE ONLY ###
        if object.position.y == 0 as f32 {
            let rb = bodies.get_mut(rb_handle).expect("Rigid-body not found.");
            rb.set_status(BodyStatus::Kinematic);
        }
        // ### END OF TESTING ###

        // We create the collider relative to the shape of 'object'
        // The shape (Ball, Triangle, ...) associated to the object is at position 1 of the tuple
        let collider = ColliderDesc::new(tuple.1)
        .density(object.density)
        // Permet de définir si l'objet rebondit (restitution, friction)
        .material(MaterialHandle::new(BasicMaterial::new(object.restitution, object.friction)))
        .build(BodyPartHandle(rb_handle, 0));
        
        // We add the Collider to the set of colliders
        let coll_handle = colliders.insert(collider);

        // Wa add the handle to the coll_tab
        coll_tab.push(coll_handle);
    }

    loop {
        // The universe is now running/ticking 60 times per second
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );
        // ### FOR TESTING PURPOSE ONLY ###
        // Prints the object's coordinates (Ball)
        println!("{}", colliders.get(coll_tab[0]).unwrap().position());
        // ### END OF TESTING ###
    }
}

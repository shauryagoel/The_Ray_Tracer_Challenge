mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod matrix_small;
mod projectile;
mod ray;
mod sphere;
mod transformation;
mod tuple;
mod utils;
mod world;

pub use canvas::Canvas;
pub use color::Color;
pub use intersection::{Computation, Intersection, Intersections};
pub use light::Light;
pub use material::Material;
pub use matrix::Matrix;
pub use matrix_small::{Matrix2, Matrix3};
pub use projectile::{Environment, Projectile};
pub use ray::Ray;
pub use sphere::Sphere;
pub use tuple::{point, vector, Tuple};
pub use utils::Compare;
pub use world::World;

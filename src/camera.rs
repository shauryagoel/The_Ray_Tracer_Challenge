use crate::point;
use crate::Canvas;
use crate::Matrix;
use crate::Ray;
use crate::World;

// Lets us take pictures of the scene
// Main responsibility is to map the 3D scene to a 2D canvas, by projecting rays through the camera to the canvas
// The camera's canvas will always be exactly one unit in front of the camera
pub struct Camera {
    hsize: u16,            // Horizontal size in pixels of the canvas
    vsize: u16,            // Vertical size in pixels of the canvas
    field_of_view: f64,    // An angle that describes how much the camera can see
    pub transform: Matrix, // Transformation matrix that describes how the world is moved relative to the camera (is a view transform)
    aspect: f64,           // Ascpect ratio of the canvas
    half_width: f64,       // Just half of the width of the canvas
    half_height: f64,      // Just half of the height of the canvas
    pixel_size: f64,       // Size of a single pixel
}

impl Camera {
    pub fn new(hsize: u16, vsize: u16, field_of_view: f64) -> Camera {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = hsize as f64 / vsize as f64;
        let (mut half_width, mut half_height) = (half_view, half_view);
        if aspect >= 1.0 {
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
        }
        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::I(),
            aspect,
            half_width,
            half_height,
            pixel_size,
        }
    }

    /// Returns a ray that starts at the camera and passes through the (x,y) pixel on the canvas
    /// Camera is at origin and canvas is at (0, 0, -1)
    pub fn ray_for_pixel(&self, x: u16, y: u16) -> Ray {
        // Get the pixel center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // Change pixel coordinates to world coordinates
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // Transform the point in the canvas
        // Basically, move the camera relative to the world
        let camera_transform_inv = self.transform.inverse();
        let pixel = camera_transform_inv * point(world_x, world_y, -1.0);
        let origin = camera_transform_inv * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    /// Renders the world with the camera and returns the canvas
    /// A ray is casted through the pixel and the pixel is colored with the corresponding intersection
    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize as usize, self.vsize as usize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                canvas.write_pixel(x as usize, y as usize, color);
            }
        }
        canvas
    }
}

#[cfg(test)]
mod camera_test {
    use super::*;
    use crate::{vector, Color};
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn constructing_camera() {
        let hsize: u16 = 160;
        let vsize: u16 = 120;
        let field_of_view = FRAC_PI_2;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, Matrix::I());
    }

    #[test]
    fn ray_through_center_of_canvs() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.transform = Matrix::get_rotation_y_matrix(FRAC_PI_4)
            * Matrix::get_translation_matrix(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, vector(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
    }

    #[test]
    fn render_a_world() {
        let w = World::default();
        let mut c = Camera::new(11, 11, FRAC_PI_2);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = Matrix::get_view_transform(from, to, up);
        let canvas = c.render(&w);
        assert_eq!(canvas.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}

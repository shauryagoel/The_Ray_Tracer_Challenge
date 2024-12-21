use ray_tracer::{point, Canvas, Color, Ray, Sphere};
// use std::f64::consts::PI;

// Cast a shadow of a sphere on a canvas located at `canvas_z` parallel to the `xy` axis
// Ray is cast from the +ve z coordinate towards the canvas
// For each point in the canvas we find the ray and whether it hits the sphere
// If it hits the sphere, then, we draw that point on the canvas
fn main() {
    let canvas_size: usize = 350; // in pixels
    let canvas_z: f64 = -200.0;
    let ray_origin = point(0.0, 0.0, 2.0);
    let sphere_color = Color::new(1.0, 0.0, 0.0);

    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let s: Sphere = Default::default();
    // s.set_transform(Matrix::get_scaling_matrix(1.0, 0.5, 1.0));
    // s.set_transform(Matrix::get_scaling_matrix(0.5, 1.0, 1.0));
    // s.set_transform(
    //     Matrix::get_rotation_z_matrix(PI / 4.0) * Matrix::get_scaling_matrix(0.5, 1.0, 1.0),
    // );
    // s.set_transform(
    //     Matrix::get_shearing_matrix(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    //         * Matrix::get_scaling_matrix(0.5, 1.0, 1.0),
    // );

    let half_width: i32 = (canvas_size as i32) / 2;
    for x in -half_width..half_width {
        for y in (-half_width + 1)..=half_width {
            let ray_direction = point(x as f64, y as f64, canvas_z) - ray_origin;
            let r = Ray::new(ray_origin, ray_direction.normalize());

            let xs = s.intersect(r);
            if xs.hit().is_some() {
                // Translate with respect to cavas coordinate space
                let translated_x = (x + half_width) as usize;
                let translated_y = (half_width - y) as usize;
                canvas.write_pixel(translated_x, translated_y, sphere_color);
            }
        }
    }

    let ppm_string = canvas.get_ppm();
    canvas.write_ppm(&ppm_string, "chapter5_end.ppm");
}

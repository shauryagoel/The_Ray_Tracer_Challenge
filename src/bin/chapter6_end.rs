use ray_tracer::{point, Canvas, Color, Light, Material, Ray, Sphere};
// use std::f64::consts::PI;

// shading routine of a sphere on a canvas located at `canvas_z` parallel to the `xy` axis
// Ray is cast from the -ve z coordinate towards the canvas
// For each point in the canvas we find the ray and whether it hits the sphere
// If it hits the sphere, then, we calcuate the color based on the `Phong reflection model` and draw it on canvas
fn main() {
    let canvas_size: usize = 600; // in pixels
    let canvas_z: f64 = 400.0;
    let ray_origin = point(0.0, 0.0, -2.0); // Basically eye position
    let light = Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let m: Material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ..Default::default()
    };

    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let mut s: Sphere = Default::default();
    s.material = m;
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
            if let Some(hit) = xs.hit() {
                let hit_point = r.position(hit.t);
                let point_normal = hit.object.normal_at(hit_point);
                let eye_vector = -r.direction;

                let calculated_color =
                    hit.object
                        .material
                        .lighting(light, hit_point, eye_vector, point_normal, false);

                // Translate with respect to cavas coordinate space
                let translated_x = (x + half_width) as usize;
                let translated_y = (half_width - y) as usize;
                canvas.write_pixel(translated_x, translated_y, calculated_color);
            }
        }
    }

    let ppm_string = canvas.get_ppm();
    canvas.write_ppm(&ppm_string, "chapter6_end.ppm");
}

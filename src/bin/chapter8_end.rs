use ray_tracer::{point, vector, Camera, Color, Light, Matrix, Sphere, World};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

fn main() {
    let mut floor = Sphere::default();
    floor.set_transform(Matrix::get_scaling_matrix(10.0, 0.01, 10.0));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transform(
        Matrix::get_translation_matrix(0.0, 0.0, 5.0)
            * Matrix::get_rotation_y_matrix(-FRAC_PI_4)
            * Matrix::get_rotation_x_matrix(FRAC_PI_2)
            * Matrix::get_scaling_matrix(10.0, 0.01, 10.0),
    );
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.set_transform(
        Matrix::get_translation_matrix(0.0, 0.0, 5.0)
            * Matrix::get_rotation_y_matrix(FRAC_PI_4)
            * Matrix::get_rotation_x_matrix(FRAC_PI_2)
            * Matrix::get_scaling_matrix(10.0, 0.01, 10.0),
    );
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.set_transform(Matrix::get_translation_matrix(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.set_transform(
        Matrix::get_translation_matrix(1.5, 0.5, -0.5) * Matrix::get_scaling_matrix(0.5, 0.5, 0.5),
    );
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.set_transform(
        Matrix::get_translation_matrix(-1.5, 0.33, -0.75)
            * Matrix::get_scaling_matrix(0.33, 0.33, 0.33),
    );
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World {
        light: Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
        ..Default::default()
    };
    world.objects.clear();
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    let mut camera = Camera::new(1000, 500, FRAC_PI_3);
    // let mut camera = Camera::new(100, 50, FRAC_PI_3);
    camera.transform = Matrix::get_view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);
    let ppm_string = canvas.get_ppm();
    canvas.write_ppm(&ppm_string, "chapter8_end.ppm");
}

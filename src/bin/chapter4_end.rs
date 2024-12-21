use ray_tracer::{point, Canvas, Color, Matrix};
use std::f64::consts::PI;

// Draw pixels around a circle where hours are present in a clock, center of clock is center of canvas
fn main() {
    let mut canvas = Canvas::new(100, 100);
    let hour_color = Color::white();

    let translate_to_canvas_centre = Matrix::get_translation_matrix(
        (canvas.width as f64) / 2.0,
        (canvas.height as f64) / 2.0,
        0.0,
    );

    // point to get the location of hours (start at 12)
    let mut p = point(0.0, -40.0, 0.0);

    // Get rotation matrix which rotates by hour
    let hour_in_radian = 2.0 * PI / 12.0;
    let rotation_of_hour = Matrix::get_rotation_z_matrix(hour_in_radian);

    // Keep rotation p at every iteration, but, when need to draw then
    // translate to center of canvas
    for _ in 0..12 {
        let t = translate_to_canvas_centre * p;
        canvas.write_pixel(t.x as usize, t.y as usize, hour_color);
        p = rotation_of_hour * p;
    }

    // Save the canvas to a file
    let ppm_string = canvas.get_ppm();
    canvas.write_ppm(&ppm_string, "chapter4_end.ppm");
}

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FPS: usize = 60;
const POINT_COLOR: u32 = 0x55FF55;

// Rotating + fading cube
fn main() {
    let mut window = Window::new(
        "Moving away",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(FPS);

    let size = (WIDTH, HEIGHT);
    let mut buffer = [0x000000; WIDTH * HEIGHT];
    let object = Object::Circle { radius: 9 };

    let mut delta_z = 0.0;
    buffer.fill(0x000000);

    // every negative 'z' means the object is "behind us"
    let vertices = [
        (1., 1., -1.),
        (-1., 1., -1.),
        (-1., -1., -1.),
        (1., -1., -1.),
        (1., 1., 1.),
        (-1., 1., 1.),
        (-1., -1., 1.),
        (1., -1., 1.),
    ];

    // lines between vertices (indexing vertices array)
    let lines = [
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![0, 4],
        vec![1, 5],
        vec![2, 6],
        vec![3, 7],
    ];

    let mut angle = 0.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0x000000);

        // -- [start] Rendering
        for xyz in vertices {
            let (x, y) = screen_coordinates(
                project_3d(translate_z(rotate_xz(xyz, angle), delta_z)),
                size.0,
                size.1,
            );
            object.draw(&mut buffer, x, y, size.0, size.1, POINT_COLOR);
        }

        // Draw edges
        for indexes in &lines {
            let mut i = 0;
            while i < indexes.len() {
                let a = vertices[indexes[i]];
                let b = vertices[indexes[(i + 1) % indexes.len()]];

                let (a_x, a_y) = screen_coordinates(
                    project_3d(translate_z(rotate_xz(a, angle), delta_z)),
                    size.0,
                    size.1,
                );
                let (b_x, b_y) = screen_coordinates(
                    project_3d(translate_z(rotate_xz(b, angle), delta_z)),
                    size.0,
                    size.1,
                );

                bresenham(&mut buffer, a_x, a_y, b_x, b_y, size.0, size.1, POINT_COLOR);

                i += 1;
            }
        }

        let delta_t = 1. / (FPS as f32);
        delta_z += 1. * delta_t;
        angle += std::f32::consts::PI * delta_t;
        // --  [end]  Rendering

        window.update_with_buffer(&buffer, size.0, size.1).unwrap();
    }
}

fn screen_coordinates((x, y): (f32, f32), width: usize, height: usize) -> (usize, usize) {
    // <-1; 1>  =>  <0; 2> => <0; 1> => <0; width / height>
    (
        ((x + 1.) / 2. * (width as f32)) as usize,
        ((1. - (y + 1.) / 2.) * (height as f32)) as usize,
    )
}

fn project_3d((x, y, z): (f32, f32, f32)) -> (f32, f32) {
    const NEAR_PLANE: f32 = -0.0;
    if z < NEAR_PLANE {
        (0., 0.)
    } else {
        (x / z, y / z)
    }
}

fn translate_z((x, y, z): (f32, f32, f32), delta_z: f32) -> (f32, f32, f32) {
    (x, y, z + delta_z)
}

// https://en.wikipedia.org/wiki/Rotation_matrix
fn rotate_xz((x, y, z): (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let s = angle.sin();
    let c = angle.cos();
    (x * c - z * s, y, x * s + z * c)
}

#[allow(clippy::too_many_arguments)]
fn bresenham(
    buffer: &mut [u32],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < width as isize && y0 >= 0 && y0 < height as isize {
            buffer[y0 as usize * width + x0 as usize] = color;
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

#[allow(dead_code)]
enum Object {
    Point,
    Circle { radius: usize },
    Square { radius: usize },
}

impl Object {
    fn draw(
        &self,
        buffer: &mut [u32],
        center_x: usize,
        center_y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        match self {
            Object::Point => draw_point(buffer, center_x, center_y, width, color),
            Object::Circle { radius } => {
                draw_circle(buffer, center_x, center_y, width, height, *radius, color)
            }
            Object::Square { radius } => {
                draw_square(buffer, center_x, center_y, width, height, *radius, color)
            }
        }
    }
}

fn draw_point(buffer: &mut [u32], center_x: usize, center_y: usize, width: usize, color: u32) {
    buffer[center_y * width + center_x] = color;
}

fn draw_square(
    buffer: &mut [u32],
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
    radius: usize,
    color: u32,
) {
    for dy in -(radius as isize)..=(radius as isize) {
        for dx in -(radius as isize)..=(radius as isize) {
            let x = center_x as isize + dx;
            let y = center_y as isize + dy;

            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                let index = y as usize * width + x as usize;
                buffer[index] = color;
            }
        }
    }
}

fn draw_circle(
    buffer: &mut [u32],
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
    radius: usize,
    color: u32,
) {
    let radius_sq = (radius * radius) as f32;

    for dy in -(radius as isize)..=(radius as isize) {
        for dx in -(radius as isize)..=(radius as isize) {
            let dist_sq = (dx * dx + dy * dy) as f32;

            if dist_sq <= radius_sq {
                let x = center_x as isize + dx;
                let y = center_y as isize + dy;

                if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                    let index = y as usize * width + x as usize;
                    buffer[index] = color;
                }
            }
        }
    }
}

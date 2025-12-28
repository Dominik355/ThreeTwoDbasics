use minifb::{Window, WindowOptions};
use three_two_d_basics::{draw_line, draw_pixel};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let geometry = GeometryDrawer::new(WIDTH);

    let mut window = Window::new(
        "minifb_geometry - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let _ = geometry.draw_box(&mut buffer, 120, 130, 220, 230, 0xffff00);
    let _ = geometry.draw_circle(&mut buffer, 320, 180, 50, 0xffffff);
    let _ = geometry.draw_rectangle(&mut buffer, 420, 130, 520, 230, 5, 0x00ff00);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

pub struct GeometryDrawer {
    window_width: usize,
}

impl GeometryDrawer {
    pub fn new(window_width: usize) -> Self {
        Self { window_width }
    }

    pub fn draw_box(
        &self,
        buffer: &mut [u32],
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        color: u32,
    ) -> three_two_d_basics::Result<()> {
        for i in start_x..end_x {
            for j in start_y..end_y {
                draw_pixel(buffer, i, j, color, self.window_width)?;
            }
        }
        Ok(())
    }

    pub fn screen_clear(
        &self,
        buffer: &mut [u32],
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> three_two_d_basics::Result<()> {
        self.draw_box(buffer, start_x, start_y, end_x, end_y, 0x00_00_00)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_rectangle(
        &self,
        buffer: &mut [u32],
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        border_thickness: usize,
        color: u32,
    ) -> three_two_d_basics::Result<()> {
        for i in 0..border_thickness {
            draw_line(
                buffer,
                start_x + i,
                start_y,
                start_x + i,
                end_y,
                color,
                self.window_width,
            )?;
            draw_line(
                buffer,
                end_x - i - 1,
                start_y,
                end_x - i - 1,
                end_y,
                color,
                self.window_width,
            )?;
            draw_line(
                buffer,
                start_x,
                start_y + i,
                end_x,
                start_y + i,
                color,
                self.window_width,
            )?;
            draw_line(
                buffer,
                start_x,
                end_y - i - 1,
                end_x,
                end_y - i - 1,
                color,
                self.window_width,
            )?;
        }
        Ok(())
    }

    pub fn draw_circle(
        &self,
        buffer: &mut [u32],
        start_x: usize,
        start_y: usize,
        radius: usize,
        color: u32,
    ) -> three_two_d_basics::Result<()> {
        let mut d: isize = 3 - 2 * radius as isize;
        let mut x = 0;
        let mut y = radius;
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x as isize - y as isize) + 10;
            } else {
                d = d + 4 * x as isize + 6;
            }
            draw_pixel(buffer, start_x + x, start_y + y, color, self.window_width)?;
            draw_pixel(buffer, start_x - x, start_y + y, color, self.window_width)?;
            draw_pixel(buffer, start_x + x, start_y - y, color, self.window_width)?;
            draw_pixel(buffer, start_x - x, start_y - y, color, self.window_width)?;
            draw_pixel(buffer, start_x + y, start_y + x, color, self.window_width)?;
            draw_pixel(buffer, start_x - y, start_y + x, color, self.window_width)?;
            draw_pixel(buffer, start_x + y, start_y - x, color, self.window_width)?;
            draw_pixel(buffer, start_x - y, start_y - x, color, self.window_width)?;
        }
        Ok(())
    }
}

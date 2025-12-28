use bresenham::Bresenham;

pub mod cube;
pub mod penguin;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn draw_line(
    buffer: &mut [u32],
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    color: u32,
    window_width: usize,
) -> Result<()> {
    for (x, y) in Bresenham::new(
        (start_x as isize, start_y as isize),
        (end_x as isize, end_y as isize),
    ) {
        draw_pixel(buffer, x as usize, y as usize, color, window_width)?;
    }
    Ok(())
}

pub fn draw_pixel(
    buffer: &mut [u32],
    x: usize,
    y: usize,
    color: u32,
    window_width: usize,
) -> Result<()> {
    if (y * window_width + x) < buffer.len() {
        buffer[y * window_width + x] = color;
        Ok(())
    } else {
        Err(format!("x: {x} y: {y}").into())
    }
}

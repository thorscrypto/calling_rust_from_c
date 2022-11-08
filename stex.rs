#![crate_type = "staticlib"]

#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}

#[no_mangle]
pub extern fn move_point(p: Point, x_diff: f64, y_diff: f64) -> Point {
    Point { x: p.x + x_diff, y: p.y + y_diff }
}

#[no_mangle]
pub extern fn move_point_inplace(p: &mut Point, x_diff: f64, y_diff: f64) -> () {
    p.x += x_diff;
    p.y += y_diff;
}

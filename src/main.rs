
use macroquad::prelude::*;

const DOGE_DATA: &'static [u8] = include_bytes!("../Doge.png");


#[macroquad::main("gui_test")]
async fn main() {
    let doge_texture = Texture2D::from_file_with_format(DOGE_DATA, None);
    let mut box_position = Vec3::ZERO;
    let mut box_size = Vec3::splat(5.0);

    let mut camera = Camera3D { 
        position: Vec3::splat(10.0), 
        target: Vec3::ZERO, 
        up: Vec3::Y, 
        fovy: 45.0, 
        projection: Projection::Perspective,
        ..Default::default()
    };

    loop {
        clear_background(WHITE);

        // draw_hexagon(screen_width() / 2.0, screen_height() / 2.0,100.0, 1.0, false, BLUE, GREEN);
        set_camera(&camera);
        draw_grid(20, 1.0, GREEN, BLUE);
        draw_cube(box_position, box_size, Some(&doge_texture), WHITE);
        draw_cube_wires(box_position, box_size, BLACK);

        let input = get_input();
        println!("\x1B[2J\x1B[1;1H\n{:#?}", input);
        
        box_position += input.velocity;
        box_size += input.delta_size;

       




        next_frame().await;
    }

}


#[derive(Debug)]
struct Input {
    velocity: Vec3,
    delta_size: Vec3,
}

fn get_input() -> Input {
    let mut velocity = Vec3::ZERO;
    let mut delta_size = Vec3::ZERO;
    
    if is_key_down(KeyCode::Up) {
        velocity.y += 1.0;
    }
    if is_key_down(KeyCode::Down) {
        velocity.y -= 1.0;
    }
    if is_key_down(KeyCode::Left) {
        velocity.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) {
        velocity.x += 1.0;
    }
    if is_key_down(KeyCode::PageUp) {
        velocity.z += 1.0;
    }
    if is_key_down(KeyCode::PageDown) {
        velocity.z -= 1.0;
    }
    let velocity = velocity.normalize_or_zero() * get_frame_time();

    if is_key_down(KeyCode::W) {
        delta_size.y += 1.0;
    }
    if is_key_down(KeyCode::S) {
        delta_size.y -= 1.0;
    }
    if is_key_down(KeyCode::A) {
        delta_size.x -= 1.0;
    }
    if is_key_down(KeyCode::D) {
        delta_size.x += 1.0;
    }
    if is_key_down(KeyCode::E) {
        delta_size.z += 1.0;
    }
    if is_key_down(KeyCode::Q) {
        delta_size.z -= 1.0;
    }
    let delta_size = delta_size.normalize_or_zero() * get_frame_time();

    return Input { velocity, delta_size };
}

use std::{io, thread::sleep, time::Duration};

use nannou::{App, Frame};
use nannou::color::{BLACK, GRAY, WHITE, RED};
use nannou::event::{Update, Key};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    draw_mode: [bool; 2],
    alive: Vec<(f32, f32)>,
    running: bool,
    speed: u32,
    marker: (f32, f32),
    markermode: bool,
    zoom_scale: f32,
    movement_offset: [f32; 2],
}

fn model(app: &App) -> Model {
    let draw_mode = [false; 2];
    let alive: Vec<(f32, f32)> = Vec::new();
    let running = false;
    let movement_offset = [0.0; 2];
    let markermode: bool = false;
    let marker: (f32, f32) = (0.0, 0.0);
    let zoom_scale = 10.0;
    let mut speedinput = true;
    let mut speed = 1000;
    while speedinput {
        println!("Enter the speed of the simulation (in milliseconds)");
        let mut speedstr = String::new();
        io::stdin()
            .read_line(&mut speedstr)
            .expect("Failed to read line");
    
        let speedstr: u32 = match speedstr.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        speed = speedstr;
        speedinput = false;
    }
    app
        .new_window()
        .size(1005,1005)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model { draw_mode, alive, running, movement_offset, speed, marker, markermode, zoom_scale }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

    if _model.running == false { //Runs during the drawing phase of the program.
        if _app.mouse.buttons.left().is_down() { //if the LMB is held down, draw/remove cells at the mouse position.
            let pos = ((_app.mouse.x/_model.zoom_scale - _model.movement_offset[0]).round(), (_app.mouse.y/_model.zoom_scale - _model.movement_offset[1]).round());
            if _model.alive.contains(&pos) && !_model.draw_mode[1] { //checks if the last cell was drawn here, and if it was do not attempt to draw again.
                _model.alive.retain(|&x| x != pos);
                _model.draw_mode[0] = true;
            } else if !_model.alive.contains(&pos) && !_model.draw_mode[0] {
                _model.alive.push(pos);
                _model.draw_mode[1] = true;
            }
        } else if _app.mouse.buttons.left().is_up() {
            _model.draw_mode = [false; 2];
        }
    } else {
        let mut dying: Vec<(f32, f32)> = Vec::new();
        let mut born: Vec<(f32, f32)> = Vec::new();
        for cell in _model.alive.iter() {
            let neighbors = find_neighbors(&cell);
            let mut living_neighbors: u8 = 0;
            for neighbor in neighbors.iter() {
                if _model.alive.contains(neighbor) {
                    living_neighbors += 1;
                } else {
                    let neighbors_neighbors = find_neighbors(neighbor);
                    let mut neighbors_living_neighbors: u8 = 0;
                    for neighbor_neighbor in neighbors_neighbors.iter() {
                        if _model.alive.contains(neighbor_neighbor) {
                            neighbors_living_neighbors += 1;
                        }
                    }
                    if neighbors_living_neighbors == 3 && !born.contains(neighbor) {
                        born.push(*neighbor);
                    }
                }
            }
            if living_neighbors > 3 || living_neighbors < 2 && !dying.contains(cell) {
                dying.push(*cell);
            }
        }
        for cell in dying.iter() {
            if _model.alive.contains(cell) {
                _model.alive.retain(|&x| x != *cell);
            }
        }
        for cell in born.iter() {
            if !_model.alive.contains(cell) {
                _model.alive.push(*cell);
            }
        }
        dying.clear();
        born.clear();

        if key_down(_app, _model,&Key::I) {
            _model.zoom_scale += 0.5;
        } if key_down(_app, _model,&Key::O) {
            _model.zoom_scale -= 0.5;
        }

        sleep(Duration::from_millis(_model.speed.into()));
        
    }
}

fn find_neighbors(pos: &(f32, f32)) -> Vec<(f32, f32)> {
    let mut neighbors: Vec<(f32, f32)> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                neighbors.push((pos.0 + x as f32, pos.1 + y as f32));
            }
        }
    }
    neighbors
}

fn key_down(app: &App, _model: &mut Model, key: &Key) -> bool{
    app.keys.down.contains(&key)
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if !model.running {
        match key {
            Key::Tab => {
                model.markermode = !model.markermode;
            }
            Key::Space => {
                if model.markermode {
                    if model.alive.contains(&(0.0 - model.movement_offset[0], 0.0 - model.movement_offset[1])) {
                        model.alive.retain(|&x| x != (0.0 - model.movement_offset[0], 0.0 - model.movement_offset[1]));
                    } else {
                        model.alive.push((0.0 - model.movement_offset[0], 0.0 - model.movement_offset[1]));
                    }
                }
            }
            _other_key => {}
        }
    }
    match key {
        Key::Left => {
            model.movement_offset[0] += 1.0;
        }
        Key::Right => {
            model.movement_offset[0] -= 1.0;
        }
        Key::Up => {
            model.movement_offset[1] -= 1.0;
        }
        Key::Down => {
            model.movement_offset[1] += 1.0;
        }
        Key::Return => {
            model.running = !model.running;
        }
        Key::I => {
            model.zoom_scale += 0.5;
            //model.movement_offset = [(model.movement_offset[0]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0, (model.movement_offset[1]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0];
        }
        Key::O => {
            model.zoom_scale -= 0.5;
            //model.movement_offset = [(model.movement_offset[0]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0, (model.movement_offset[1]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0];
        }
        _other_key => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();


    draw.background().color(BLACK);
    let bottomleft: (i16, i16) = ((app.window_rect().bottom_left().x/model.zoom_scale).ceil() as i16 - 1, (app.window_rect().bottom_left().y/model.zoom_scale).ceil() as i16 - 1);
    let topright: (i16, i16) = ((app.window_rect().top_right().x/model.zoom_scale).ceil() as i16 + 1, (app.window_rect().top_right().y/model.zoom_scale).ceil() as i16 + 1);
    let width = (topright.0 - bottomleft.0) as f32 * model.zoom_scale;
    let height = (topright.1 - bottomleft.1) as f32 * model.zoom_scale;

    //display marker and grid for editing mode
    if !model.running {
        for i in bottomleft.0..topright.0 {
            draw.rect()
                .x_y((i as f32 + 0.5) * model.zoom_scale, 0.0)
                .w_h(1.0, height)
                .color(GRAY);
        }
        for i in bottomleft.1..topright.1 {
            draw.rect()
                .x_y(0.0, (i as f32 + 0.5) * model.zoom_scale)
                .w_h(width, 1.0)
                .color(GRAY);
        }
    }

    //loop through every alive cell and draw a rectangle at that coordinate
    for i in &model.alive {
        draw.rect()
            .x_y((i.0 + model.movement_offset[0]) * model.zoom_scale, (i.1 + model.movement_offset[1]) * model.zoom_scale)
            .w_h(model.zoom_scale, model.zoom_scale)
            .color(WHITE);
    }
    if !model.running && model.markermode {
        draw.rect()
            .x_y(model.marker.0 * model.zoom_scale, model.marker.1 * model.zoom_scale)
            .w_h(model.zoom_scale + 3.0, model.zoom_scale + 3.0)
            .color(RED);
        if model.alive.contains(&(0.0 - model.movement_offset[0], 0.0 - model.movement_offset[1])) {
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(model.zoom_scale, model.zoom_scale)
                .color(WHITE);
        } else {
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(model.zoom_scale, model.zoom_scale)
                .color(BLACK);
        }
    }
    if app.elapsed_frames() % 120 == 0 {
        println!("{:?}", model.alive);
    }
    //draw frame
    draw.to_frame(app, &frame).unwrap();
    
}

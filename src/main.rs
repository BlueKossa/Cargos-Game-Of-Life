use nannou::prelude::*;
use std::io;
use std::{thread::sleep, time::Duration};

const SIZE: f32 = 1005.0;

const GRIDSIZE: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    has_drawn: bool,
    last_cell: (f32, f32),
    alive: Vec<(f32, f32)>,
    running: bool,
    speed: u32,
    marker: (f32, f32),
    markermode: bool,
    zoom_scale: f32,
}

fn model(app: &App) -> Model {
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
        .size(SIZE as u32,SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    let has_drawn = false;
    let last_cell = (0.1, 0.1);
    let alive: Vec<(f32, f32)> = Vec::new();
    let running = false;
    let markermode: bool = false;
    let marker: (f32, f32) = (0.0, 0.0);
    let zoom_scale = GRIDSIZE;
    Model { has_drawn, last_cell, alive, running, speed, marker, markermode, zoom_scale }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.running == false {
        if _app.mouse.buttons.left().is_down() /* && !_model.has_drawn */ {
            _model.has_drawn = true;
            let pos = ((_app.mouse.x/_model.zoom_scale).round(), (_app.mouse.y/_model.zoom_scale).round());
            if _model.alive.contains(&pos) && _model.last_cell != pos {
                _model.alive.retain(|&x| x != pos);
            } else if _model.last_cell != pos {
                _model.alive.push(pos);
            }
            _model.last_cell = pos;
            _model.has_drawn = true;
        } else if _app.mouse.buttons.left().is_up() {
            _model.has_drawn = false;
        }
        let mut moving = false;
        if key_down(_app, _model,&Key::Return) {
            _model.running = true;
        } if key_down(_app, _model,&Key::Tab) {
            _model.markermode = !_model.markermode;
            moving = true;

        } if _model.markermode == true && key_down(_app, _model,&Key::Left) {
            _model.marker.0 -= 1.0;
            moving = true;
        } if _model.markermode == true && key_down(_app, _model,&Key::Right) {
            _model.marker.0 += 1.0;
            moving = true;
        } if _model.markermode == true && key_down(_app, _model,&Key::Up) {
            _model.marker.1 += 1.0;
            moving = true;
        } if _model.markermode == true && key_down(_app, _model,&Key::Down) {
            _model.marker.1 -= 1.0;
            moving = true;
        } if _model.markermode == true && key_down(_app, _model,&Key::Space) {
            if _model.alive.contains(&_model.marker) && _model.last_cell != _model.marker {
                _model.alive.retain(|&x| x != _model.marker);
            } else if _model.last_cell != _model.marker {
                _model.alive.push(_model.marker);
            }
            _model.last_cell = _model.marker;
            moving = true;
        }

        if key_down(_app, _model,&Key::I) {
            _model.zoom_scale += 0.3;
            moving = true;
        } if key_down(_app, _model,&Key::O) {
            _model.zoom_scale -= 0.3;
            moving = true;
        }


        if moving == true {
            sleep(Duration::from_millis(100));
        }
        if !_app.mouse.buttons.left().is_down() && !key_down(_app, _model,&Key::Space) {
            _model.last_cell = (0.1,0.1);
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
            .x_y(i.0 * model.zoom_scale, i.1 * model.zoom_scale)
            .w_h(model.zoom_scale, model.zoom_scale)
            .color(WHITE);
    }
    if !model.running && model.markermode {
        draw.rect()
            .x_y(model.marker.0 * model.zoom_scale, model.marker.1 * model.zoom_scale)
            .w_h(13.0, 13.0)
            .color(RED);
        if model.alive.contains(&model.marker) {
            draw.rect()
                .x_y(model.marker.0 * model.zoom_scale, model.marker.1 * model.zoom_scale)
                .w_h(10.0, 10.0)
                .color(WHITE);
        } else {
            draw.rect()
                .x_y(model.marker.0 * model.zoom_scale, model.marker.1 * model.zoom_scale)
                .w_h(10.0, 10.0)
                .color(BLACK);
        }
    }
    //draw frame
    draw.to_frame(app, &frame).unwrap();
    
}
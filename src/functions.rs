use std::collections::HashSet;
use std::{io, thread::sleep, time::Duration};

use nannou::{App, Frame};
use nannou::color::{BLACK, GRAY, WHITE, RED, LAWNGREEN, srgba, rgba, DARKGRAY};
use nannou::event::{Update};

use crate::keyboard_functions::{brush, selector, key_pressed};

pub struct Model {
    pub draw_mode: [bool; 2],
    pub start_pos: (i32, i32),
    pub current_pos: (i32, i32),
    pub selector_active: bool,
    pub clipboard: HashSet<(i32, i32, bool)>,
    pub sel_points: HashSet<(i32, i32)>,
    pub alive_hash: HashSet<(i32, i32)>,
    pub last_alive_count: i32,
    pub running: bool,
    pub speed: u32,
    pub marker: (f32, f32),
    pub markermode: bool,
    pub zoom_scale: f32,
    pub movement_offset: [i32; 2],
}

pub fn model(app: &App) -> Model {
    let draw_mode = [false; 2];
    let selector_active = false;
    let start_pos = (0, 0);
    let current_pos = (0, 0);
    let clipboard = HashSet::new();
    let sel_points = HashSet::new();
    let alive_hash: HashSet<(i32, i32)> = HashSet::new();
    let last_alive_count = 0;
    let running = false;
    let movement_offset = [0; 2];
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
        .fullscreen()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model { draw_mode, start_pos, current_pos, selector_active, clipboard, sel_points, alive_hash, last_alive_count, running, movement_offset, speed, marker, markermode, zoom_scale }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    println!("poop");
    if _model.running == false { //Runs during the drawing phase of the program.
        brush(_app, _model);
        selector(_app, _model);
    } else { //Runs during the update phase of the program.
        let mut dying: Vec<(i32, i32)> = Vec::new();
        let mut born: Vec<(i32, i32)> = Vec::new();
        _model.last_alive_count = _model.alive_hash.len() as i32;

        for cell in _model.alive_hash.iter() {
            let neighbors = find_neighbors(&cell);
            let mut living_neighbors: u8 = 0;
            for neighbor in neighbors.iter() {
                if _model.alive_hash.contains(neighbor) {
                    living_neighbors += 1;
                } else if !born.contains(neighbor) {
                    let neighbors_neighbors = find_neighbors(neighbor);
                    let mut neighbors_living_neighbors: u8 = 0;
                    for neighbor_neighbor in neighbors_neighbors.iter() {
                        if _model.alive_hash.contains(neighbor_neighbor) {
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
            _model.alive_hash.remove(cell);
        }
        for cell in born.iter() {
            _model.alive_hash.insert(*cell);
        }
        dying.clear();
        born.clear();

        sleep(Duration::from_millis(_model.speed.into()));
        
    }
}

pub fn find_neighbors(pos: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                neighbors.push((pos.0 + x , pos.1 + y));
            }
        }
    }
    neighbors
}


pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();


    draw.background().color(BLACK);
    let bottomleft: (f32, f32) = ((app.window_rect().bottom_left().x/model.zoom_scale), (app.window_rect().bottom_left().y/model.zoom_scale));
    let topright: (f32, f32) = ((app.window_rect().top_right().x/model.zoom_scale), (app.window_rect().top_right().y/model.zoom_scale));
    let width = (topright.0 - bottomleft.0) as f32 * model.zoom_scale;
    let height = (topright.1 - bottomleft.1) as f32 * model.zoom_scale;

    //display marker and grid for editing mode
    if !model.running {
        for i in bottomleft.0.ceil() as i16..topright.0.ceil() as i16 {
            draw.rect()
                .x_y((i as f32 + 0.5) * model.zoom_scale, 0.0)
                .w_h(1.0, height)
                .color(GRAY);
        }
        for i in bottomleft.1.ceil() as i16..topright.1.ceil() as i16 {
            draw.rect()
                .x_y(0.0, (i as f32 + 0.5) * model.zoom_scale)
                .w_h(width, 1.0)
                .color(GRAY);
        }
    }   
    draw.text(&format!("Total Alive: {}", model.alive_hash.len()))
        .w_h(width, height)
        .left_justify()
        .align_text_top()
        .font_size((width / 100.0).round() as u32);
    draw.text(&format!("\n{:>+}", model.alive_hash.len() as i32 - model.last_alive_count))
        .w_h(width, height)
        .left_justify()
        .align_text_top()
        .font_size((width / 100.0).round() as u32)
        .color(if model.alive_hash.len() as i32 - model.last_alive_count > 0 {
        LAWNGREEN
    } else if model.alive_hash.len() as i32 - model.last_alive_count < 0 {
        RED
    } else {
        WHITE
    });
    
    //loop through every alive cell and draw a rectangle at that coordinate
    for i in &model.alive_hash {
        draw.rect()
            .x_y((i.0 + model.movement_offset[0]) as f32 * model.zoom_scale, (i.1 + model.movement_offset[1]) as f32 * model.zoom_scale)
            .w_h(model.zoom_scale, model.zoom_scale)
            .color(WHITE);
    }
    if !model.running && model.markermode {
        draw.rect()
            .x_y(model.marker.0 * model.zoom_scale, model.marker.1 * model.zoom_scale)
            .w_h(model.zoom_scale + 3.0, model.zoom_scale + 3.0)
            .color(RED);
        if model.alive_hash.contains(&(-model.movement_offset[0], -model.movement_offset[1])) {
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
    if !model.running {
        if model.sel_points.len() > 0 {
            let sel_height = (model.start_pos.1 - model.current_pos.1).abs() as f32 + 1.0;
            let sel_width = (model.start_pos.0 - model.current_pos.0).abs() as f32 + 1.0;
            let sel_middle = ((model.start_pos.0 + model.current_pos.0) as f32/2.0, (model.start_pos.1 + model.current_pos.1) as f32/2.0);
            draw.rect()
                .x_y((sel_middle.0 + model.movement_offset[0] as f32) * model.zoom_scale, (sel_middle.1 + model.movement_offset[1] as f32) * model.zoom_scale)
                .w_h(sel_width * model.zoom_scale, sel_height * model.zoom_scale)
                .color(srgba(0.5, 0.5, 0.5, 0.2));
            draw.rect()
                .x_y((sel_middle.0 + model.movement_offset[0] as f32 - (sel_width/2.0)) * model.zoom_scale, (sel_middle.1 + model.movement_offset[1] as f32) * model.zoom_scale)
                .w_h(1.5, sel_height * model.zoom_scale + 1.5)
                .color(DARKGRAY);
            draw.rect()
                .x_y((sel_middle.0 + model.movement_offset[0] as f32 + (sel_width/2.0)) * model.zoom_scale, (sel_middle.1 + model.movement_offset[1] as f32) * model.zoom_scale)
                .w_h(1.5, sel_height * model.zoom_scale + 1.5)
                .color(DARKGRAY);
            draw.rect()
                .x_y((sel_middle.0 + model.movement_offset[0] as f32) * model.zoom_scale, (sel_middle.1 + model.movement_offset[1] as f32 - (sel_height/2.0)) * model.zoom_scale)
                .w_h(sel_width * model.zoom_scale + 1.5, 1.5)
                .color(DARKGRAY);
            draw.rect()
                .x_y((sel_middle.0 + model.movement_offset[0] as f32) * model.zoom_scale, (sel_middle.1 + model.movement_offset[1] as f32 + (sel_height/2.0)) * model.zoom_scale)
                .w_h(sel_width * model.zoom_scale + 1.5, 1.5)
                .color(DARKGRAY);
        }
    }
    //draw frame
    draw.to_frame(app, &frame).unwrap();
}

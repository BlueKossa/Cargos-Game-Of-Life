use nannou::{App, event::Key};
use std::{cmp::{min, max}};
use crate::functions::Model;


pub fn mouse(_app: &App, _model: &mut Model) {
    if _app.keys.down.contains(&Key::LShift) {
        selector(&_app, _model);
    } else {
        brush(&_app, _model);
    }
}
pub fn brush(_app: &App, _model: &mut Model) {
    if _app.mouse.buttons.left().is_down() { //if the LMB is held down, draw/remove cells at the mouse position.
        let pos = ((_app.mouse.x/_model.zoom_scale).round() as i32 - _model.movement_offset[0], (_app.mouse.y/_model.zoom_scale).round() as i32 - _model.movement_offset[1]);
        if !_model.draw_mode[1] && _model.alive_hash.insert(pos) { //checks if the last cell was drawn here, and if it was do not attempt to draw again.
            _model.draw_mode[0] = true;
        } else if !_model.draw_mode[0] && _model.alive_hash.remove(&pos) {
            _model.draw_mode[1] = true;
        }
    } else if _app.mouse.buttons.left().is_up() {
        _model.draw_mode = [false; 2];
    }
}

pub fn selector(_app: &App, _model : &mut Model) {
    if _app.mouse.buttons.left().is_down() && !_model.selector_active {
        _model.start_pos = ((_app.mouse.x/_model.zoom_scale).round() as i32 - _model.movement_offset[0], (_app.mouse.y/_model.zoom_scale).round() as i32 - _model.movement_offset[1]);
        _model.current_pos = _model.start_pos;
        _model.selector_active = true;
    } else if _app.mouse.buttons.left().is_down() && _model.selector_active {
        _model.current_pos = ((_app.mouse.x/_model.zoom_scale).round() as i32 - _model.movement_offset[0], (_app.mouse.y/_model.zoom_scale).round() as i32 - _model.movement_offset[1]);
        _model.sel_points.clear();
        for x in min(_model.start_pos.0, _model.current_pos.0)..max(_model.start_pos.0, _model.current_pos.0) {
            for y in min(_model.start_pos.1, _model.current_pos.1)..max(_model.start_pos.1, _model.current_pos.1) {
                _model.sel_points.insert((x,y));
            }
        }
    } else if _app.mouse.buttons.left().is_up() {
        _model.selector_active = false;
    }
}


pub fn mouse_move(_app: &App, _model: &mut Model) {
    if _app.mouse.buttons.right().is_down() && !_model.moving {
        _model.moving_origin = (_app.mouse.x.round() as i32, _app.mouse.y.round() as i32);
        _model.moving = true;
        _model.moved_points = (0, 0);
    } else if _app.mouse.buttons.right().is_down() && _model.moving {
        let curr_pos = (_app.mouse.x.round() as i32, _app.mouse.y.round() as i32);
        let moved_points = (((_model.moving_origin.0 - curr_pos.0) as f32/_model.zoom_scale).round() as i32, ((_model.moving_origin.1 - curr_pos.1) as f32/_model.zoom_scale).round() as i32);
        _model.movement_offset = [_model.movement_offset[0] - (moved_points.0 - _model.moved_points.0), _model.movement_offset[1] - (moved_points.1 - _model.moved_points.1)];
        _model.moved_points = moved_points;
    } else if _app.mouse.buttons.right().is_up() {
        _model.moving = false;
    }
}


pub fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if !model.running {
        match key {
            Key::Tab => {
                model.markermode = !model.markermode;
            }
            Key::Space => {
                if model.markermode {
                    let pos = (-model.movement_offset[0] as i32, -model.movement_offset[1] as i32);
                    if model.alive_hash.insert(pos) {}
                    else if model.alive_hash.remove(&pos) {}
                }
            }
            Key::C => {
                if _app.keys.down.contains(&Key::LControl) && !model.selector_active {
                    model.clipboard.clear();
                    for point in &model.sel_points {
                        if model.alive_hash.contains(point) {
                            model.clipboard.insert((point.0 - model.start_pos.0, point.1 - model.start_pos.1, true));
                        } else {
                            model.clipboard.insert((point.0 - model.start_pos.0, point.1 - model.start_pos.1, false));
                        }
                    }
                }
            }
            Key::V => {
                if _app.keys.down.contains(&Key::LControl) && !model.selector_active {
                    for point in &model.clipboard {
                        if point.2 {
                            model.alive_hash.insert((point.0 - model.movement_offset[0], point.1 - model.movement_offset[1]));
                        } else {
                            model.alive_hash.remove(&(point.0 - model.movement_offset[0], point.1 - model.movement_offset[1]));
                        }
                    }
                }
            }
            _other_key => {}
        }
    }
    match key {
        Key::Left => {
            model.movement_offset[0] += 1;
        }
        Key::Right => {
            model.movement_offset[0] -= 1;
        }
        Key::Up => {
            model.movement_offset[1] -= 1;
        }
        Key::Down => {
            model.movement_offset[1] += 1;
        }
        Key::Return => {
            model.running = !model.running;
        }
        Key::I => {
            model.zoom_scale += 0.5;
            //model.movement_offset = [(model.movement_offset[0]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0, (model.movement_offset[1]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0];
        }
        Key::O => {
            if model.zoom_scale > 1.5 {
                model.zoom_scale -= 0.5;
            }
            //model.movement_offset = [(model.movement_offset[0]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0, (model.movement_offset[1]/model.zoom_scale*2.0).round()*model.zoom_scale/2.0];
        }
        Key::Q => {
            let colors = model.colors;
            model.colors = [colors[1], colors[0]];
        }
        _other_key => {}
    }
}
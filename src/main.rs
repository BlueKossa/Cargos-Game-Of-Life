mod keyboard_functions;
mod functions;
use functions::*;
use keyboard_functions::*;


use std::collections::HashSet;
use std::{io, thread::sleep, time::Duration};

use nannou::{App, Frame};
use nannou::color::{BLACK, GRAY, WHITE, RED, LAWNGREEN};
use nannou::event::{Update, Key};

fn main() {
    nannou::app(model).update(update).run();
}
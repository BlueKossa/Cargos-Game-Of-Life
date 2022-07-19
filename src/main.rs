mod keyboard_functions;
mod functions;
use functions::*;
fn main() {
    nannou::app(model).update(update).run();
}
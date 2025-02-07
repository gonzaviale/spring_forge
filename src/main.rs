mod print_ascii_art;
mod inputs;
mod structs;
mod generate_crud;

use crate::inputs::inputs::inputs;
use crate::generate_crud::generate_crud;
use crate::print_ascii_art::print_ascii_art;



fn main() {
    print_ascii_art();
    let entities = inputs();
    for entity in entities {
        generate_crud(&entity);
    }
}

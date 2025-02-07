mod print_ascii_art;
mod get_entity_name;

use spring_forge::generate_crud;

use crate::print_ascii_art::print_ascii_art;
use crate::get_entity_name::get_entity_name;



fn main() {
    print_ascii_art();
    let entity_name = get_entity_name();
    generate_crud(&entity_name);
}

use std::collections::HashMap;
use colored::Colorize;
use crate::{inputs::get_input::get_input, structs::Entity};

/// Muestra un menú y obtiene la opción del usuario
fn get_menu_choice(prompt: &str, options: &[&str]) -> String {
    loop {
        println!("{}", prompt);
        let input = get_input();
        if options.contains(&input.as_str()) {
            return input;
        }
        println!("{}", "Invalid input".red().bold());
    }
}

/// Solicita el nombre de la entidad
fn get_entity_name() -> String {
    print!("{}", "Enter the entity name: ".blue().bold());
    get_input()
}

/// Agrega atributos a una entidad
fn add_attributes(entity: &mut Entity) {
    loop {
        let input = get_menu_choice(
            &format!("{}\n{}", 
                "Press 1 to add an attribute: ".blue().bold(),
                "Press 2 to exit: ".yellow().bold()
            ),
            &["1", "2"],
        );

        if input == "1" {
            print!("{}", "Enter the attribute name: ".blue().bold());
            let attribute_name = get_input();
            print!("{}", "Enter the attribute type: ".blue().bold());
            let attribute_type = get_input();
            entity.add_attribute(attribute_name, attribute_type);
            println!("{}", "Attribute added successfully".green().bold());
        } else {
            break;
        }
    }
}

/// Función principal que gestiona la entrada de entidades
pub fn inputs() -> Vec<Entity> {
    let mut entities = Vec::new();

    loop {
        let input = get_menu_choice(
            &format!("{}\n{}", 
                "Press 1 to create a new entity: ".blue().bold(),
                "Press 2 to exit: ".yellow().bold()
            ),
            &["1", "2"],
        );

        if input == "1" {
            let mut entity = Entity::new(String::new(), HashMap::new());
            let entity_name = get_entity_name();
            entity.set_name(entity_name);
            add_attributes(&mut entity);
            entities.push(entity);
        } else {
            break;
        }
    }

    entities
}

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::structs::Entity;

pub fn generate_crud(entity: &Entity) {
    let name = entity.get_name();
    let attributes_str = entity._get_attributes()
    .iter()
    .map(|(field_name, field_type)| format!("    private {} {};", field_type, field_name))
    .collect::<Vec<String>>() 
    .join("\n");
    let dir = "output";
    let dir_models = Path::new(dir).join("models");
    let dir_repositories = Path::new(dir).join("repositories");
    let dir_services = Path::new(dir).join("services");
    let dir_controllers = Path::new(dir).join("controllers");

    fs::create_dir_all(dir_models).expect("Error al crear el directorio");
    fs::create_dir_all(dir_repositories).expect("Error al crear el directorio");
    fs::create_dir_all(dir_services).expect("Error al crear el directorio");
    fs::create_dir_all(dir_controllers).expect("Error al crear el directorio");
    fs::create_dir_all(dir).expect("Error al crear el directorio");

    // 1️⃣ Generar la entidad
    let entity = format!(
        r#"package com.example.demo.models;

import jakarta.persistence.*;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
@Entity
public class {0}Model {{
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
{1}
}}"#, name, attributes_str
    );
    write_file("output/models", &format!("{name}Model.java"), &entity);

    // 2️⃣ Generar el repositorio
    let repository = format!(
        r#"package com.example.demo.repositories;

import com.example.demo.models.{0}Model;
import org.springframework.data.jpa.repository.JpaRepository;

public interface I{0}Repository extends JpaRepository<{0}Model, Long> {{}}
"#, name
    );
    write_file("output/repositories", &format!("I{name}Repository.java"), &repository);

    // 3️⃣ Generar el servicio
    let service = format!(
        r#"package com.example.demo.services;

import com.example.demo.models.{0}Model;
import com.example.demo.repositories.I{0}Repository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@Service
public class {0}Service {{
    @Autowired
    private I{0}Repository repository;

    public List<{0}Model> getAll() {{ 
        return repository.findAll(); 
    }}

    public Optional<{0}Model> getById(Long id) {{ 
        return repository.findById(id); 
    }}

    public {0}Model create({0}Model entity) {{ 
        return repository.save(entity); 
    }}

    public {0}Model update(Long id, {0} entity) {{ 
        entity.setId(id);
        return repository.save(entity); 
    }}

    public void delete(Long id) {{ 
        repository.deleteById(id); 
    }}
}}"#, name
    );
    write_file("output/services", &format!("{name}Service.java"), &service);

    // 4️⃣ Generar el controlador
    let controller = format!(
        r#"package com.example.demo.controllers;

import com.example.demo.models.{0}Model;
import com.example.demo.services.{0}Service;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Optional;

@RestController
@RequestMapping("/{1}")
public class {0}Controller {{
    @Autowired
    private {0}Service service;

    @GetMapping
    public List<{0}Model> getAll() {{
        return service.getAll();
    }}

    @GetMapping("/{{id}}")
    public Optional<{0}Model> getById(@PathVariable Long id) {{
        return service.getById(id);
    }}

    @PostMapping
    public {0}Model create(@RequestBody {0}Model entity) {{
        return service.create(entity);
    }}

    @PutMapping("/{{id}}")
    public {0}Model update(@PathVariable Long id, @RequestBody {0}Model entity) {{
        return service.update(id, entity);
    }}

    @DeleteMapping("/{{id}}")
    public void delete(@PathVariable Long id) {{
        service.delete(id);
    }}
}}"#, name, name.to_lowercase()
    );
    write_file("output/controllers", &format!("{name}Controller.java"), &controller);

    println!("✅ CRUD for `{}` generated successfully in `{}`.", name, dir);
}

fn write_file(dir: &str, filename: &str, content: &str) {
    let path = Path::new(dir).join(filename);
    let mut file = fs::File::create(path).expect("Error al crear el archivo");
    file.write_all(content.as_bytes()).expect("Error al escribir el archivo");
}

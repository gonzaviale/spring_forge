use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_crud(name: &str) {
    let dir = "output";
    fs::create_dir_all(dir).expect("Error al crear el directorio");

    // 1️⃣ Generar la entidad
    let entity = format!(
        r#"package com.example.demo.model;

import jakarta.persistence.*;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
@Entity
public class {0} {{
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
}}"#, name
    );
    write_file(dir, &format!("{name}.java"), &entity);

    // 2️⃣ Generar el repositorio
    let repository = format!(
        r#"package com.example.demo.repositories;

import com.example.demo.model.{0};
import org.springframework.data.jpa.repository.JpaRepository;

public interface I{0}Repository extends JpaRepository<{0}, Long> {{}}
"#, name
    );
    write_file(dir, &format!("I{name}Repository.java"), &repository);

    // 3️⃣ Generar el servicio
    let service = format!(
        r#"package com.example.demo.service;

import com.example.demo.model.{0};
import com.example.demo.repositories.I{0}Repository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@Service
public class {0}Service {{
    @Autowired
    private I{0}Repository repository;

    public List<{0}> getAll() {{ 
        return repository.findAll(); 
    }}

    public Optional<{0}> getById(Long id) {{ 
        return repository.findById(id); 
    }}

    public {0} create({0} entity) {{ 
        return repository.save(entity); 
    }}

    public {0} update(Long id, {0} entity) {{ 
        entity.setId(id);
        return repository.save(entity); 
    }}

    public void delete(Long id) {{ 
        repository.deleteById(id); 
    }}
}}"#, name
    );
    write_file(dir, &format!("{name}Service.java"), &service);

    // 4️⃣ Generar el controlador
    let controller = format!(
        r#"package com.example.demo.controller;

import com.example.demo.model.{0};
import com.example.demo.service.{0}Service;
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
    public List<{0}> getAll() {{
        return service.getAll();
    }}

    @GetMapping("/{{id}}")
    public Optional<{0}> getById(@PathVariable Long id) {{
        return service.getById(id);
    }}

    @PostMapping
    public {0} create(@RequestBody {0} entity) {{
        return service.create(entity);
    }}

    @PutMapping("/{{id}}")
    public {0} update(@PathVariable Long id, @RequestBody {0} entity) {{
        return service.update(id, entity);
    }}

    @DeleteMapping("/{{id}}")
    public void delete(@PathVariable Long id) {{
        service.delete(id);
    }}
}}"#, name, name.to_lowercase()
    );
    write_file(dir, &format!("{name}Controller.java"), &controller);

    println!("✅ CRUD de `{}` generado en la carpeta `{}`.", name, dir);
}

fn write_file(dir: &str, filename: &str, content: &str) {
    let path = Path::new(dir).join(filename);
    let mut file = fs::File::create(path).expect("Error al crear el archivo");
    file.write_all(content.as_bytes()).expect("Error al escribir el archivo");
}

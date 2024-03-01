mod item;
mod list;

use std::io;
use crate::item::Item;
use crate::list::{create_list, show_items, add_item, remove_item, console_add, console_remove, get_item_body, console_get_index};

fn main() -> io::Result<()> {
    // Se crea una lista mutable de tipo `Vec<Item>`
    let mut list: Vec<Item> = create_list();

    loop {
        // Se imprime el menú de opciones
        println!("¿Qué quieres hacer?");
        println!("1. Agregar un elemento");
        println!("2. Mostrar todos los elementos");
        println!("3. Eliminar un elemento");
        println!("4. Mostrar un elemento");
        println!("5. Salir");

        // Se lee la entrada del usuario
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(err) => {
                println!("Error leyendo la entrada: {}", err);
                break;
            }
        }

        // Se procesa la entrada del usuario
        match input.trim() {
            "1" => {
                // Se obtienen el título y la descripción del nuevo elemento
                let (title, description) = console_add().unwrap();
                // Se agrega el nuevo elemento a la lista mutable
                list = add_item(list, title, description);
                // Se muestran los elementos de la lista
                show_items(list.iter());
            }
            "2" => {
                if list.is_empty() {
                    println!("La lista está vacía.");
                } else {
                    show_items(list.iter());
                }
            }
            "3" => {
                // Se solicita al usuario el índice del elemento a eliminar
                let index = console_remove(&list).unwrap();
                // Se elimina el elemento de la lista
                list = remove_item(list, index);
                println!("Elemento eliminado correctamente.");
            }
            "4" => {
                if list.is_empty() {
                    println!("La lista está vacía.");
                } else {
                    // Se solicita al usuario el índice del elemento
                    let index = match console_get_index(&list) {
                        Ok(index) => index,
                        Err(err) => {
                            println!("Error al obtener el índice: {}", err);
                            return Err(err);
                        }
                    };

                    // Se obtiene el cuerpo del elemento
                    let body = match get_item_body(&list, index) {
                        Some(body) => body,
                        None => continue,
                    };

                    // Se muestra el cuerpo del elemento
                    println!("**Cuerpo del elemento {}:**\n{}", index, body);
                }
            }
            "5" => break,
            _ => println!("Opción no válida"),
        }
    }

    Ok(())
}
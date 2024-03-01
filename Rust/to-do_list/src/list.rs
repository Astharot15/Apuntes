use std::io;
use std::io::Write;
use crate::item::Item;

pub fn create_list() -> Vec<Item> {
    Vec::new()
}

pub fn add_item(mut list: Vec<Item>, title: String, description: String) -> Vec<Item> {
    let new_item = Item::new(String::from(title), String::from(description));
    list.push(new_item);
    list
}

pub fn remove_item(mut list: Vec<Item>, index: usize) -> Vec<Item> {
    if index >= list.len() {
        println!("Índice fuera de rango: {}", index);
        return list;
    }
    list.remove(index);
    list
}

pub fn show_items<'a>(items: impl Iterator<Item = &'a Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

pub fn get_item_body(list: &[Item], index: usize) -> Option<String> {
    // Se comprueba si el índice está dentro del rango válido
    if index >= list.len() {
        println!("Índice fuera de rango: {}", index);
        return None;
    }

    // Se retorna el cuerpo del elemento
    Some(list[index].description.clone())
}

pub fn console_add() -> io::Result<(String, String)> {
    print!("Introduzca el título: \n");
    io::stdout().flush()?;

    let mut input1: String = String::new();
    io::stdin().read_line(&mut input1)?;

    print!("Introduzca la descripción: \n");
    io::stdout().flush()?;

    let mut input2: String = String::new();
    io::stdin().read_line(&mut input2)?;

    Ok((input1.trim().to_string(), input2.trim().to_string()))
}

pub fn console_remove(list: &[Item]) -> io::Result<usize> {
    println!("Introduzca el índice del elemento a eliminar (0-{} ):", list.len() - 1);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let index = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("Entrada no válida. Debe ser un número entre 0 y {}.", list.len() - 1);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Entrada no válida"));
        }
    };

    Ok(index)
}

pub fn console_get_index(list: &[Item]) -> io::Result<usize> {
    println!("Introduzca el índice del elemento (0-{} ):", list.len() - 1);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let index = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("Entrada no válida. Debe ser un número entre 0 y {}.", list.len() - 1);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Entrada no válida"));
        }
    };

    Ok(index)
}
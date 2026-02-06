use std::collections::*;

fn main() {

    // Vector
    let mut v: Vec<i32> = Vec::new();
    // Insertar elementos
    v.push(10);
    v.push(20);
    v.push(30);
    // Obtener un valor
    let primero = v[0]; // vector[indice]
    println!("Primero: {}", primero); 
    // Obtener y eliminar el último elemento
    let eliminado = v.pop();
    println!("Eliminado: {:?}", eliminado);
    // :? proporciona un none si no se retorna nada, es decir, si la lista sta vacia.
    // Eliminar por índice
    v.remove(0); 

    println!("Vector: {:?}", v);
    

    // Vector de cola doble
    let mut cola: VecDeque<i32> = VecDeque::new();
    // Insertar al final
    cola.push_back(3);
    cola.push_back(4);
    // Insertar al principio
    cola.push_front(2);
    cola.push_front(1);
    // Obtener el primer valor
    println!("Primero: {:?}", cola.front());
    // Obtener el ultimo valor
    println!("Último: {:?}", cola.back());

    // Obterner y eliminar el primer valor 
    cola.pop_front();
    // Obterner y eliminar el utlimo valor 
    cola.pop_back();

    println!("Vector de cola doble: {:?}", cola);




    // HashMap    
    let mut map: HashMap<String, i32> = HashMap::new();
    // Insertar valores, en este un objeto de tipo String
    map.insert(String::from("A"), 10);
    map.insert(String::from("B"), 20);

    // Obtener un valor
    if let Some(valor) = map.get("C") { // Para obtener el valor de un hashmap es necesario ver si el valor existe, para ello se hace uso del condicional "if"
        println!("C: {}", valor); // En caso de que "C" Exista regresa el valor de C
    } else {
        println!("C no existe"); // Si no existe, entonces se puede definir algun valor por defecto mediante un else
    }

    // Insert tambien permite modificar un valor si su llave ya existe en la coleccion
    map.insert(String::from("A"), 30);

    // Eliminar un valor
    map.remove("B");

    println!("Mapa: {:?}", map);
    // Tambien es posible agreagar elementos en el siguiente formato (lista de tuplas):
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);



    // Arbol binario
    let mut map: BTreeMap<i32, &str> = BTreeMap::new();
    // Agregar valores al arbol
    map.insert(2, "dos");
    map.insert(1, "uno");
    map.insert(3, "tres");
    // Iterar entre cada elemento clave-valor del arbol    
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // Buscar elementos:
    let to_find = [1, 2, 4];
    for k in &to_find {
        match map.get(k) {
        Some(numero) => println!("{k}: {numero}"),
        None => println!("{k} no existe.")
        }
    }
    // Elimintar un par clave-valor:
    map.remove(&2);

}

#[derive(Debug)]

// Ejemplo Struct
// Es necesario definir el contenido del struct antes de la funcion main, el nombre del struct siempre empieza con mayuscula
struct Usuario {
    nombre: String, // Se debe definir el nombre y tipo de variable
    edad: u8,
    activo: bool
}

struct Numeros {
    numero_1: i32,
    numero_2: i32
}

impl Numeros { // Uso de implementaciones con struct Object method, permite aplicar funciones haciendo uso de los valores del struct de fomra directa: 
    fn suma(&self) -> i32 {
        self.numero_1 + self.numero_2 // En este caso para hacer una suma, esto permite a cada estruct Numero aplicar la funcion suma empleando sus propios valores directamente sin necesidad de usar un get
    }
} 


// Ejemplo Enums
/*
Un struct nos permite agrupar datos relacionados.
En este caso, todas las mascotas tienen:
- nombre
- sonido
*/

struct DatosMascota {
    nombre: String,
    sonido: String,
}

fn main() {

    // Crear un Struct 
    let mut struct_1 = Usuario {
        nombre: String::from("David"),
        edad: 26,
        activo: true

    };
    // Tambien es posible completar la informacion de un struct con la de otro struct
    let struct_2 = Usuario {
        nombre: String::from("Rivas"),
        ..struct_1 // Esto quiere decir: "completalo con la informacion del struct 1"
    };

    println!("{:#?}", struct_2); // Usamos un print para ver el contenido completo del struct, :#? nos sirve para darle un formato leible al contenido del print. 
    // Pero usarlo requiere el codigo de la linea 1

    // Uso de implementaciones, metodo Suma del struct Numeros
    let struct_numeros = Numeros {
        numero_1: 2,
        numero_2: 3
    };

    println!("El resultao de la suma de los numeros del struct es: {}", struct_numeros.suma());

    // Para leer o modificar un valor se hace uso de la notacion punto:
    // Obtener un valor
    println!("El nombre usuario del struct 1 es: {}", struct_1.nombre);
    // Modificar un valor 
    struct_1.activo = false;
    println!("El estado del usuario del struct 1 paso a: {}", struct_1.activo);




    // Enums
    /* Cada variante del enum ahora contiene un struct con información de la mascota.
    El enum se define asignandole un nombre a cada posible valor que puede contener, en este caso la mascota puede ser un perro, un gato o una ave. 
    Cada una de ellas se define por el struct DatosMascota
    */
    enum Mascota { 
        Perro(DatosMascota), // 
        Gato(DatosMascota),
        Ave(DatosMascota),
    }

    // Creamos un enum para cada caso:
    let perro = Mascota::Perro(DatosMascota {
        nombre: String::from("Solovino"),
        sonido: String::from("Guau guau"),
    });

    let gato = Mascota::Gato(DatosMascota {
        nombre: String::from("Renguito"),
        sonido: String::from("Miau miau"),
    });

    let ave = Mascota::Ave(DatosMascota {
        nombre: String::from("Piolín"),
        sonido: String::from("Pío pio"),
    });

    // y llamamos una funcion llamada informacion_mascota para hacer uso del contenido dependiendo del tipo de struct
    informacion_mascota(perro);
    informacion_mascota(gato);
    informacion_mascota(ave);


    fn informacion_mascota(mascota: Mascota) {
        // EL match funciona de forma similar al condicional "if". Al coincidir con el tipo de mascota se emplea una accion especifica para cada caso 
        match mascota {
            Mascota::Perro(datos) => {
                println!(
                    "Es un perro llamado {} y hace {} 🐶",
                    datos.nombre, datos.sonido
                );
            }
            Mascota::Gato(datos) => {
                println!(
                    "Es un gato llamado {} y hace {} 🐱",
                    datos.nombre, datos.sonido
                );
            }
            Mascota::Ave(datos) => {
                println!(
                    "Es un ave llamada {} y hace {} 🐦",
                    datos.nombre, datos.sonido
                );
            }
        }
    }

}

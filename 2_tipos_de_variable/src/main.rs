fn main() {
    
    // Definicion de variables
    // Existen 3 tipos de variables:

    // Variable no mutable
    let number = 1;
    // Variable que una vez definida no pude cambiar su valor. 

    // variable mutable 
    let mut num = 2; 
    // Variable que puede cambiar varias veces su valor y tipo de variable (shadowing)

    // Constante
    const NUMERO:u32 = 12 * 12; //  que solo puede almacenar una expresion constante, es decir, que se evalue al correr el codigo.
    
    // ⚠️ IMPORTANTE ⚠️
    /* La principal diferencia entre el let y const es que let permite el uso de funciones para definir el valor de la variable, const no.
    Ejemplo:
    const nombre = String::from("Bob esponja"); // ❌
    const nombre: &str = "Bob esponja"; // ✅
     */



    // Tipos de variables escalares:
    // Numeros enteros
    // i = numero con signo | u = numero sin singno

    // 8 bits
    let a: i8 = -128; // -128 a 127
    let b: u8 = 255; // 0 a 255

    println!("i8: {}", a);
    println!("u8: {}", b);

    // 16 bits
    let a: i16 = -32_000; // -32,768 a 32,767
    let b: u16 = 65_000; // 0 a 65,535

    println!("i16: {}", a);
    println!("u16: {}", b);

    // 32 bits (por defecto)
    let a: i32 = -2_000_000; // -2,147,483,648 a 2,147,483,647
    let b: u32 = 4_000_000; // 0 a 4,294,967,295 

    println!("i32: {}", a);
    println!("u32: {}", b);

    // 64 bits 
    let a: i64 = -9_000_000_000; // -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
    let b: u64 = 18_000_000_000; // 0 a 18,446,744,073,709,551,616

    println!("i64: {}", a);
    println!("u64: {}", b);

    // 128 bits
    let a: i128 = -170_000_000_000_000_000;
    let b: u128 = 340_000_000_000_000_000;

    println!("i128: {}", a); // -1.7014118346046923173168730371588e+38 a 1.7014118346046923173168730371588e+38 -1 
    println!("u128: {}", b); // 0 a 3.4028236692093846346337460743177e+38

    // Numeros dependientes de la arquitectura del procesador (32 o 64 bits)
    let a: isize = -10;
    let b: usize = 10;

    println!("isize: {}", a);
    println!("usize: {}", b);



    // Tambien es posible emplear diferentes sistemas numericos:
    // Binario (base 2)
    let binary = 0b1111_0000;
    println!("Binario: {}", binary);

    // Octal (base 8)
    let octal = 0o77;
    println!("Octal: {}", octal);

    // Decimal (base 10)
    let decimal = 98_222;
    println!("Decimal: {}", decimal);

    // Hexadecimal (base 16)
    let hex = 0xff;
    println!("Hexadecimal: {}", hex);



    // Numeros flotantes 
    let flotante_32: f32 = 1.5;
    let flotante_64: f64 = 2.5;
    // Todos los numeros flotantes son con signo



    // Booleanos, valores que permiten gestionar el control y flujo de nuestro codigo mediante su uso comparaciones y condicionales
    let verdadero: bool = true;
    let falso = false;



    // Texto 
    // Char, caracter de 4 bits. Solo puede almacenar una letra, emojis son elementos validos.
    let a: char = 'a';
    let b: char = 'ñ';
    let mut  emoji: char = '😄';
    emoji = '✨';

    // &str, string slice. Es inmutable debido a que es una referencia a un string, es almacenado en el binario del programa (no se crea en ejecucion)
    let hw: &str = "Hola mundo";
    println!("{}", hw);

    // String, cadena de texto que puede ser mutable o inmutable 
    let mut s = String::from("Hola");
    s.push_str(" mundo");
    println!("{}", s);



    // Variables compuestas
    // Tuplas, agrupacion de dos o varios elementos que pueden ser del mismo o diferente tipo. No se puede cambiar su tamaño (agregar o quitar elementos)
    let tupla = (1, "uno"); // tupla de dos valores, entero y string
    let tupla: (u8, &str) = (1, "uno"); // tambien es valido definir los tipos de los elementos que integran la tupla
    let (numero, texto) = tupla; // deconstruccion, forma de acceder al valor de una tupla mediante la descomposicion de la tupla en variables
    let numero = tupla.1; // notacion de punto, otra forma de leer o cambiar el valor de una tupla.

    let mut tupla = (1, 2, 3); // Definicion de una nueva tupla con numeros enteros
    tupla.0 = 0; // cambio de valor del primer elemento de 1 a 0. 



    // arrays, agrupacion de dos o mas elementos de mismo tipo, son de longitud fija
    let arr = [0,1,2,3,4]; // tupla de 5 numeros enteros 
    let arr: [u8; 5] = [0,1,2,3,4]; // tambien es valido definir sus tipos y el tamaño del array
    let arr = [0;4]; // Se pueden crear arrays con un solo elemento al especificar un tipo (en este caso el 0, entero sin signo) y el tamaño del array
    let valor_arr = arr[1]; // Para acceder al valor de un elemento basta con conocer su indice. Los arrays tambien pueden ser mutables.


}

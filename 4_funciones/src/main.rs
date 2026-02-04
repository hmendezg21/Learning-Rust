/* Main es nuestra funcion principal, la que se ejecuta siempre al correr el codigo. 
Cada funcion se encuentra delimitada por corchetes "{}", los cuales ayudan a indicar qu parte del codigo le pertenece a cada funcion, condicional o ciclo,
a esto se le conoce como scope (alcance).
*/

fn main() {
    println!("Hello, world!");
    funcion_de_prueba(); // Llamado a "funcion_de_prueba"
    println!("{}", funcion_suma(1,2)); // llamado a "funcion_suma"
    println!("{}", funcion_suma_p(2,3)); // llamado a "funcion_suma_p"

    println!("La suma de 10 y 50 da: {}", calculadora(1,10,50)); 
    println!("La resta de 10 y 50 da: {}", calculadora(2,10,50)); 
    println!("La multiplicacion de 10 y 50 da: {}", calculadora(3,10,50)); 
    println!("La division de 10 y 50 da: {}", calculadora(4,10,50)); 
    println!("El modulo de 10 y 50 da: {}", calculadora(5,10,50)); 
    println!("Usar un valor de operacion diferente da: {}", calculadora(6,10,50)); 
}

/*  Es posible crear mas que la funcion main dentro de un .rs de Rust. Por convencion, cada funcion y variable es escrita en el estilo "snake case" 
(todo en minusculas y separado por guiones bajos).

Declarar una funcion es sencillo, solo es necesario escribir "fn" acompañado del nombre de la funcion, abrir un parentesis "()" que corresponen a los 
parametros de la funcion, y por ultimo abrir unos corchetes "{}", que es donde estara contenido el codigo de la funcion, en este caso, un print simple:
*/

fn funcion_de_prueba() { // Importante llamar la funcion en main
    println!("La funcion de prueba es accesible y funcional!");
}
/* 
Las funciones pueden incluir parametros, es decir, pedir valores con los cuales trabajar e interactuar. Esto es de bastante utilidad cuando 
deseas separar en funciones partes de codigo que se reptien constantemente. De esta manera en lugar de repetir el mismo codigo en diferentes 
partes del mismo, lo separas en una funcion que llamas cada que la necesites. 

Cada parametro de la funcion debe estar definida por un nombre y tipo de variable correspondiente por ejemplo:
- (x: u8) para un entero de 8bits 
- (nombre: String) para un string

Es posible usar n cantidad de parametros de diferentes tipos. De igual manera, las funciones tambien pueden retornar valores:
*/
fn funcion_suma(x:u8, y:u8) -> u8 {
    return x + y;
}

/* Para retornar un valor es necesario especificar el tipo de dato retornado mediante "->" despues de los parentesis, aunque 
tambien es posible hacerlo de la siguiente manera:
*/
fn funcion_suma_p(x:u8, y:u8) -> u8 {
    x + y
}

/* Tomando en cuenta lo visto anteriormente crearemos una funcion que realice diferentes operaciones:
1. suma 
2. resta 
3. multiplicacion 
4. division 
5. operacion modulo
*/
fn calculadora(operacion: u8, x:i32, y:i32) -> i32 {
    if (operacion < 1) | (operacion > 5) { // Condicion incial, se verifica si el numero introducido es valido antes de proseguir
        println!("Operacion no valida, intente de nuevo con un numero de operacion valido");
        return 0;
    } else if operacion == 1 { // Si operacion es igual a 1 se hace la suma y se regresa el valor obtenido
        return x + y;
    } else if operacion == 2 { // Si operacion es igual a 1 se hace la resta y se regresa el valor obtenido
        return x - y;
    } else if operacion == 3 { // Si operacion es igual a 1 se hace la multiplicacion y se regresa el valor obtenido
        return x * y;
    } else if operacion == 4 { // Si operacion es igual a 1 se hace la division y se regresa el valor obtenido
        return x / y;
    } else {    // Si ninguna de las condiciones anteriores se cumple, se hace la operacion modulo y se regresa el valor obtenido
        return x % y;
    }
}
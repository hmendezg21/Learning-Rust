/*
El sistema de ownership sirve para la memoria del programa, 
Para comenzar es importante conocer las 3 reglas del ownership (propiedad):
- Cada valor tiene un owner (dueño)
- Cada valor solo puede tener un owner
- Cuando el owner sale del scope (alcance), el valor será liberado (eliminado).

NOTA: El "scope" se define como el rango dentro del programa donde donde el elemento es valido y usable.
El rango es normalmente definido por "{ }"
*/


fn main() {
    // Ejemplo de scope:    
    {
        let texto = String::from("Hola Scope!!!");
        println!("{}", texto); // Es posible usar la variable texto aqui porque esta dentro de "{ }"
    } // Aquí se libera automáticamente
    /*  println!("{}", texto);  hacer uso de la variable fuera de las llaves seria un error 
    porque no existe
    */



    // Ownership con tipo de datos primitivos
    /* En los tipos primitivos como enteros, booleanos y flotantes implementan el trait Copy.
    Esto significa que al asignarlos o pasarlos a una función, su valor se COPIA, en lugar 
    de ceder el ownership.
    */
    let x = 10; 
    let y = x; // Se copia el valor de x en lugar de dar el ownership

    println!("x: {}, y: {}", x, y); // Ambos siguen siendo válidos

    

    // Ownership con tipo de datos heap
    // Al asignarlo que el caso anterior, el ownership se mueve de variable.
    let var_1 = String::from("Ownership con heap");
    let var_2 = var_1; // El ownership pasa de var_1 a var_2.

    // println!("{}", var_1); // Llamar a la variable var_1 daira error porque no existe
    println!("{}", var_2); // En su lugar, var_2 es el nuevo dueño


// Ownership en funciones
    /*
    Al pasar un valor a una función, algo similar. Si el tipo de variable es primitivo su valor se 
    copia, si es de tipo heap, entonces se cede el ownership al parametro de la funcion:
    - Tipos Copy → se copian
    - Tipos heap → se cede el owner
     */
    // Primitivos:
    let numero = 5;
    imprimir_numero(numero); // Copia
    println!("{}", numero);  // Sigue siendo válido

    fn imprimir_numero(n: i32) {
        println!("Número: {}", n);
    }
    // Heap:
    let palabra = String::from("Funciones con heap");
    imprimir_string(palabra); // Se mueve a "s"
    
    fn imprimir_string(s: String) {
        println!("Se imprimió: {}", s);
    } // s se libera aquí
    // hacer esto deja a la variable "palabra" sea limpiada, para evitarlo es necesario devolver 
    // el valor de la variable antes de finalizar la funcion imprimir_string:
    fn imprimir_y_devolver_string(s: String) -> String {
        println!("Se imprimió y devolvió: {}", s);
        s
    }



    // Referencias (Borrowing)
    /* Las referencias (&) se usan para evitar el ceder la propiedad, en su lugar solo se comparte
    la referencia del valor que contiene la variable referenciada.
     */
    let mensaje = String::from("Hola referencia!!!");
    imprimir_referencia(&mensaje); // El ownership se conserva
    println!("{}", mensaje);       // Usar la variable mnsaje sigue siendo válido
    
    fn imprimir_referencia(s: &String) {
        println!("Referencia: {}", s);
    }

    /*Existen dos tipos de referencias, las mutables y las no mutables (como es el caso anterior)
    Para diferenciar una de otra se debe compañar con el "&" con "mut". Lo que permite no solo leer 
    el valor, tambien modificarlo:
     */
    let mut hola_mundo = String::from("Hola");
    modificar_string(&mut hola_mundo);
    println!("{}", hola_mundo);

    fn modificar_string(s: &mut String) {
        s.push_str(" mundo"); 
    } // Notese que no es necesario regresar la propiedad

    // ES IMPORTANTE TENER EN CUENTA QUE:
    // No se pueden tener referencias mutables dobles 
    let mut x = String::from("Test");
    let  y = &mut x;
    // let  z = &mut y; ❌ 

    // pero si multiples refrencias no mutables
    let x = String::from("Test");
    let y = &x;
    let z = &y;
}
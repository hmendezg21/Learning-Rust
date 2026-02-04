fn main() {

    // Control de flujo 
    // Primero definimos la variable "numero" que almacene un numero entero.    
    let numero = 1;
    /*  el condicional "if" nos permite controlar el flujo del codigo mediante 
    la comparacion de dos valores, dicha comparacion puede ser:
    > -> mayor que 
    < -> menor que 
    >= -> mayor o igual que 
    <= -> menor o igual que 
    == -> igual que 
    != -> diferente que 

    y tambien pueden incorporar operaciones booleanas para que el flujo dependa de mas comparaciones o estados:
    & (and) -> permite establecer que es necesario que dos o mas condiciones se cumplan (condicion_1 y condicion_2)
    | (or) -> permite establecer que es necesario que solo una condicion se cumpla (condicion_1 o condicion_2)
    ! (not) -> invierte el valor de la variable booleana, es decir si es true pasa a false, y si es false pasa a true
    Ejemplos: 
    if simple, se compara si "numero" es igual a 1
    */
    if numero == 1 {
        println!("Numero es igual a 1!");
    } 

    // el operador & sirve para asegurar que ambas condiciones se cumplan. En este caso que numero sea 
    // menor mayor a 0 y menor que dos 
    if (numero > 0) & (numero < 2) {
        println!("Numero es igual a 1!");
    }
    // el operador | es mas flexible, ya que especifica que solo es necesario que una condicion se cumpla
    // en este caso, que numero sea diferente de 0 o sea igual a 1.
    if (numero !=0) | (numero==10) {
        println!("Numero es diferente de 0 o igual a 10!");
    }
    // Por ultimo, el operador ! nos permite negar un estado, es decir, alternar su valor booleano.
    // como se desea verificar que el numero es igual a 1, pero usamos != al ser iguales dara como resultado 
    // false, lo que en lugar de ejecutar el codigo dentro del if, lo omitira. Para solucionarlo, se niega
    // el resultado de la comparacion mediante el operador not
    if !(numero != 1) {
        println!("Numero es igual a 1!")
    }

    // Es posible almacenar el resultado de una comparacion y de operaciones booleanas en una variable:
    let estado = numero == 1;
    
    /*  Como complemento del if, se tienen otras expresiones como el "else" y el "else if":
    
    else if -> permite establecer una condicion alernativa en dado caso de que la primera no se cumpla
    esto difiere del uso de las operaciones booleneas en el sentido de que solo se analiza si la primera
    no es verdadera, seria como decir:
    "Si esta condicion se cumple haz esto, si no, analiza esta segunda condicion, y si se cumple entonces haz esto"

    else -> funciona como accion de respaldo que se efectua solo si ninguna condicion anteriormente establecida
    se cumple.
    ejemplo: */
    if numero < 1 {
        println!("Numero es menor a 1")
    } else if numero > 1 {
        println!("Numero es mayor a 1")
    } else {
        println!("Numero es igual a 1")
    }

    // tambien es posible comprimir el if a una sola linea (sin el uso de "else if"):
    let comparacion = numero < 10;
    let estado = if comparacion {"Menor"} else {"Mayor o igual"};
    println!("{}", &estado);

    // Ciclos: Codigo que se ejecuta siempre y cuando se cumpla una condicion
    //loop: se ejecuta infinitamente hasta que se establece una condicion de paro mediante "break"
    let mut contador_loop = 0; 
    let ciclo_loop = loop {
        contador_loop += 1;
        if contador_loop == 10 {
            break contador_loop;
        } 
    };
    // es posible emplear loop para asignar un valor a una variable
    let mut contador_loop = 0; 
    let ciclo_loop = loop {
        contador_loop += 1;
        if contador_loop == 10 {
            break contador_loop;
        } 
    };
    println!("Cuenta: {}", ciclo_loop);

    //while: se ejecuta solo si la condicion establecida se cumple 
    let mut contador_while = 5;
    while contador_while !=0 {
        println!("{}", contador_while);
        contador_while -= 1;
    }
    println!("While terminado!!!");

    //for: se ejecuta depndiendo del tamaño de una coleccion o lista de elementos con los cuales puede interactuar 
    let a = [1;3]; // se crea un array de 3 elementos cuyo valor es 1
    for n in a.iter() {
        println!("{}", n); // imprime cada elemento del array
    }

    for n in 0..10 { // se crea una secuencia de numeros de 0 a 9.
        println!("{}", n); // imprime cada numero de la secuencia.
    }
}

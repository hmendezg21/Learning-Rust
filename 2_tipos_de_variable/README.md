# Tipos de Datos y Variables en Rust
En `Rust`, comprender cómo funcionan las variables y los tipos de datos es fundamental para escribir código correcto, seguro y eficiente. A diferencia de otros lenguajes, `Rust` es fuertemente tipado y muchas decisiones relacionadas con la memoria y los valores se verifican en tiempo de compilación. Este archivo describe de forma teórica los conceptos que se utilizan en el código de ejemplo de este módulo.

## Variables y constantes
En `Rust` existen tres formas principales de trabajar con valores: `variables inmutables`, `variables mutables` y `constantes`.

Por defecto, las variables en `Rust` son inmutables. Esto significa que, una vez asignado un valor, no puede cambiarse. Esta decisión de diseño ayuda a evitar errores y hace que el código sea más predecible.

Cuando se necesita que una variable cambie su valor, se debe declarar explícitamente como mutable usando la palabra clave `mut`. La mutabilidad es una propiedad de la variable, no del tipo de dato.

Las constantes, declaradas con `const`, representan valores que no cambian durante toda la ejecución del programa. A diferencia de las variables, las constantes:

* Deben tener siempre un tipo explícito.

* Solo pueden definirse usando expresiones constantes evaluables en tiempo de compilación.

* No pueden depender de llamadas a funciones.

Esto hace que las constantes sean ideales para valores fijos y conocidos de antemano.

--- 
### Tipos de datos escalares
Los tipos escalares representan un solo valor. `Rust` incluye cuatro categorías principales: `enteros`, `flotantes`, `booleanos` y `caracteres`.

#### Números enteros

`Rust` permite el uso de diferentes tipos de numeros enteros:

* valores negativos (signed `i`)

* valores positivos (unsigned `u`)

* El tamaño en bits (`8`, `16`, `32`, `64`, `128`)

Cada tipo tiene un rango bien definido. Por ejemplo, `i8` puede almacenar valores entre `-128` y `127`, mientras que `u8` va de `0` a `255`. El tipo entero por defecto es `i32`, ya que suele ofrecer un buen equilibrio entre rendimiento y rango.

Además, `Rust` proporciona los tipos `isize` y `usize`, cuyo tamaño depende de la arquitectura del procesador (`32` o `64` bits). Estos tipos se utilizan principalmente para índices, tamaños de colecciones y operaciones relacionadas con memoria.

--- 
#### Sistemas numéricos

Los números enteros pueden expresarse en distintos sistemas numéricos:

* `Decimal` (base 10)

* `Binario` (base 2)

* `Octal` (base 8)

* `Hexadecimal` (base 16)

El uso de estos formatos no cambia el valor del número, pero puede mejorar la legibilidad o ser útil en programación de bajo nivel.

---
#### Números flotantes

`Rust` soporta dos tipos de números de punto flotante:

* `f32` (32 bits)

* `f64` (64 bits)

Por defecto, `Rust` utiliza `f64`, ya que ofrece mayor precisión. Todos los números flotantes son con signo.

---
#### Booleanos

El tipo booleano `bool` solo puede tomar dos valores: `true` o `false`.
Los booleanos son fundamentales para controlar el flujo del programa mediante condicionales y comparaciones.

--- 
#### Caracteres (char)

El tipo `char` representa un solo carácter Unicode y ocupa 4 bytes. Esto significa que puede almacenar no solo letras ASCII, sino también `caracteres acentuados`, `símbolos` y `emojis`. A diferencia de otros lenguajes, un `char` en `Rust` no equivale a un byte.

---
#### Texto: &str y String

`Rust` distingue claramente entre texto inmutable y texto dinámico.

Un `&str` (string slice) es una referencia a una secuencia de texto. Generalmente proviene de un literal de cadena y se almacena directamente en el binario del programa. Es inmutable y no posee la memoria que apunta.

Un `String`, en cambio, es una estructura que posee su propia memoria, la cual se almacena en el heap. Puede ser mutable y crecer dinámicamente durante la ejecución del programa. El uso de `String` está estrechamente ligado al modelo de ownership de `Rust`.

---
### Tipos de datos compuestos

Los tipos compuestos permiten agrupar múltiples valores en una sola estructura.

#### Tuplas

Las tuplas agrupan una cantidad fija de valores, que pueden ser de distintos tipos. Una vez definida, una `tupla` no puede cambiar su tamaño. Los valores de una `tupla` pueden accederse mediante `desestructuración` o usando la `notación punto` (.0, .1, etc.). Si la `tupla` es mutable, sus elementos también pueden modificarse.

---
#### Arrays

Los `arrays` son colecciones de elementos del mismo tipo y de longitud fija. A diferencia de otros lenguajes, `Rust` verifica en tiempo de compilación que los accesos a los `arrays` sean seguros. Los `arrays` son útiles cuando se conoce de antemano el número exacto de elementos y se desea un almacenamiento contiguo en memoria.


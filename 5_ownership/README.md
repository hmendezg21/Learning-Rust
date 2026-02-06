# Ownership en Rust: Stack, Heap y Gestión de Memoria

Uno de los aspectos que hace a Rust diferente de otros lenguajes es su forma de manejar la memoria.  
Para entender el **sistema de ownership**, primero es indispensable comprender **qué son el stack y el heap**, ya que Rust toma decisiones distintas dependiendo de dónde se almacenen los datos.

---

## Stack y Heap: los dos lugares donde vive la memoria

Cuando un programa se ejecuta, utiliza principalmente dos áreas de memoria: **stack** y **heap**.  
Ambas sirven para almacenar datos, pero lo hacen de maneras muy distintas.

---

## Stack (pila)

El **stack** es una región de memoria rápida y ordenada. Funciona bajo el principio **LIFO** (*Last In, First Out*), es decir, el último valor en entrar es el primero en salir, lo que trae consigo varias ventajas:

- Velocidad, es memoria de rápido acceso
- Tamaño fijo y conocido en tiempo de compilación
- La memoria se libera automáticamente al salir del scope
- Es ideal para datos simples y pequeños

En Rust, los **tipos primitivos** suelen almacenarse en el stack, por ejemplo:
- Enteros (`i32`, `u8`, etc.)
- Booleanos (`bool`)
- Flotantes (`f32`, `f64`)
- `char`
- Tuplas y estructuras pequeñas con tamaño fijo

Cuando una variable del stack sale de su **scope**, Rust elimina automáticamente su valor.

---

## Heap 

El **heap** es una región de memoria más flexible, pero también más costosa de usar.  
Aquí se almacenan datos cuyo tamaño **no se conoce en tiempo de compilación** o puede cambiar durante la ejecución. En comparación con el **stack**, el **heap** difiere en:

- Más lento que el stack
- Permite tamaños dinámicos
- Requiere gestión explícita
- Ideal para datos complejos o grandes

tipos como `String`, `Vec<T>` y otras estructuras dinámicas usan el heap.

Y funciona de la siguiente manera:

1. Se reserva espacio dinámicamente
2. Se obtiene una referencia (puntero) a esa memoria
3. Esa referencia se guarda en el stack

---

## ¿Qué es el sistema de ownership?

El sistema de ownership es el mecanismo que se usa para **gestionar automáticamente la memoria**, sin garbage collector y sin requerir liberación manual.

Rust aplica estas reglas en tiempo de compilación:

1. Cada valor tiene un único owner (dueño).
2. Un valor solo puede tener un owner a la vez.
3. Cuando el owner sale del scope, el valor se libera automáticamente.

---

## Scope (alcance)

El **scope** es el rango del programa donde una variable es válida y puede ser utilizada.  
Generalmente está delimitado por llaves `{}`.

Cuando una variable sale de su scope:
- Si estaba en el stack, se elimina inmediatamente
- Si era dueña de memoria en el heap, Rust libera esa memoria automáticamente

## **Continuación: ¿Qué sigue?**
[Ver ejemplos en código](/5_ownership/src/main.rs)

[Da clic aquí para ir a la siguiente sección](/6_colecciones/README.md)
# Colecciones

Una **colección** es una estructura de datos que permite **almacenar múltiples valores**, generalmente del mismo tipo, de forma **dinámica**. A diferencia de los arrays o las tuplas, las colecciones:

- Se almacenan en el **heap**
- Pueden **crecer o reducirse en tiempo de ejecución**
- Ofrecen mayor flexibilidad para manejar datos
- Están integradas con el sistema de **ownership y borrowing**

Las colecciones se proporcionan a través del módulo `std::collections`, cada una diseñada para resolver necesidades específicas de almacenamiento y acceso a datos. Mas información [Aquí](https://doc.rust-lang.org/std/collections/index.html)

---

## ¿Cuándo usar `Vec`?

Usa un **`Vec` (vector)** cuando:

- Quieres recolectar elementos para procesarlos o enviarlos a otro lugar más adelante, y **no te importa ninguna propiedad especial de los valores almacenados**.
- Necesitas una **secuencia de elementos en un orden específico**, y solo planeas agregar elementos **al final (o cerca del final)**.
- Quieres implementar una **pila (stack)**.
- Necesitas un **array redimensionable**.
- Quieres un **array almacenado en el heap**.

`Vec` es la colección más común y versátil en Rust, y suele ser la primera opción cuando no existe un requerimiento más específico.

---

## ¿Cuándo usar `VecDeque`?

Usa un **`VecDeque`** cuando:

- Necesitas un `Vec` que permita **inserciones y eliminaciones eficientes en ambos extremos** de la secuencia.
- Quieres implementar una **cola (queue)**.
- Necesitas una **cola de doble extremo (double-ended queue o deque)**.

`VecDeque` es ideal para escenarios donde el acceso frecuente ocurre tanto al inicio como al final de la colección.

---

## ¿Cuándo usar `HashMap`?

Usa un **`HashMap`** cuando:

- Necesitas **asociar claves arbitrarias con valores arbitrarios**.
- Quieres implementar un **sistema de caché**.
- Necesitas un **mapa simple**, sin funcionalidades adicionales como ordenamiento.

`HashMap` ofrece búsquedas, inserciones y eliminaciones rápidas, pero **no garantiza ningún orden** en los elementos.

---

## ¿Cuándo usar `BTreeMap`?

Usa un **`BTreeMap`** cuando:

- Necesitas un **mapa ordenado por sus claves**.
- Quieres obtener **rangos de entradas bajo demanda**.
- Te interesa conocer **el par clave–valor más pequeño o más grande**.
- Necesitas encontrar **la clave más grande o más pequeña** que sea menor o mayor que un valor dado.

`BTreeMap` es más lento que `HashMap`, pero ofrece **orden y operaciones por rango**, lo cual es fundamental en ciertos casos.

## **Continuación: ¿Qué sigue?**
[Ver ejemplos en código](/6_colecciones/src/main.rs)

[Da clic aquí para ir a la siguiente sección](/7_structs_y_enums/README.md)

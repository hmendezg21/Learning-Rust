# Structs y Enums

## ¿Qué es un `struct`?

Un `struct` es una estructura de datos que permite **agrupar múltiples valores relacionados bajo un mismo tipo**. Cada valor dentro del struct tiene un **nombre** y un **tipo**, lo que hace que el código sea más legible y fácil de mantener.

Los `structs` son ideales cuando:

- Los datos **siempre existen juntos**
- Todos los campos representan **propiedades de una misma entidad**
- La forma de los datos **no cambia**

Ejemplos comunes de uso de `struct` incluyen:
- Usuarios
- Configuraciones
- Objetos matemáticos
- Registros de base de datos

En el código, el `struct Usuario` representa una entidad con nombre, edad y estado, mientras que `Numeros` agrupa dos valores numéricos relacionados.

---

## Implementaciones (`impl`) y métodos

Rust permite asociar funciones directamente a un `struct` mediante bloques `impl`. Esto habilita el uso de **métodos**, que funcionan de forma similar a los métodos de objetos en otros lenguajes.

Gracias a esto:
- El comportamiento queda ligado a los datos
- Se evita pasar múltiples parámetros innecesarios
- El código es más expresivo y reutilizable

Este enfoque es conocido como **programación orientada a datos**

---

## ¿Qué es un `enum`?

Un `enum` (enumeración) permite definir un tipo que puede tomar **uno de varios valores posibles**, llamados *variantes*. A diferencia de otros lenguajes, los enums en Rust pueden **contener datos**, incluso structs completos.

Un `enum` es ideal cuando:

- Un valor puede estar en **diferentes estados**
- Existen **múltiples variantes excluyentes**
- Cada variante puede requerir **información distinta**

En el ejemplo del código, el `enum Mascota` define que una mascota puede ser un perro, un gato o un ave, y cada una contiene un `struct` con su información.

---

## ¿Cuándo usar `struct` y cuándo `enum`?

Usa un **struct** cuando:
- Estás modelando un objeto con propiedades claras
- La estructura de los datos no cambia
- Quieres asociar métodos directamente a los datos

Usa un **enum** cuando:
- Algo puede existir en múltiples formas
- Quieres representar estados, opciones o resultados
- Necesitas seguridad al manejar diferentes casos


en pocas palabras:
- Usa **struct** cuando todos los datos existen siempre
- Usa **enum** cuando algo puede ser *una cosa u otra*


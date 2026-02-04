# Control de Flujo

El control de flujo permite controlar **qué partes del código se ejecutan y en qué orden**, dependiendo de condiciones, comparaciones o repeticiones. 


---

## Condicionales (`if`, `else if`, `else`)

La expresión `if` permite ejecutar un bloque de código **solo si una condición es verdadera** (`true`). La condición debe ser **estrictamente booleana**, no se permiten valores implícitos como `0` o `1`.

### Operadores de comparación

Las condiciones suelen construirse usando operadores de comparación:

- `>`  → mayor que  
- `<`  → menor que  
- `>=` → mayor o igual que  
- `<=` → menor o igual que  
- `==` → igual que  
- `!=` → diferente que  

Estas comparaciones siempre producen un valor booleano (`true` o `false`).

---

## Operadores lógicos (booleanos)

Rust permite combinar múltiples condiciones usando operadores lógicos:

- `&` (**AND**)  
  Todas las condiciones deben cumplirse para que el resultado sea `true`.

- `|` (**OR**)  
  Basta con que una condición se cumpla para que el resultado sea `true`.

- `!` (**NOT**)  
  Niega el valor booleano, convirtiendo `true` en `false` y viceversa.


---

## Uso de `else if` y `else`

Además del `if`, se complementa con:

- `else if`: define una **condición alternativa** que solo se evalúa si la condición anterior fue falsa.
- `else`: actúa como un **caso de respaldo**, ejecutándose únicamente si ninguna condición previa se cumple.


---

## `if` como expresión

Una característica importante es que `if` es una **expresión**, no solo una instrucción.  
Esto significa que puede devolver un valor y asignarse a una variable.

Para que esto sea válido:
- Todos los bloques deben devolver el **mismo tipo de dato**.
- No se usa punto y coma (`;`) al final del valor retornado.


---

## Ciclos 
Los ciclos permiten ejecutar código de forma repetitiva siempre dependiendo de una condición establecida implícita o explícitamente

---

## `loop`

El ciclo `loop` se ejecuta de forma **infinita** hasta que se indica explícitamente que debe detenerse usando `break`.

Una característica especial de `loop` es que puede **retornar un valor** cuando se interrumpe, lo que permite asignar ese valor a una variable. Esto es útil cuando se necesita repetir una operación hasta cumplir cierta condición y obtener un resultado final.

---

## `while`

El ciclo `while` se ejecuta **mientras una condición sea verdadera**.  
Antes de cada iteración, Rust evalúa la condición; si esta es falsa, el ciclo termina.

Este tipo de ciclo es útil cuando no se conoce de antemano cuántas veces se repetirá el bloque de código, pero sí la condición de salida.

---

## Ciclo `for`

Se emplea para iterar sobre **colecciones**, **rangos** o **iteradores**, evitando errores comunes como desbordamientos de índices.

Rust maneja internamente el control del ciclo, lo que lo convierte en la opción recomendada siempre que sea posible.

Ejemplos comunes de uso incluyen:
- Recorrer arrays
- Iterar sobre rangos numéricos
- Procesar elementos de una colección



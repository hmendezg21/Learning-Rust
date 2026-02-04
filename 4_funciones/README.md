# Funciones en Rust

Las funciones son uno de los bloques fundamentales de cualquier programa en Rust. Permiten **organizar el código**, **reutilizar lógica** y **mejorar la legibilidad**, separando responsabilidades en unidades pequeñas y claras.

Todo programa en Rust comienza su ejecución desde una función especial llamada `main`.

---

## La función `main`

La función `main` es el **punto de entrada** del programa.  
Cada vez que se ejecuta el binario, Rust comienza a ejecutar el código contenido dentro de esta función.

Las funciones en Rust se definen usando la palabra clave `fn` y su cuerpo se encuentra delimitado por llaves `{}`.  
Estas llaves definen el **scope (alcance)**, es decir, qué variables y expresiones pertenecen a una función, condicional o ciclo determinado.

El scope es importante porque determina:
- Cuándo una variable es accesible
- Cuándo se libera la memoria asociada a ella

---

## Definición y llamado de funciones

Rust permite definir múltiples funciones dentro de un mismo archivo `.rs`.  
Por convención, los nombres de funciones y variables se escriben en **snake_case**, es decir, todo en minúsculas y separadas por guiones bajos.

Para ejecutar una función, esta debe ser **llamada explícitamente**, generalmente desde `main` u otra función.

Las funciones pueden:
- No recibir parámetros
- Recibir uno o más parámetros
- Retornar o no un valor

---

## Funciones sin parámetros ni retorno

Una función puede simplemente ejecutar código sin recibir valores externos ni devolver resultados.  
Este tipo de funciones se utiliza comúnmente para tareas como mostrar información, inicializar procesos o ejecutar acciones específicas.

---

## Funciones con parámetros

Las funciones pueden recibir **parámetros**, que son valores que se pasan a la función para que trabaje con ellos.  
Cada parámetro debe tener:
- Un nombre
- Un tipo explícito

Esto refuerza el sistema de tipos de Rust y evita errores en tiempo de compilación.

Los parámetros permiten:
- Reutilizar lógica
- Evitar duplicación de código
- Hacer funciones más flexibles

---

## Retorno de valores

Las funciones en Rust pueden retornar valores. Para ello:
- Se debe indicar el tipo de retorno usando `->`
- El valor retornado debe coincidir exactamente con ese tipo

Existen dos formas de retornar valores:

1. Usando la palabra clave `return`
2. Mediante una **expresión final sin punto y coma**

Rust favorece la segunda forma, ya que refuerza su enfoque expresivo y funcional.



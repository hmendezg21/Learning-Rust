# Conociendo la terminal
Eres nuevo programando o usando sistemas operativos base linux?, estas en el lugar adecuado para aprender :D

La terminal, también conocida como línea de comandos, es una herramienta que permite interactuar directamente con el sistema operativo mediante instrucciones escritas para navegar por archivos y carpetas, crear, mover o eliminar archivos, instalar programas, ejecutar comandos, compilar código y automatizar tareas.

Un primer vistazo a la terminal del entorno en codespaces es el siguiente:
```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ 
```
Donde vemos como primer elemento nuestro nombre usuario de Github acompañado de `/workspaces/Learning-Rust (main) $`. Pero, ¿qué quiere decir esto?, vamos a desglosarlo por partes:

* `/workspaces/Learning-Rust`: nos muestra nuestra ubicación de la términal dentro de nuestro proyecto. En este caso nos encontramos en la carpeta raíz de nuestro proyecto en `/workspaces` (nombre que asigna Github a los entornos creados en codespaces) y `/Learning-Rust`, el nombre de nuestro repositorio.

* `(main)`: indica la ramificación del repositorio. El término ramificación proviene de la analogía de como a partir de un tronco principal (de aqui viene el concepto `main`) se dispersan y crecen nuevas ramas (`branch`). En este caso nuestro código puede contar con una versión principal o `main` y versiones secundarias (`branches`), lo que nos permite experimentar y desarrollar nuestro código sin necesidad de modificar o sacrificar la estabilidad de nuestro codigo original.

* `$`: Forma parte de una convención para separar la información de la terminal con respecto a los comandos escritos. Además, ayuda a diferenciar el usuario tiene privilegios de administrador.

## Comandos básicos:
```bash
ls 
```
Proporciona una lista de los elementos (carpetas y archvios) que se encuentran en el directorio actual:

```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls
images  Introducción  README.md
```
---

```bash
cd nombre_de_la_carpeta
```
`cd` priviene de `change directory` (cambiar directorio), comando usado para moverse entre carpetas de la siguiente manera:

```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ cd Introducción/ # comando introducido 
@DvdRivas ➜ /workspaces/Learning-Rust/Introducción (main) $  # resultado
```
Es importante como la ubicación de la terminal cambio de: `/workspaces/Learning-Rust` a `/workspaces/Learning-Rust/Introducción`.

El comando:
```bash
cd ..
```
Nos permite posicionarnos en la carpeta anterior:

```bash
@DvdRivas ➜ /workspaces/Learning-Rust/Introducción (main) $ cd ..
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ 
```
Pasando de: `/workspaces/Learning-Rust/Introducción` a `/workspaces/Learning-Rust`

Es posible escribir direcciones completas o especificar mas de un cambio a la vez mediante una separación por `/`:
```bash
cd nombre_carpeta/nombre_sub_carpeta/nombre_sub_sub_carpeta
o
cd ../../ # Regresa a dos carpetas anteriores 
```

---

```bash
mkdir nombre_de_la_carpeta
```
Crea una nueva carpeta en el directorio actual:
```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # vemos la lista de los elementos actuales
images  Introducción  README.md
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ mkdir proyecto # Creamos una carpeta de nombre proyecto
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # volvemos a ver la lista de elementos
images  Introducción  proyecto  README.md # vemos a diferencia de la vez pasada una nueva carpeta de nombre proyecto
```

---

```bash
touch nombre_del_archivo.extension_del_archivo
```
Crea un archivo en blanco:

```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # vemos la lista de los elementos actuales
images  Introducción  proyecto  README.md
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ touch archivo.txt # Creamos un documento de texto (extension .txt) de nombre archivo 
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ touch readme.md # posteriormente, de igualmanera ahora creamos un archivo de tipo markdown (.md como el de esta pagina) de nombre readme
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # volvemos a ver la lista de elementos para verificar la creacion de los nuevos elementos
archivo.txt  images  Introducción  proyecto  readme.md  README.md  
```

---

```bash
cat nombre_del_archivo
```
Visualiza el contenido de un archivo:

```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ cat README.md # ponemos como ejemplo el README.md
# Learning-Rust
![Banner](./images/banner.jpg)
Bienvenidos a este repositorio, donde te guiaremos paso a paso en tu proceso de aprendizaje de Rust 🦀, no importa si eres nuevo en el ámbito de la programación o si ya tienes experiencia en algún otro lenguaje!
... # Continua con mas contenido
```

---

```bash
rm nombre_del_archivo
rm -r nombre_de_la_carpeta
```
Nos permite borrar archivos o carpetas **es mportante tener en cuenta que no se van a una papelera, son borrados definitivamente**:
```bash
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # vemos la lista de los elementos actuales
archivo.txt  images  Introducción  proyecto  readme.md  README.md
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ rm archivo.txt # borramos el archivo.txt
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ rm -r proyecto # borramos la carpeta proyecto
@DvdRivas ➜ /workspaces/Learning-Rust (main) $ ls # volvemos a ver la lista de elementos para verificar que ya no existen ambos elementos
images  Introducción  readme.md  README.md
```
## **Continuación: ¿Qué sigue?**
[Da clic aquí para ir a la siguiente sección](/2_tipos_de_variable/README.md)
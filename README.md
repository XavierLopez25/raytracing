# Minecraft Diorama Raytracer

https://github.com/user-attachments/assets/2f6a5260-f897-4b63-933d-38676d5d0607

---
## Descripción

Este programa es un **raytracer** que genera una escena de estilo *Minecraft* utilizando cubos texturizados y varias fuentes de luz. La escena incluye varios objetos como árboles, herramientas del jugador, cactus, shroomlights y un portal (el cual está imcompleto ya que el jugador de este Skyblock colocó mal la obsidiana para su portal). Además, el raytracer soporta múltiples efectos de iluminación, materiales personalizados con texturas, reflexión, refracción y sombras, además de incluir un **skybox** para dar profundidad a la escena. 

El programa está escrito en **Rust** y utiliza la librería **minifb** para la visualización en una ventana interactiva. Asimismo, implementa la paralelización del cálculo de rayos usando **Rayon**, lo cual mejora significativamente el rendimiento.

### Características principales:
- **Múltiples fuentes de luz**: Incluye luces que emiten desde posiciones definidas, como bloques de *shroomlight* y una luz principal blanca.
- **Materiales texturizados**: Cada bloque tiene su propia textura aplicada y sus características únicas, como reflexión y transparencia.
- **Cálculo de sombras**: Los objetos proyectan sombras basadas en la posición de las fuentes de luz, generando una escena más realista. **Soporte de multiples sombras**
- **Cámara interactiva**: La cámara puede moverse alrededor de la escena y hacer zoom.
- **Skybox**: Un fondo que da la sensación de un entorno infinito.
  
## Requisitos

- **Rust** (versión reciente)
- **Cargo** (herramienta de gestión de paquetes de Rust)
  
## Instalación y ejecución

1. Clona el repositorio en tu máquina local:
   ```bash
   git clone [https://github.com/usuario/diorama_raytracer.git](https://github.com/XavierLopez25/raytracing.git)
   ```
2. Navega al directorio del proyecto:
   ```bash
   cd raytracer
   ```
3. Para Windows, puedes construir el proyecto utilizando Powershell:
   ```powershell
   PowerShell -ExecutionPolicy Bypass -File .\run.ps1
   ```
4. En su defecto construye el proyecto usando **Cargo**:
   ```bash
   cargo build --release
   ```
5. Ejecuta el programa:
   ```bash
   cargo run --release
   ```

## Interacción

- **Movimiento de la cámara**:
  - `A/D` para mover la cámara en los ejes laterales.
  - `Q/E` para mover la cámara en el eje vertical.
  - `Up/Down` para acercar o alejar la cámara del centro.
  - `Left/Right` para rotar la cámara alrededor del centro.
  - `W/S` para inclinar la cámara hacia arriba o hacia abajo.

## Estructura del código

El código está dividido en varios módulos que manejan los aspectos claves del raytracer:

- **framebuffer**: Encargado de la representación de los píxeles en pantalla.
- **camera**: Controla el movimiento y la perspectiva de la cámara.
- **light**: Maneja las diferentes fuentes de luz.
- **material**: Define los materiales con sus propiedades visuales, como textura, albedo, reflexión y refracción.
- **texture**: Controla la carga de texturas y cómo se aplican a los objetos.
- **cube**: Define la geometría básica de los cubos que componen la escena.

## Evaluación según la rúbrica

### [20 puntos] Complejidad de la escena
- [x] La escena incluye una variedad de objetos, cada uno con materiales distintos, como shroomlights, cactus, bloques del nether, un portal, herramientas del jugador y un árbol, lo que demuestra una buena complejidad en términos de composición y materiales.

### [10 puntos] Atractivo visual
- [x] La escena es visualmente atractiva con un entorno inspirado en Minecraft. Los bloques de luz como *shroomlights* iluminan el área, y el *skybox* da profundidad. Además, las sombras generan un efecto visual más realista.

### [25 puntos] Performance del raytracer
- [x] El programa está paralelizado usando **Rayon**, lo que mejora el rendimiento. La escena se renderiza a un framerate estable dependiendo del hardware, incluso con múltiples fuentes de luz y sombras.

### [5 puntos por material] Diferentes materiales
- [x] Implementación de 5 materiales con sus propias texturas y parámetros:
  - *Grass* (textura, albedo, specular)
  - *Dirt* (textura, albedo, specular)
  - *Crafting Table* (textura, albedo, specular)
  - *Smoker* (textura, albedo, specular)
  - *Bookshelf* (textura, albedo, specular)
  - *Obsidian* (textura, albedo, specular)
  - *Shroomlight* (textura, albedo, specular, emisivo)
  - *Oak Log* (textura, albedo, specular)
  - *Oak Planks* (textura, albedo, specular)

### [15 puntos] Skybox
- [x] Se implementa un skybox que añade profundidad visual al renderizado.

### [5 puntos] Zoom en la cámara
- [x] La cámara tiene controles para acercarse y alejarse del centro de la escena.

### [5 puntos] Fresnel en transparencia y reflectividad
- [x] Se calcula el efecto Fresnel en materiales transparentes y reflectivos para mejorar la interacción visual de la luz.

### [10 puntos] Múltiples fuentes de luz
- [x] El programa soporta múltiples fuentes de luz con diferentes intensidades y colores, como la luz principal blanca y las luces anaranjadas emitidas por los bloques *shroomlight*.

### [20 puntos] Mapeo de normales
- [ ] No se ha implementado mapeo de normales.

### [10 puntos] Ciclo de día y noche
- [ ] No se ha implementado un ciclo de día y noche con cambios en la iluminación.

### [15 puntos] Materiales emisivos
- [x] Los materiales emisivos están implementados. Los bloques de *glowstone* y *shroomlight* emiten luz que afecta el entorno.

### [25 puntos] Animación de texturas
- [ ] No se ha implementado animación de texturas, como agua fluyendo o fuego.

---

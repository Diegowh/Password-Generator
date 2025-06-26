# Generador de Contraseñas
Aplicación de escritorio que genera contraseñas únicas y y reproducibles basadas en una contraseña maestra y un nombre.
## Características
- **Generación determinística**: La misma contraseña maestra + nombre de servicio siempre produce la misma contraseña
- **Seguro**: Utiliza hashing
- **Fondo dinámico**: El color de fondo cambia según tu contraseña maestra para darte confirmación visual
- **Copia al portapapeles**: Botón dedicado para copiar contraseñas de forma segura

## Cómo funciona

1. Introduce tu contraseña maestra
2. Introduce el nombre del sitio web o aplicación, como "github", "github.com", o cualquier combinación que te inventes
3. La aplicación genera una contraseña única combinando ambos elementos

## Instalación

### Descargar ejecutable (Recomendado)
1. Ve a la sección [Releases](../../releases)
2. Descarga el archivo `.exe` más reciente
3. Ejecuta la aplicación directamente

### Compilar desde código fuente

#### Requisitos previos
- [Rust](https://rustup.rs/) (versión 1.70 o superior)
- Git

#### Pasos para compilar

1. **Clona el repositorio**
   ```bash
   git clone https://github.com/tu-usuario/pass_generator.git
   cd pass_generator
   ```

2. **Instala las dependencias**
   ```bash
   cargo check
   ```

3. **Compila en modo release**
   ```bash
   cargo build --release
   ```

4. **Ejecuta la aplicación**
   ```bash
   cargo run --release
   ```

   O encuentra el ejecutable en: `target/release/password_generator.exe`

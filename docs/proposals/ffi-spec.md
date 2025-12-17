# Especificación de FFI para U (v0.9)

## 🎯 Objetivo
Permitir llamadas seguras desde **U → C** y **C → U** (callbacks), con gestión explícita de memoria y tipos.

## 1. Sintaxis

### Declaración de funciones externas
```ulang
extern "C" {
    fn printf(format: str, ...) -> i32;
    fn malloc(size: usize) -> ptr;
    fn free(ptr: ptr);
}
```
### Callbacks (C → U)

// Tipo de función para callback
type OnClick = fn() -> void;

extern "C" {
    fn skia_set_click_handler(callback: OnClick);
}

### Uso en bloque unsafe
fn main() {
    unsafe {
        printf("Hello from C!\n");
        skia_set_click_handler(handle_click);
    }
}

fn handle_click() {
    print("Button clicked!");
}

### 2. Mapeo de Tipos
U Type                C Type           Notas
i32                   int32_t          Directo
usize                 size_t           Directo
f64                   double           Directo
bool                  _Bool            Directo
str                   const char*      Conversión implícita (copia)
ptr                   void*            Puntero opaco
Option<T>             T*               None = NULL
fn() -> T             T (*)()          Solo para callbacks


### 3. Modelo de Seguridad
Todas las llamadas FFI deben estar en unsafe.
Los callbacks deben ser funciones de nivel superior (no closures).
El compilador generará wrappers C para callbacks:
// Generado automáticamente
void u_callback_handler_1() {
    u_handle_click(); // llama a función U
}

### 4. Gestión de Memoria
U → C: strings se copian a C (null-terminated).
C → U: el programador debe garantizar que los punteros sean válidos.
Callbacks: U debe mantener la función viva (no se descarga).



### 5. Requisitos para MVP
Soporte para extern "C" { }
Tipo ptr
Conversión str ↔ char*
Bloques unsafe
Callbacks de C a U (funciones globales)
10+ pruebas unitarias
Ejemplo: llamar printf y registrar un callback
✅ Este FFI habilita Skia, OpenGL, SDL, y SDKs móviles.

---

## 📄 Archivo 3: `docs/skia-binding.md`  
*(Binding inicial a Skia en C, listo para usar con FFI)*

```markdown
# Binding Inicial a Skia para U

## 🎯 Objetivo
Crear un conjunto mínimo de funciones C que expongan Skia para renderizado básico, listo para llamarse desde U vía FFI.

## 1. Archivo: `skia_glue.c`

```c
// skia_glue.c — Minimal Skia binding for U
#include "include/core/SkCanvas.h"
#include "include/core/SkSurface.h"
#include "include/core/SkColor.h"
#include "include/core/SkPaint.h"
#include "include/core/SkRect.h"
#include "include/core/SkString.h"
#include "include/ports/SkFontMgr.h"
#include "include/core/SkFont.h"
#include "include/gpu/GrDirectContext.h"

// Opaque handles (passed as `ptr` in U)
static SkSurface* g_surface = nullptr;
static SkCanvas* g_canvas = nullptr;

extern "C" {

void skia_init(int width, int height) {
    SkImageInfo info = SkImageInfo::MakeN32Premul(width, height);
    g_surface = SkSurface::MakeRaster(info).release();
    g_canvas = g_surface->getCanvas();
}

void skia_clear(uint32_t color) {
    g_canvas->clear(color);
}

void skia_draw_rect(float x, float y, float w, float h, uint32_t color) {
    SkPaint paint;
    paint.setColor(color);
    g_canvas->drawRect(SkRect::MakeXYWH(x, y, w, h), paint);
}

void skia_draw_text(const char* text, float x, float y, uint32_t color) {
    SkFont font(SkFontMgr::RefDefault()->makeFromName("Arial", SkFontStyle()), 16);
    SkPaint paint;
    paint.setColor(color);
    g_canvas->drawString(text, x, y, font, paint);
}

void skia_flush() {
    // For window systems: present to screen
    // For headless: noop
}

} // extern "C"
```
### 2. Dependencias
Skia: submódulo o build estático (BSD license → compatible con MIT).
Compilado con: zig c++ skia_glue.c -lskia -I./skia/include -L./skia/out/...

### 3. Uso desde U 
extern "C" {
    fn skia_init(width: i32, height: i32);
    fn skia_clear(color: u32);
    fn skia_draw_rect(x: f64, y: f64, w: f64, h: f64, color: u32);
    fn skia_draw_text(text: str, x: f64, y: f64, color: u32);
    fn skia_flush();
}

fn main() {
    unsafe {
        skia_init(800, 600);
        skia_clear(0xFF121212); // gris oscuro
        skia_draw_rect(100.0, 100.0, 200.0, 100.0, 0xFFFF0000); // rojo
        skia_draw_text("¡Hola U!", 120.0, 150.0, 0xFFFFFFFF); // blanco
        skia_flush();
    }
}

### 4. Próximos pasos
Añadir soporte para eventos (mouse, teclado).
Integrar con backend de ventana (Linux: X11, Windows: Win32, etc.).
Migrar a backend GPU (Vulkan/Metal) para móviles.



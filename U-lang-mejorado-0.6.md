# U: Diseño Mejorado y Directrices Legales para un Lenguaje Open Source

> **Versión 0.6** — Diseño técnico + marco legal + ajustes realistas post evaluación técnica  
> ✅ MVP enfocado | ✅ Runtime transparente | ✅ Reglas de ownership explícitas | ✅ Terminología precisa

---

## 🎯 Visión Ajustada

U es un lenguaje **de sistemas moderno** que combina:

- ✅ **Seguridad estructural**: sin null, sin data races por memoria compartida, sin UB.
- ✅ **Control de bajo nivel**: sin garbage collector, sin abstracciones ocultas.
- ✅ **Productividad**: sintaxis limpia (Python-like), inferencia de tipos, errores expresivos.
- ✅ **Portabilidad real**: binarios estáticos y autocontenidos para Linux, Windows, macOS y WebAssembly (WASI).
- ✅ **Extensibilidad**: metaprogramación en tiempo de compilación.

> **Filosofía**: *"Zero-cost safety, human-first syntax, native everywhere — sin promesas prematuras."*

---

## ⚖️ Directrices Legales y de Originalidad

### 1. **Código 100% original**
- Todo el compilador, runtime (si aplica), bibliotecas estándar y herramientas se escribirán **desde cero**.
- No se copiará código fuente, gramáticas literales, documentación técnica ni especificaciones de otros lenguajes (Rust, Zig, Go, etc.), salvo fragmentos triviales o de dominio público con atribución explícita.

### 2. **Inspiración permitida, copia prohibida**
- Está permitido **tomar ideas conceptuales**: ownership, actores, inferencia de tipos, AOT compilation, etc.
- **No está permitido** replicar estructuras patentadas, gramáticas idénticas, macros complejas o diseños propietarios sin adaptación sustancial.
- La sintaxis de U será **original**, aunque inspirada en Python, Rust y Nim — al igual que lo hicieron Swift, Kotlin o Julia.

### 3. **Licencia: MIT**
- Todo el proyecto se distribuirá bajo **MIT License**.
- Compatible con uso comercial, modificación y redistribución.
```text
Copyright (c) 2025 Webcien and U contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```


### 4. **Herramientas externas (uso seguro)**
- **Zig (MIT)**: usado solo como enlazador y toolchain de cross-compilación.
- **LLVM (Apache 2.0 + LLVM Exception)**: opcional, compatible con MIT.
- **Rust (MIT/Apache 2.0)**: lenguaje de implementación; no afecta la licencia del proyecto.

### 5. **Atribuciones**
Se mantendrá un archivo ACKNOWLEDGEMENTS.md para:
Reconocer ideas inspiradas en otros lenguajes.
Listar herramientas usadas (Zig, LLVM, etc.).
Agradecer a la comunidad open source.
- Ejemplo: _“Inspirado en el modelo de ownership de Rust y el enfoque de cross-compilation de Zig.”_

---

## 🔧 Principios Técnicos Clarificados

| Principio | Implementación precisa |
|---------|------------------------|
| **Memoria segura** | Modelo de ownership simplificado (ver sección abajo). |
| **Ningún `null`** | Todos los tipos son non-nullable; se usa `Option<T>`. |
| **Concurrencia** | Actores con **solo paso de mensajes** → **Prevención estructural de data races mediante ausencia de memoria compartida.**. |
| **Runtime** | **Micro-runtime estático y opcional** (≤5 KB) para actores: scheduling cooperativo, mailboxes, ciclo de vida. Solo se enlaza si se usa concurrencia. |
| **Tipado** | Estático obligatorio con inferencia. Todo resuelto en compile-time. |
| **Binarios** | AOT → código C → enlazado con Zig → binario estático sin dependencias. |

---

## 🧱 Reglas Formales de Ownership (v0.6)

Para evitar ambigüedad y garantizar seguridad sin complejidad:

1. **Cada valor tiene exactamente un dueño**.
2. El dueño puede ser **mutable** o **inmutable**, pero no ambos a la vez.
3. **Copias profundas** requieren llamada explícita a `.clone()`.
4. **Referencias inmutables** están permitidas, pero sólo dentro del ámbito léxico que las crea.
5. **No hay borrowing mutable**.
6. **No hay lifetimes explícitos**; el compilador infiere validez por ámbito.
7. La **transferencia de ownership** ocurre en asignación o llamada a función.

> Esto es suficiente para seguridad en memoria y evita reimplementar Rust.

---

### 📁 Estructura del Proyecto: u-lang/
La estructura completa y proyectada del repositorio de U, diseñada para:
- Cumplir con buenas prácticas de ingeniería de software,
- Ser compatible con la licencia MIT,
- Facilitar el desarrollo incremental del compilador,
- Incluir documentación legal y técnica desde el inicio,
- Soportar multiplataforma mediante Zig,
- Y preparar el terreno para una comunidad open source.


u-lang/
├── .github/
│   └── workflows/
│       ├── ci.yml                 # CI/CD: builds en Linux, Windows, macOS, WASM
│       └── release.yml            # Publicación automatizada de binarios
│
├── compiler/                      # Código fuente del compilador (100% original)
│   ├── src/
│   │   ├── lexer.rs               # Análisis léxico
│   │   ├── parser.rs              # Análisis sintáctico (gramática EBNF propia)
│   │   ├── ast.rs                 # Árbol sintáctico abstracto
│   │   ├── type_checker.rs        # Inferencia y verificación de tipos
│   │   ├── ownership_checker.rs   # Verificación de seguridad en memoria
│   │   ├── concurrency_checker.rs # Prevención de data races
│   │   ├── codegen/
│   │   │   ├── c.rs               # Generador de código C (para Zig)
│   │   │   └── llvm.rs            # Opcional: generador de LLVM IR
│   │   └── main.rs                # CLI del compilador
│   └── Cargo.toml                 # Dependencias del compilador (solo crates MIT/Apache2)
│
├── std/                           # Biblioteca estándar de U (opcional en MVP)
│   ├── core.ul                    # Tipos básicos, Option, Result
│   ├── mem.ul                     # Gestión segura de memoria
│   └── actor.ul                   # Sistema de actores para concurrencia
│
├── examples/                      # Ejemplos verificables
│   ├── hello.ul
│   ├── fibonacci.ul
│   ├── concurrent_counter.ul
│   └── web_wasm.ul                # Ejemplo para WebAssembly
│
├── tests/                         # Suite de pruebas
│   ├── lexer/
│   ├── parser/
│   ├── type_check/
│   └── integration/               # Pruebas de binarios generados
│
├── tools/
│   └── ul-build/                  # Wrapper para Zig (ul build --target ...)
│
├── docs/
│   ├── SPEC.md                    # Especificación del lenguaje (gramática, semántica)
│   ├── ROADMAP.md                 # Basado en U-lang-mejorado-0.5.md
│   └── TUTORIAL.md                # Guía para nuevos usuarios
│
├── ACKNOWLEDGEMENTS.md            # Reconocimientos legales y éticos
├── LICENSE                        # Licencia MIT (ver abajo)
├── README.md                      # Presentación del proyecto + cómo contribuir
├── CONTRIBUTING.md                # Guía para contribuidores
└── .gitignore


### 🧱 Arquitectura del Compilador (Propia y Original)
1. Frontend: Lexer + parser en Rust (implementación original con lalrpop o nom).
2. Middle-end: Verificador de seguridad (ownership, bounds, race conditions).
3. Backend:
   - Por defecto: genera código C portable → compilado con zig cc.
   - Opcional: genera LLVM IR para optimizaciones avanzadas.
4. Build system: ul build --target x86_64-linux (inspirado en Cargo, pero más simple).
Todo el código del compilador será propio, auditable y compatible con MIT.

5. La extensión de archivo recomendada para los archivos fuente de U es:
.ul
Breve y clara: .ul deriva directamente del nombre del lenguaje (U).
✅ Recomendación oficial en el proyecto:
Usa .ul para todos los archivos fuente de U.

### 🔧 Flujo de Compilación Propuesto

archivo.ul 
   │
   ▼
Lexer → Parser → AST 
   │
   ▼
Type + Ownership + Concurrency Checker 
   │
   ▼
Generador de código C 
   │
   ▼
zig cc -target <plataforma> -lc → binario estático.

### 🗺️ Roadmap Técnico + Legal
Fase                                  Objetivo                                  Acción Legal/Técnica
1. Especificación                     Gramática EBNF original                   No copiar de otros lenguajes; validar con ejemplos propios
2. MVP compilador                     print("Hola") en 3 plataformas            Implementación 100% desde cero
3. Sistema de tipos                   Null-safety + ownership                   
4. Concurrencia                       Actores sin data races                    Inspirado, no replicado
5. Cross-compilation                  Soporte con Zig                           Uso permitido por licencia MIT de Zig
6. Lanzamiento Alpha                  Repositorio público + docs                Incluir LICENSE (MIT) y ACKNOWLEDGEMENTS.md



## 🧪 MVP Realista (v0.6)

### ✅ Incluido
- Tipos básicos (`i32`, `str`, `bool`, `Option<T>`, `Result<T, E>`)
- Funciones y control de flujo
- Inferencia de tipos local
- Ownership básico (reglas de arriba)
- Actores simples (spawn, send, await)
- Generación de código C → enlace con Zig
- CLI: `ul build --target <plataforma>`
- Soporte inicial: **Linux, Windows, macOS, WebAssembly (WASI)**

### 🚫 Fuera del MVP (pero planificado)
- Android/iOS → **fase experimental v0.7+**
- Traits/interfaces
- Macros
- Sistema de paquetes
- Debugger, linter, formatter
- ABI estable

> **Razón**: Enfocarse en demostrar seguridad, compilación y concurrencia básica antes de expandir alcance.

---

## 🧱 Arquitectura del Compilador (Propia y Evolutiva)



- **Frontend**: Rust con `lalrpop` (gramática EBNF propia).
- **Middle-end**: Verificadores modulares e independientes.
- **Backend C**: Portable y fácil de auditar.
- **Backend LLVM**: Opcional en fases posteriores.

---

## 🗺️ Roadmap Ajustado

| Fase | Objetivo | Alcance |
|------|--------|--------|
| v0.6 | MVP funcional | Linux, Windows, macOS, WASM; actores simples; CLI |
| v0.7 | Plataformas experimentales | Android (CLI via NDK), iOS (CLI via toolchain) |
| v0.8 | Tooling básico | Formatter, linter, VS Code plugin |
| v0.9 | Ecosistema | Gestor de paquetes, stdlib extendida |
| v1.0 | Estabilidad | ABI fijo, especificación completa, CI en 6+ plataformas |

---


### ✅ Conclusión
U será un proyecto ético, legal y técnicamente original, que:

- Respeta las licencias de la comunidad open source.
- Evita riesgos de copyright mediante diseño propio.
- Se beneficia de ideas públicas sin depender de código ajeno.
- Está listo para crecer bajo la licencia MIT, con total libertad para uso, modificación y distribución.
- **Técnicamente viable**: con un MVP reducido pero completo.
- **Legalmente seguro**: bajo MIT, con código original y atribuciones claras.
- **Estratégicamente enfocado**: evita sobre-ingeniería temprana.
- **Honesto en sus capacidades**: terminología precisa, promesas realistas.
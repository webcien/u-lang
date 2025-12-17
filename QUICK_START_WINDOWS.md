# Guía Rápida: U Language en Windows

**Fecha:** 17 de diciembre de 2025  
**Versión:** 1.5.0

---

## 🚀 Opción 1: WSL2 (RECOMENDADO)

Esta es la forma más fácil y confiable de usar U en Windows.

### Paso 1: Instalar WSL2

```powershell
# En PowerShell como Administrador
wsl --install
```

Reinicia tu computadora.

### Paso 2: Instalar Dependencias en WSL

```bash
# Dentro de WSL (Ubuntu)
sudo apt update
sudo apt install -y build-essential curl git

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Instalar Zig
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /usr/local/zig
echo 'export PATH="/usr/local/zig:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Paso 3: Clonar y Compilar U

```bash
git clone https://github.com/webcien/u-lang.git
cd u-lang/compiler
cargo build --release
```

### Paso 4: Compilar tu Primer Programa

```bash
cd ../examples

# Compilar para Linux
../compiler/target/release/ul build hello.ul
./hello

# Compilar para Windows
../compiler/target/release/ul build hello.ul --target x86_64-windows

# Copiar el .exe a Windows
cp hello.exe /mnt/c/Users/TU_USUARIO/Desktop/
```

Ahora puedes ejecutar `hello.exe` desde tu escritorio de Windows.

---

## 🔧 Opción 2: Windows Nativo (EXPERIMENTAL)

### Paso 1: Instalar Rust

1. Descarga Rust desde https://rustup.rs/
2. Ejecuta `rustup-init.exe`
3. Sigue las instrucciones

### Paso 2: Instalar Zig

1. Descarga Zig desde https://ziglang.org/download/
2. Extrae el archivo ZIP
3. Agrega Zig al PATH:
   - Busca "Variables de entorno" en Windows
   - Edita "Path"
   - Agrega la ruta donde extrajiste Zig

### Paso 3: Instalar Git

1. Descarga Git desde https://git-scm.com/download/win
2. Instala con las opciones por defecto

### Paso 4: Clonar y Compilar U

```powershell
# En PowerShell
git clone https://github.com/webcien/u-lang.git
cd u-lang\compiler
cargo build --release
```

### Paso 5: Compilar tu Primer Programa

```powershell
cd ..\examples
..\compiler\target\release\ul.exe build hello.ul
.\hello.exe
```

---

## 📝 Ejemplos de Programas

### Hello World

```u
// hello.ul
fn main() {
    unsafe {
        printf("Hello from U Language!\n");
    }
    return 0;
}

extern "C" {
    fn printf(format: ptr, ...);
}
```

Compilar:
```bash
ul build hello.ul --target x86_64-windows
```

### Calculadora

```u
// calc.ul
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let result = add(10, 20);
    unsafe {
        printf("10 + 20 = %d\n", result);
    }
    return 0;
}

extern "C" {
    fn printf(format: ptr, ...);
}
```

---

## 🎯 Qué Puedes Crear en Windows

### ✅ Totalmente Funcional
- CLI tools (herramientas de línea de comandos)
- Data processors (procesadores de datos)
- Parsers y compiladores
- Utilidades de desarrollo

### ⚠️ Requiere Configuración Adicional
- GUI applications (requiere Skia)
- Network servers (requiere FFI a winsock)

---

## 🐛 Solución de Problemas

### Error: "zig: command not found"

**Solución:** Asegúrate de que Zig esté en tu PATH.

```powershell
# Verificar
zig version
```

### Error: "cargo: command not found"

**Solución:** Reinicia tu terminal después de instalar Rust.

### Error al compilar con Zig

**Solución:** Usa `--no-link` para generar solo el código C:

```bash
ul build hello.ul --no-link
# Genera hello.c

# Compilar manualmente con gcc/clang
gcc hello.c -o hello.exe
```

---

## 📚 Recursos

- **Repositorio:** https://github.com/webcien/u-lang
- **Documentación:** https://github.com/webcien/u-lang/tree/master/docs
- **Ejemplos:** https://github.com/webcien/u-lang/tree/master/examples

---

**¡Bienvenido a U Language en Windows! 🚀**

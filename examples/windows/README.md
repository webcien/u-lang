# U Language - Windows GUI Examples

Este directorio contiene ejemplos de aplicaciones GUI para Windows usando U Language.

## Requisitos

1. **Compilador U** - Compilado con `build.bat` o `build.ps1`
2. **Zig** - Para compilación (https://ziglang.org/download/)
3. **Skia** (Opcional) - Para GUI real. Si no está instalado, se usa una implementación mock.

## Instalación de Skia (Opcional)

Para GUI con renderizado real, instala Skia:

```powershell
.\scripts\download_skia_windows.ps1
```

O manualmente desde: https://github.com/JetBrains/skia-build/releases

## Compilar y Ejecutar

### Opción 1: Script Automático (Recomendado)

```powershell
# Compilar
..\..\ compiler\target\release\ul.exe build hello_gui.ul --no-link

# Linkear
..\..\scripts\link_gui_windows.ps1 hello_gui

# Ejecutar
.\hello_gui.exe
```

### Opción 2: Manual

```powershell
# Compilar a C
..\..\ compiler\target\release\ul.exe build hello_gui.ul --no-link

# Compilar con MSVC
cl hello_gui.c ..\..\runtime\skia_windows.c /Fe:hello_gui.exe /link user32.lib gdi32.lib

# O con MinGW
gcc hello_gui.c ..\..\runtime\skia_windows.c -lgdi32 -luser32 -o hello_gui.exe

# Ejecutar
.\hello_gui.exe
```

## Ejemplos Disponibles

### 1. hello_gui.ul
Aplicación GUI simple con texto y botón.

```powershell
ul build hello_gui.ul --no-link
..\..\scripts\link_gui_windows.ps1 hello_gui
.\hello_gui.exe
```

### 2. calculator_gui.ul
Calculadora con interfaz gráfica.

```powershell
ul build calculator_gui.ul --no-link
..\..\scripts\link_gui_windows.ps1 calculator_gui
.\calculator_gui.exe
```

## Notas

- Sin Skia instalado, los ejemplos usan una implementación mock que imprime a consola
- Con Skia instalado, los ejemplos generan archivos PNG con el renderizado real
- Para aplicaciones interactivas completas, se requiere integración con el event loop de Windows

## Solución de Problemas

### Error: "SKIA_DIR not set"
Esto es normal si no has instalado Skia. Los ejemplos funcionarán con la implementación mock.

### Error: "cl: command not found"
Instala Visual Studio o usa MinGW:
```powershell
choco install mingw
```

### Error al compilar
Verifica que Zig esté instalado:
```powershell
zig version
```

## Próximos Pasos

- Instalar Skia para renderizado real
- Explorar más ejemplos en `../`
- Crear tus propias aplicaciones GUI

## Licencia

MIT License — Copyright (c) 2025 Webcien and U contributors

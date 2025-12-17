
---

## 📄 Archivo 4: `docs/ui-dsl.md`  
*(DSL declarativo para UI en U — gramática y ejemplos)*

```markdown
# DSL Declarativo de UI para U

## 🎯 Objetivo
Permitir construir interfaces gráficas modernas con código **declarativo, seguro y conciso** en U.

## 1. Principios

- **Declarativo**: describes *qué* quieres, no *cómo* hacerlo.
- **Reactivo**: cambios en estado → actualización automática.
- **Tipo-seguro**: el compilador valida la estructura.
- **Responsive**: sistema de layout integrado.

## 2. Gramática (EBNF parcial)

```ebnf
ui_expression = widget_constructor | identifier ;
widget_constructor = "ui.", identifier, "(", [field_assignments], ")" ;
field_assignments = field_assignment, { ",", field_assignment } ;
field_assignment = identifier, ":", expression ;
```
## 3. Ejemplos
Hola mundo

import ui

fn main() {
    ui.run(ui.Text("¡Hola desde U!"));
}

ui.run inicializa:
- el backend de ventana por plataforma,
- el loop de eventos del sistema,
- y el motor de renderizado (Skia).
Skia se usa exclusivamente para dibujo.


### App interactiva
import ui

fn main() {
    ui.run(
        ui.Column {
            children: [
                ui.Text("Contador: 0"),
                ui.Button("Incrementar", on_click: increment),
            ]
        }
    );
}

fn increment() {
    // Lógica de estado (se implementará en v1.0+)
}

## Layout responsive
ui.Container {
    width: ui.Fill,       // ocupa todo el ancho
    child: ui.Row {
        spacing: 10.0,
        children: [
            ui.Button("OK", on_click: ok),
            ui.Button("Cancelar", on_click: cancel),
        ]
    }
}

## 4. Widgets Básicos 
Widget                           Propiedades                                  Uso
Text                             text: str, color: u32                        Mostrar texto
Button                           label: str, on_click: fn()                   Botón interactivo
Input                            value: str, on_change: fn(str)               Campo de texto
Column                           children: [Widget], spacing: f64             Layout vertical
Row                              children: [Widget], spacing: f64             Layout horizontal
Container                        width, height, padding, child                Contenedor con estilo


## 5. Sistema de Tipos
Todos los widgets implementan el trait Widget (se agregará en v1.0).
El compilador valida que children sea una lista de Widget.

## 6. Próximos pasos
Diseñar sistema de estado reactivo.
Implementar layout engine (flexbox-like).
Añadir temas (modo claro/oscuro).
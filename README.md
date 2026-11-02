# awg_rust

Aplicación de escritorio escrita en **Rust** para consultar y calcular equivalencias de calibres de cable **AWG** (American Wire Gauge), con interfaz gráfica construida con [egui/eframe](https://github.com/emilk/egui).

> Conversión AWG ↔ mm², diámetro, resistencia, capacidad de corriente, etc.).

## Características

- Interfaz gráfica ligera y multiplataforma (backend `glow`).
- Equivalencias entre calibres AWG y unidades métricas.
- Compilación optimizada en modo release (`opt-level = 3`).

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) 1.85 o superior (el proyecto usa `edition = "2024"`).
- Linux: puede requerir librerías de sistema para ventanas, por ejemplo:

  ```bash
  sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libgl1-mesa-dev
  ```

## Instalación y uso

```bash
# Clonar el repositorio
git clone https://github.com/TU_USUARIO/awg_equiv.git
cd awg_equiv

# Ejecutar en modo desarrollo
cargo run

# Compilar versión optimizada
cargo build --release
./target/release/awg_equiv
```

## Dependencias principales

| Crate    | Versión | Uso                         |
|----------|---------|-----------------------------|
| `eframe` | 0.37.0  | Framework de GUI (egui)     |

## Estructura del proyecto

```text
awg_equiv/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

## Contribuir

Las contribuciones son bienvenidas. Abre un *issue* o envía un *pull request* con tus cambios.

## Licencia

Define aquí la licencia de tu proyecto (por ejemplo, MIT o Apache-2.0) y agrega el archivo `LICENSE` correspondiente.

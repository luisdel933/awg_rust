use eframe::egui;
use std::f64::consts::PI;

fn diametro_awg(awg: f64) -> f64 {
    0.127 * 92.0_f64.powf((36.0 - awg) / 39.0)
}

fn area_awg(awg: f64) -> f64 {
    let d = diametro_awg(awg);
    PI * (d / 2.0).powi(2)
}

// Tabla AWG 0..=40
fn awg_tabla() -> Vec<(i32, f64)> {
    (0..=40).map(|i| (i, area_awg(i as f64))).collect()
}

fn awg_equivalente_por_area(tabla: &[(i32, f64)], area: f64) -> (i32, f64) {
    for i in (0..tabla.len()).rev() {
        if tabla[i].1 >= area {
            return tabla[i];
        }
    }
    tabla[0]
}

fn buscar_awg_por_area_inferior(tabla: &[(i32, f64)], area: f64) -> Option<(i32, f64)> {
    for i in 0..tabla.len() {
        if tabla[i].1 <= area {
            return Some(tabla[i]);
        }
    }
    None
}

struct Row {
    awg: String,
    hilos: String,
}

struct App {
    rows: Vec<Row>,
    result: String,
    tabla: Vec<(i32, f64)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rows: vec![Row { awg: String::new(), hilos: String::new() }],
            result: String::new(),
            tabla: awg_tabla(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.set_max_width(720.0);
            ui.heading(egui::RichText::new("Calculadora AWG equivalente").color(egui::Color32::from_rgb(56, 189, 248)).size(24.0));
            ui.add_space(8.0);

            let w = ui.available_width();
            for i in 0..self.rows.len() {
                ui.horizontal(|ui| {
                    let row = &mut self.rows[i];
                    ui.add(egui::TextEdit::singleline(&mut row.awg).hint_text("Calibre AWG").desired_width(w * 0.5 - 5.0));
                    ui.add(egui::TextEdit::singleline(&mut row.hilos).hint_text("Número de hilos").desired_width(w * 0.5 - 5.0));
                });
            }

            ui.add_space(4.0);
            let btn_w = ui.available_width();
            if ui.add_sized([btn_w, 28.0], egui::Button::new("➕ Agregar calibre")).clicked() {
                self.rows.push(Row { awg: String::new(), hilos: String::new() });
            }
            if ui.add_sized([btn_w, 28.0], egui::Button::new("📐 Calcular")).clicked() {
                self.calcular();
            }

            ui.add_space(8.0);
            if !self.result.is_empty() {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Resultado").strong());
                    ui.separator();
                    for line in self.result.lines() {
                        ui.label(line);
                    }
                });
            }
        });
    }
}

impl App {
    fn calcular(&mut self) {
        let mut area_total = 0.0_f64;
        for row in &self.rows {
            if let (Ok(awg), Ok(n)) = (row.awg.trim().parse::<f64>(), row.hilos.trim().parse::<f64>()) {
                if n > 0.0 {
                    area_total += area_awg(awg) * n;
                }
            }
        }

        let awg_eq = awg_equivalente_por_area(&self.tabla, area_total);

        let mut out = String::new();
        out.push_str(&format!("Área total: {:.4} mm²\n", area_total));
        out.push_str(&format!("AWG equivalente: AWG {} ({:.4} mm²)\n\n", awg_eq.0, awg_eq.1));
        out.push_str("Descomposición AWG (referencial):\n");

        let smallest = self.tabla[40].1;
        let mut resto = area_total;
        let mut paso = 1;
        while resto >= smallest {
            match buscar_awg_por_area_inferior(&self.tabla, resto) {
                Some(found) => {
                    out.push_str(&format!("{}. AWG {} → {:.4} mm²\n", paso, found.0, found.1));
                    resto -= found.1;
                    paso += 1;
                    if paso > 50 { break; }
                }
                None => break,
            }
        }
        out.push_str(&format!("\nÁrea residual: {:.4} mm²", resto));

        self.result = out;
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("AWG equivalente por área")
            .with_inner_size([760.0, 620.0]),
        ..Default::default()
    };
    eframe::run_native(
        "AWG equivalente por área",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(App::default()))
        }),
    )
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default idades when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    visible: bool,
    nome: String,
    massa: f32,
    idade: i32,
    apresentacao_selecionada: String,
    apresentacoes: Vec<String>, //#[serde(skip)] // This how you opt-out of serialization of a field
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            visible: true,
            nome: "Nome".to_owned(),
            massa: 2.5,
            idade: 2,
            apresentacoes: vec!["mg/dL".to_owned(), "mg/Kg/h".to_owned()],
            apresentacao_selecionada: "mg".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Window::new("Identificação")
            .default_pos((100.0, 100.0)) // Posição inicial
            .default_width(80.0)
            .default_height(80.0)
            .resizable(true) // Permite redimensionar
            .show(ctx, |ui| {
                ui.label("Ficha pessoal");
                ui.horizontal(|ui| {
                    ui.label("Nome do paciente: ");
                    ui.text_edit_singleline(&mut self.nome);
                });
            });

        if self.visible {
            let _ = egui::Area::new("popup".into())
                .movable(true)
                .show(ctx, |ui| {
                    ui.heading("Popup Flutuante");
                    ui.label("Este é um exemplo de área flutuante.");
                    if ui.button("Fechar").clicked() {
                        self.visible = false;
                    }
                });
        };

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Prescrição detalhada");

            ui.horizontal(|ui| {
                ui.label("Nome do paciente: ");
                ui.text_edit_singleline(&mut self.nome);
            });

            ui.add(egui::Slider::new(&mut self.idade, 0..=100).text("Idade"));
            if ui.button("Incrementar 1 ano").clicked() {
                self.idade += 1;
            };

            ui.add(egui::Slider::new(&mut self.massa, 1.0..=200.0).text("Massa"));
            if ui.button("Incrementar 100g").clicked() {
                self.massa += 0.1;
            };
            if ui.button("Incrementar 1kg").clicked() {
                self.massa += 1.0;
            };

            egui::ComboBox::from_label("Escolha uma apresentação")
                .selected_text(&self.apresentacao_selecionada)
                // Adiciona as opções ao menu
                .show_ui(ui, |ui: &mut egui::Ui| {
                    ui.selectable_value(
                        &mut self.apresentacao_selecionada,
                        "mg/dL".to_string(),
                        "mg/dL",
                    );
                    ui.selectable_value(
                        &mut self.apresentacao_selecionada,
                        "mg/Kg".to_string(),
                        "mg/Kg",
                    );
                    ui.selectable_value(&mut self.apresentacao_selecionada, "mg".to_string(), "mg");
                });
            // for item in &self.apresentacoes {
            //             ui.selectable_value(
            //                 &mut self.apresentacao_selecionada, // Referência mutável ao item selecionado
            //                 item.clone(),            // Valor a ser definido se selecionado
            //                 item,                    // Texto exibido
            //             );
            //         }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.hyperlink_to(
            "Um projeto de Jefferson T.",
            "https://jeffersontorres.com.br/",
        );
    });
}

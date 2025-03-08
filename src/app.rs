//use serde::{Deserialize, Serialize};

use crate::{
    data::BULARIO, Aplicacao, Apresentacao, Float, Idade, Massa, Medicamento, Posologia, Via,
    Volume,
};

//#[derive(Serialize, Deserialize)]
pub struct Instancia {
    // Example stuff:
    visible: bool,
    nome: String,
    massa: Massa,
    idade: Idade,
    medicamento_selecionado: Medicamento,
    apresentacao_selecionada: Apresentacao, //#[serde(skip)] // This how you opt-out of serialization of a field
    posologia_selecionada: Posologia,
}

impl Default for Instancia {
    fn default() -> Self {
        Self {
            // Example stuff:
            visible: true,
            nome: "Algum medicamento".to_owned(),
            massa: Massa::G(Float(0.0)),
            idade: Idade::Anos(2),
            apresentacao_selecionada: Apresentacao::DoseVolume(
                Massa::Mg(Float(0.0)),
                Volume::Ml(Float(0.0)),
            ),
            medicamento_selecionado: Medicamento {
                nome: "medicamento",
                nome_comercial: None,
                apresentacoes: &[Apresentacao::DoseVolume(
                    Massa::Mg(Float(0.0)),
                    Volume::Ml(Float(0.0)),
                )],
                posologias: &[Posologia::DoseUnica(Massa::Mg(Float(0.0)))],
                advertencias: None,
            },
            posologia_selecionada: Posologia::DoseUnica(Massa::Mg(Float(0.0))),
        }
    }
}

///// Called once before the first frame.
//pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//// This is also where you can customize the look and feel of egui using
//// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
//
//// Load previous app state (if any&).
/// Note that you must enable the `persistence` feature for this to work.
//// if let Some(storage) = cc.storage {
////     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
//// }
//
//Default::default&()
//}

impl eframe::App for Instancia {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Window::new("Paciente")
            // .default_pos((100.0, 100.0)) // Posição inicial
            // .default_width(80.0)
            // .default_height(80.0)
            .resizable(true) // Permite redimensionar
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // let mut valor_massa: f32 = self.massa.clone().valor();

                    if ui.button("Incrementar 100g").clicked() {
                        self.massa.incrementar(0.1);
                    };
                    if ui.button("Incrementar 1kg").clicked() {
                        self.massa.incrementar(1.0);
                    };

                    if ui.button("Decrementar 100g").clicked() {
                        self.massa.incrementar(-0.1);
                    };
                    if ui.button("Decrementar 1kg").clicked() {
                        self.massa.incrementar(-1.0);
                    };
                    ui.label("Massa (Kg)");
                    ui.text_edit_singleline(&mut format!("{}", self.massa.valor()));
                    // self.massa.update(valor_massa);
                });

                ui.horizontal(|ui| {
                    ui.label("Idade");
                    // ui.add(egui::DragValue::new(&mut self.idade.valor()));

                    egui::DragValue::new(self.massa.)
                        .range(0..=120)
                        .suffix(" Kg");
                    egui::ComboBox::from_id_salt("Idade")
                        .selected_text(&format!("{}", self.idade))
                        .show_ui(ui, |ui: &mut egui::Ui| {
                            ui.selectable_value(
                                &mut self.idade,
                                Idade::Anos(2),
                                Idade::Anos(0).tipo(),
                            );
                            ui.selectable_value(
                                &mut self.idade,
                                Idade::Meses(2),
                                Idade::Meses(0).tipo(),
                            );
                        });
                });
            });

        egui::Window::new("Medicamento")
            // .default_pos((100.5, 100.5))
            // .default_width(88.0)
            // .default_height(88.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ComboBox::from_label("Nome")
                    .selected_text(&format!("{}", self.medicamento_selecionado.nome))
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        //for (i, medicamento) in BULARIO {
                        for medicamento in BULARIO {
                            ui.selectable_value(
                                &mut self.medicamento_selecionado,
                                medicamento.clone(), //ok?
                                //medicamento.nome.to_string(),
                                format!("{}", medicamento),
                            );
                        }
                    });

                let mut apresentacoes: Vec<&Apresentacao> = vec![];
                let mut posologias: Vec<&Posologia> = vec![];
                for apresentacao in self.medicamento_selecionado.apresentacoes {
                    apresentacoes.push(apresentacao);
                }
                for posologia in self.medicamento_selecionado.posologias {
                    posologias.push(posologia);
                }

                egui::ComboBox::from_label("Apresentações")
                    .selected_text(&format!("{}", self.apresentacao_selecionada))
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        for apresentacao in apresentacoes {
                            ui.selectable_value(
                                &mut self.apresentacao_selecionada,
                                apresentacao.clone(), //ok?
                                //format!("{:?}", apresentacao),
                                format!("{}", apresentacao),
                            );
                        }
                    });
                egui::ComboBox::from_label("Posologias")
                    .selected_text(&format!("{}", self.posologia_selecionada))
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        for posologia in posologias {
                            ui.selectable_value(
                                &mut self.posologia_selecionada,
                                posologia.clone(), //ok?
                                //format!("{:?}", apresentacao),
                                format!("{}", posologia),
                            );
                        }
                    });

                // if egui::Button::new("Prescreva!")).clicked() {
                //     match
                // }
            });

        // let apresentacao_text: String = match apresentacao {
        //     &Apresentacao::DoseVolume(
        //         dose,
        //         volume,
        //         unidade,
        //     ) => format!("{}{}/{}", dose, unidade, volume),
        //     Apresentacao::DoseAplicacao(dose, aplicacao) => {
        //         format!("{} por {:?}", dose, aplicacao)
        //     }
        // };
        // if self.visible {
        //     let _ = egui::Area::new("popup".into())
        //         .movable(true)
        //         .show(ctx, |ui| {
        //             ui.heading("Popup Flutuante");
        //             ui.label("Este é um exemplo de área flutuante.");
        //             if ui.button("Fechar").clicked() {
        //                 self.visible = false;
        //             }
        //         });
        // };

        egui::TopBottomPanel::bottom("top_panel").show(ctx, |ui| {
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
            ui.heading("Prescrição detalhada");
            ui.horizontal(|ui| {
                ui.label(&self.nome);
                let fill_no = 100 - self.medicamento_selecionado.nome.len() - 8;
                let fill = std::iter::repeat('.').take(fill_no).collect::<String>();
                ui.label(format!(
                    "\n\nVia oral\n{}{}{}",
                    self.medicamento_selecionado.nome, fill, "1 frasco"
                ));
            });
        });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's

        //     ui.add(egui::Slider::new(&mut self.idade, 0..=100).text("Idade"));
        //     if ui.button("Incrementar 1 ano").clicked() {
        //         self.idade += 1;
        //     };

        //     ui.add(egui::Slider::new(&mut self.massa, 1.0..=200.0).text("Massa"));

        //     egui::ComboBox::from_label("Escolha uma apresentação")
        //         .selected_text(&self.medicamento_selecionado)
        //         // Adiciona as opções ao menu
        //         .show_ui(ui, |ui: &mut egui::Ui| {
        //             ui.selectable_value(
        //                 &mut self.medicamento_selecionado,
        //                 "mg/dL".to_string(),
        //                 "mg/dL",
        //             );
        //             ui.selectable_value(
        //                 &mut self.medicamento_selecionado,
        //                 "mg/Kg".to_string(),
        //                 "mg/Kg",
        //             );
        //             ui.selectable_value(&mut self.medicamento_selecionado, "mg".to_string(), "mg");
        //         });

        // for item in &self.apresentacoes {
        //             ui.selectable_value(
        //                 &mut self.medicamento_selecionado, // Referência mutável ao item selecionado
        //                 item.clone(),            // Valor a ser definido se selecionado
        //                 item,                    // Texto exibido
        //             );
        //         }

        //         ui.separator();

        //         ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //             powered_by_egui_and_eframe(ui);
        //             egui::warn_if_debug_build(ui);
        //         });
        //     });
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

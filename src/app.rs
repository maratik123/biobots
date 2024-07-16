use crate::images::Images;
use egui::{Image, TextureHandle, TextureOptions};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq)]
struct BotTextures {
    head: [TextureHandle; 8],
    body: TextureHandle,
}

#[derive(Eq, PartialEq)]
struct ImageTextures {
    apple: TextureHandle,
    organics: TextureHandle,
    rock: TextureHandle,
    bot: BotTextures,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    images: Option<ImageTextures>,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self { ..Self::default() }
    }

    fn images(&mut self, ui: &'_ mut egui::Ui) -> &ImageTextures {
        self.images.get_or_insert_with(|| {
            let images = Images::default();
            ImageTextures {
                apple: ui
                    .ctx()
                    .load_texture("apple", images.apple, TextureOptions::default()),
                organics: ui.ctx().load_texture(
                    "organics",
                    images.organics,
                    TextureOptions::default(),
                ),
                rock: ui
                    .ctx()
                    .load_texture("rock", images.rock, TextureOptions::default()),
                bot: BotTextures {
                    head: images.bot.head.map(|image| {
                        ui.ctx()
                            .load_texture("bot-head", image, TextureOptions::default())
                    }),
                    body: ui.ctx().load_texture(
                        "bot-body",
                        images.bot.body,
                        TextureOptions::default(),
                    ),
                },
            }
        })
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if let Some(storage) = frame.storage_mut() {
                        if ui.button("Save").clicked() {
                            self.save(storage);
                        }
                    }
                    // NOTE: no File->Quit on web pages
                    if !is_web {
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("biobots");

            ui.horizontal(|ui| {
                let images = self.images(ui);
                ui.add(Image::new(&images.organics));
                ui.add(Image::new(&images.apple));
                ui.add(Image::new(&images.rock));
                ui.add(Image::new(&images.bot.body));
                for head in &images.bot.head {
                    ui.add(Image::new(head));
                }
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/maratik123/biobots/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

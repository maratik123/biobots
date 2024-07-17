use crate::bot::Direction;
use crate::images::Images;
use base64::engine::general_purpose;
use core::hash;
use egui::{Color32, Image, Sense, Style, TextureHandle, TextureOptions, Visuals};
use rand::{Fill, Rng, SeedableRng};
use rand_seeder::SipHasher;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::time::Duration;
use web_time::Instant;

#[derive(Eq, PartialEq, Clone)]
struct BotTextures {
    head: [TextureHandle; 8],
    body: TextureHandle,
}

#[derive(Eq, PartialEq, Clone)]
struct ImageTextures {
    apple: TextureHandle,
    organics: TextureHandle,
    rock: TextureHandle,
    bot: BotTextures,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Copy, Clone, Debug)]
enum AutoSaveSec {
    One,
    Five,
    Fifteen,
}

impl AutoSaveSec {
    fn duration(&self) -> Duration {
        match self {
            AutoSaveSec::One => Duration::from_secs(1),
            AutoSaveSec::Five => Duration::from_secs(5),
            AutoSaveSec::Fifteen => Duration::from_secs(15),
        }
    }
}

impl Display for AutoSaveSec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.duration())
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    images: Option<ImageTextures>,
    bot_head: Direction,
    bot_color: Color32,
    lock_initial_seed: bool,
    initial_seed: String,
    rng: Xoshiro256PlusPlus,
    is_dark: bool,
    show_settings: bool,
    auto_save: Option<AutoSaveSec>,
    #[serde(skip)]
    last_save: Instant,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let initial_seed = generate_initial_seed();
        Self {
            images: Default::default(),
            bot_head: Default::default(),
            bot_color: Color32::WHITE,
            lock_initial_seed: false,
            rng: new_seeded_rand(&initial_seed),
            initial_seed,
            is_dark: Default::default(),
            show_settings: Default::default(),
            auto_save: None,
            last_save: Instant::now(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let result: TemplateApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        cc.egui_ctx.set_visuals(if result.is_dark {
            Visuals::dark()
        } else {
            Visuals::light()
        });
        result
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

    fn auto_save(&mut self, frame: &mut eframe::Frame) {
        if let Some(storage) = frame.storage_mut() {
            if let Some(auto_save) = self.auto_save {
                if self.last_save.elapsed() >= auto_save.duration() {
                    self.save(storage);
                }
            }
        }
    }

    fn top_menu(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.auto_save(frame);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();

                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if let Some(storage) = frame.storage_mut() {
                        if ui.button("Save").clicked() {
                            self.save(storage);
                            ui.close_menu();
                        }
                    }
                    // NOTE: no File->Quit on web pages
                    if !is_web {
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            ui.close_menu();
                        }
                    }
                });
                if ui.button("Settings").clicked() {
                    self.show_settings ^= true;
                }
            });
        });
    }

    pub fn global_dark_light_mode_switch(&mut self, ui: &mut egui::Ui) {
        let style: Style = (*ui.ctx().style()).clone();
        let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
        if let Some(visuals) = new_visuals {
            self.is_dark = visuals.dark_mode;
            ui.ctx().set_visuals(visuals);
        }
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("🔧 Settings")
            .auto_sized()
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.lock_initial_seed, "Lock initial seed")
                        .on_hover_text("Lock initial seed on start new simulation");
                    ui.horizontal(|ui| {
                        ui.label("Initial seed:");
                        ui.text_edit_singleline(&mut self.initial_seed);
                        if ui.button("⟳").clicked() {
                            self.initial_seed = generate_initial_seed();
                        }
                    });
                    ui.separator();
                    ui.horizontal_top(|ui| {
                        ui.label("Auto save:");
                        ui.vertical(|ui| {
                            ui.radio_value(&mut self.auto_save, None, "Disabled");
                            let mut radio_value = |auto_save| {
                                ui.radio_value(
                                    &mut self.auto_save,
                                    Some(auto_save),
                                    format!("{}", auto_save),
                                );
                            };
                            radio_value(AutoSaveSec::One);
                            radio_value(AutoSaveSec::Five);
                            radio_value(AutoSaveSec::Fifteen);
                        });
                    });
                });
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        self.last_save = Instant::now();
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.top_menu(ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("biobots");

            ui.horizontal(|ui| {
                let images = self.images(ui).clone();
                ui.add(Image::new(&images.organics));
                ui.add(Image::new(&images.apple));
                ui.add(Image::new(&images.rock));
                ui.add(Image::new(&images.bot.body));
                for head in &images.bot.head {
                    ui.add(Image::new(head));
                }
                let bot_body = ui.add(
                    Image::new(&images.bot.body)
                        .tint(self.bot_color)
                        .sense(Sense::click()),
                );
                if ui.is_rect_visible(bot_body.rect) {
                    Image::new(&images.bot.head[self.bot_head as usize])
                        .paint_at(ui, bot_body.rect);
                }
                if bot_body.clicked() {
                    self.bot_head += Direction::SE;
                    let mut rgb = [0u8; 3];
                    self.rng.fill(&mut rgb);
                    let [r, g, b] = rgb;
                    self.bot_color = Color32::from_rgb(r, g, b);
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                bottom_frame(ui);
                ui.separator();
            });
        });

        self.settings_window(ctx);
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.auto_save.is_some() {
            self.save(storage);
        }
    }
}

fn bottom_frame(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
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
        if cfg!(debug_assertions) {
            ui.separator();
            egui::warn_if_debug_build(ui);
        }
        ui.separator();
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.hyperlink_to("Source code", "https://github.com/maratik123/biobots");
        ui.label(".");
    });
}

fn generate_initial_seed() -> String {
    let mut initial_seed = [0u8; 12];
    rand::thread_rng().fill(&mut initial_seed);
    let mut writer = base64::write::EncoderStringWriter::from_consumer(
        String::with_capacity(base64::encoded_len(initial_seed.len(), false).unwrap()),
        &general_purpose::STANDARD_NO_PAD,
    );
    writer.write_all(&initial_seed).unwrap();
    writer.into_inner()
}

fn new_seeded_rand<H, R>(h: H) -> R
where
    H: hash::Hash,
    R: SeedableRng,
    R::Seed: Fill,
{
    let hasher = SipHasher::from(h);
    let mut hasher_rng = hasher.into_rng();
    let mut seed = R::Seed::default();
    hasher_rng.fill(&mut seed);
    R::from_seed(seed)
}

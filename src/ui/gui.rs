use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use rand::prelude::*;

use crate::resources::{UIState, UISettings};

fn ui_controls(
    mut state: ResMut<UIState>,
    keyboard_input: Res<Input<KeyCode>>,
    settings: Res<UISettings>,
){
    if keyboard_input.just_pressed(settings.skill_tree_key) {
        state.skill_tree_open = !state.skill_tree_open;
    }
}

fn ui_render(
    mut egui_context: ResMut<EguiContext>,
    state: Res<UIState>,
){
    if state.skill_tree_open {
        egui::Window::new("Skill Tree")
            .collapsible(false)
            .resizable(false)
            .show(egui_context.ctx_mut(), content);
    }
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl egui::Frame::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut egui::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn huge_content_painter(ui: &mut egui::Ui) {
    egui::ScrollArea::both()
        .auto_shrink([false; 2])
        .show_viewport(ui, |ui, _viewport| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                // ui.ctx().request_repaint();
                let full_size = 1000.;
                let (_id, rect) = ui.allocate_space(egui::vec2(full_size, full_size));

                let to_screen = egui::emath::RectTransform::from_to(
                    egui::Rect::from_x_y_ranges(0.0..=full_size, 0.0..=full_size),
                    rect,
                );

                let color = egui::Color32::from_additive_luminance(196);
                let size = egui::Vec2::new(100., 100.);
                let builder = SquareBuilder::new(size, color);

                for _n in 0..10 {
                    builder.draw_square(
                        to_screen * egui::Pos2::new(
                            rand::thread_rng().gen_range(0.0..=full_size),
                            rand::thread_rng().gen_range(0.0..=full_size),
                        ),
                        ui,
                    )
                }
            });
        });
}

pub struct SquareBuilder {
    color: egui::Color32,
    size: egui::Vec2,
}

impl SquareBuilder {
    fn new(size: egui::Vec2, color: egui::Color32) -> Self {
        Self {
            color: color,
            size: size,
        }
    }

    fn draw_square(&self, pos: egui::Pos2, ui: &mut egui::Ui) {
        ui.painter().rect(
            egui::Rect::from_center_size(pos, self.size),
            egui::Rounding::none(),
            self.color,
            egui::Stroke::NONE,
        );
    }
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.world.contains_resource::<EguiContext>() {
            app.add_plugin(EguiPlugin);
        }

        app.insert_resource(UIState::default());
        app.insert_resource(UISettings::default());
        app.add_system(ui_controls);
        app.add_system(ui_render);
    }
}

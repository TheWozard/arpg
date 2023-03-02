use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::egui::InnerResponse;

#[derive(Debug, Reflect, Resource)]
pub struct PersistedFrame {
    pub canvas: InnerResponse<()>,
}

impl Default for PersistedFrame {
    fn default() -> Self {
        PersistedFrame {
            canvas: egui::Frame::canvas(ui.style()).show(ui, |ui| {
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
            }),
        }
    }
}

use crate::board::Board;
fn cell_ui(ui: &mut egui::Ui, clicked: &mut bool, colour: (u8, u8, u8), size: f32, interactive: bool) -> egui::Response {
    let desired_size = egui::vec2(size, size);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.style().interact_selectable(&response, *clicked);
    if interactive && response.clicked() {
        *clicked = !*clicked;
        response.mark_changed();
    }
    if ui.is_rect_visible(rect) {
        let color = egui::Color32::from_rgb(colour.0, colour.1, colour.2);
        ui.painter()
            .rect(rect, egui::Rounding::none(), color, visuals.fg_stroke);
    }

    response
}

fn grid_ui(ui: &mut egui::Ui, board: &mut Board, interactive: bool) -> egui::Response {
    egui::Grid::new("Game Grid")
        .min_col_width(0.0)
        .min_row_height(0.0)
        .spacing(egui::Vec2::new(0.0, 0.0))
        .num_columns(board.width)
        .show(ui, |ui| {
            let width = (ui.clip_rect().width() - 2.0 * ui.cursor().left()) / board.width as f32;
            let height = (ui.clip_rect().height() - ui.cursor().top()) / (board.height + 1) as f32;
            let size = if width < height { width } else { height };

            for y in 0..board.height {
                for x in 0..board.width {
                    let mut clicked = false;
                    let colour: (u8, u8, u8) = board.get_colour(x, y);
                    ui.add(|ui: &mut egui::Ui| cell_ui(ui, &mut clicked, colour, size, interactive));
                    if clicked {
                        board.next(x, y);
                    }
                }
                ui.end_row();
            }
        })
        .response
}

pub fn grid(board: &'_ mut Board, interactive: bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| grid_ui(ui, board, interactive)
}

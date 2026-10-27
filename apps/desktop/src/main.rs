use eframe::egui::{self, Color32, Pos2, Sense, Stroke, StrokeKind, Vec2};
use illustrator_document::{Point, Rect};
use illustrator_editor::{Editor, Tool};

#[derive(Clone, Copy)]
enum Drag {
    Create { start: Point, current: Point },
    Move { start: Point, current: Point, original: Rect },
}

#[derive(Default)]
struct IllustratorApp {
    editor: Editor,
    drag: Option<Drag>,
}

impl IllustratorApp {
    fn canvas_ui(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let (response, painter) = ui.allocate_painter(available, Sense::click_and_drag());
        painter.rect_filled(response.rect, 0.0, Color32::from_gray(43));

        // Sprint 01: fixed 1:1 mapping (1 document unit = 1 logical point).
        let (doc_width, doc_height) = (self.editor.document.width, self.editor.document.height);
        let origin = Pos2::new(
            response.rect.center().x - doc_width as f32 * 0.5,
            response.rect.center().y - doc_height as f32 * 0.5,
        );
        let artboard = egui::Rect::from_min_size(
            origin,
            Vec2::new(doc_width as f32, doc_height as f32),
        );
        painter.rect_filled(artboard, 0.0, Color32::WHITE);

        let to_doc = |p: Pos2| Point::new((p.x - origin.x) as f64, (p.y - origin.y) as f64);
        let to_screen = |r: Rect| egui::Rect::from_min_size(
            Pos2::new(origin.x + r.x as f32, origin.y + r.y as f32),
            Vec2::new(r.width as f32, r.height as f32),
        );
        let in_artboard = |p: Point| p.x >= 0.0 && p.y >= 0.0
            && p.x <= doc_width && p.y <= doc_height;

        if response.drag_started_by(egui::PointerButton::Primary) {
            if let Some(pointer) = response.interact_pointer_pos() {
                let p = to_doc(pointer);
                self.drag = None;
                match self.editor.tool {
                    Tool::Rectangle if in_artboard(p) => {
                        self.drag = Some(Drag::Create { start: p, current: p });
                    }
                    Tool::Select => {
                        self.editor.select_at(p);
                        if let Some(id) = self.editor.selected {
                            if let Some(node) = self.editor.document.get(id) {
                                self.drag = Some(Drag::Move {
                                    start: p, current: p, original: node.rect,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if response.dragged_by(egui::PointerButton::Primary) {
            if let (Some(pointer), Some(drag)) = (response.interact_pointer_pos(), self.drag.as_mut()) {
                let p = to_doc(pointer);
                match drag {
                    Drag::Create { current, .. } | Drag::Move { current, .. } => *current = p,
                }
            }
        }

        if response.drag_stopped_by(egui::PointerButton::Primary) {
            if let Some(drag) = self.drag.take() {
                match drag {
                    Drag::Create { start, current } => {
                        self.editor.add_rectangle(Rect::from_points(start, current));
                    }
                    Drag::Move { start, current, original } => {
                        self.editor.move_selected(
                            original,
                            original.translated(current.x - start.x, current.y - start.y),
                        );
                    }
                }
            }
        }

        if response.clicked_by(egui::PointerButton::Primary) && self.editor.tool == Tool::Select {
            if let Some(pointer) = response.interact_pointer_pos() {
                self.editor.select_at(to_doc(pointer));
            }
        }

        // Clip artwork and previews to the artboard.
        let art_painter = painter.with_clip_rect(artboard.intersect(response.rect));
        for node in &self.editor.document.rectangles {
            let mut rect = node.rect;
            if self.editor.selected == Some(node.id) {
                if let Some(Drag::Move { start, current, original }) = self.drag {
                    rect = original.translated(current.x - start.x, current.y - start.y);
                }
            }
            let r = to_screen(rect);
            art_painter.rect_filled(r, 0.0, Color32::from_rgb(82, 151, 226));
            art_painter.rect_stroke(r, 0.0, Stroke::new(1.0_f32, Color32::from_rgb(35, 83, 150)), StrokeKind::Inside);
            if self.editor.selected == Some(node.id) {
                art_painter.rect_stroke(r.expand(2.0), 0.0, Stroke::new(2.0_f32, Color32::from_rgb(255, 130, 40)), StrokeKind::Outside);
            }
        }
        if let Some(Drag::Create { start, current }) = self.drag {
            let r = to_screen(Rect::from_points(start, current));
            art_painter.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(82, 151, 226, 70));
            art_painter.rect_stroke(r, 0.0, Stroke::new(1.5_f32, Color32::from_rgb(35, 83, 150)), StrokeKind::Inside);
        }
    }
}

impl eframe::App for IllustratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (undo, redo) = ctx.input(|input| {
            (input.modifiers.command && input.key_pressed(egui::Key::Z) && !input.modifiers.shift,
             (input.modifiers.command && input.key_pressed(egui::Key::Y))
              || (input.modifiers.command && input.modifiers.shift && input.key_pressed(egui::Key::Z)))
        });
        if self.drag.is_none() {
            if undo { self.editor.undo(); }
            if redo { self.editor.redo(); }
        }

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label(egui::RichText::new("Illustrator RS").strong());
                ui.separator();
                if ui.add_enabled(self.editor.can_undo() && self.drag.is_none(), egui::Button::new("Undo  Ctrl+Z")).clicked() {
                    self.editor.undo();
                }
                if ui.add_enabled(self.editor.can_redo() && self.drag.is_none(), egui::Button::new("Redo  Ctrl+Y")).clicked() {
                    self.editor.redo();
                }
            });
        });

        egui::SidePanel::left("tools").resizable(false).default_width(110.0).show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();
            if ui.selectable_label(self.editor.tool == Tool::Select, "Select (V)").clicked() {
                self.editor.tool = Tool::Select;
                self.drag = None;
            }
            if ui.selectable_label(self.editor.tool == Tool::Rectangle, "Rectangle (M)").clicked() {
                self.editor.tool = Tool::Rectangle;
                self.drag = None;
            }
        });

        egui::SidePanel::right("properties").default_width(235.0).show(ctx, |ui| {
            ui.heading("Layers");
            ui.separator();
            for node in self.editor.document.rectangles.iter().rev() {
                if ui.selectable_label(self.editor.selected == Some(node.id), &node.name).clicked() {
                    self.editor.selected = Some(node.id);
                }
            }
            ui.separator();
            ui.heading("Properties");
            if let Some(node) = self.editor.selected.and_then(|id| self.editor.document.get(id)) {
                ui.label(format!("Name: {}", node.name));
                ui.label(format!("X: {:.1}   Y: {:.1}", node.rect.x, node.rect.y));
                ui.label(format!("W: {:.1}   H: {:.1}", node.rect.width, node.rect.height));
            } else {
                ui.label("No selection");
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label("Rectangle: drag on artboard | Select: click and drag object | Undo: Ctrl+Z | Redo: Ctrl+Y");
        });
        egui::CentralPanel::default().show(ctx, |ui| self.canvas_ui(ui));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Illustrator RS — Sprint 01",
        options,
        Box::new(|_cc| Ok(Box::new(IllustratorApp::default()))),
    )
}

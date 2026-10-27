#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use illustrator_document::{Point, Rect};
use illustrator_editor::Editor;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    width: f64,
    height: f64,
    rectangles: Vec<illustrator_document::RectangleNode>,
    selected: Option<u64>,
    can_undo: bool,
    can_redo: bool,
}

fn snapshot(editor: &Editor) -> Snapshot {
    Snapshot { width: editor.document.width, height: editor.document.height,
        rectangles: editor.document.rectangles.clone(), selected: editor.selected,
        can_undo: editor.can_undo(), can_redo: editor.can_redo() }
}

type AppState = Mutex<Editor>;

#[tauri::command]
fn get_document(state: State<'_, AppState>) -> Result<Snapshot, String> {
    let e = state.lock().map_err(|e| e.to_string())?;
    Ok(snapshot(&e))
}

#[tauri::command]
fn add_rectangle(state: State<'_, AppState>, x: f64, y: f64, width: f64, height: f64) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.add_rectangle(Rect { x, y, width, height });
    Ok(snapshot(&e))
}

#[tauri::command]
fn select_at(state: State<'_, AppState>, x: f64, y: f64) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.select_at(Point { x, y });
    Ok(snapshot(&e))
}

#[tauri::command]
fn select_node(state: State<'_, AppState>, id: Option<u64>) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.select_id(id);
    Ok(snapshot(&e))
}

#[tauri::command]
fn move_selected(state: State<'_, AppState>, before: Rect, after: Rect) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.move_selected(before, after);
    Ok(snapshot(&e))
}

#[tauri::command]
fn undo(state: State<'_, AppState>) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.undo(); Ok(snapshot(&e))
}

#[tauri::command]
fn redo(state: State<'_, AppState>) -> Result<Snapshot, String> {
    let mut e = state.lock().map_err(|e| e.to_string())?;
    e.redo(); Ok(snapshot(&e))
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Editor::default()))
        .invoke_handler(tauri::generate_handler![get_document, add_rectangle, select_at, select_node, move_selected, undo, redo])
        .run(tauri::generate_context!())
        .expect("failed to start Illustrator RS");
}

use illustrator_document::{Document, NodeId, Point, Rect, RectangleNode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool { Select, Rectangle }

#[derive(Clone, Debug)]
pub enum Command {
    Add { node: RectangleNode, index: usize },
    Move { id: NodeId, before: Rect, after: Rect },
}

impl Command {
    fn apply(&self, document: &mut Document) {
        match self {
            Self::Add { node, index } => {
                document.rectangles.insert((*index).min(document.rectangles.len()), node.clone());
            }
            Self::Move { id, after, .. } => {
                if let Some(node) = document.get_mut(*id) { node.rect = *after; }
            }
        }
    }

    fn undo(&self, document: &mut Document) {
        match self {
            Self::Add { node, .. } => {
                document.rectangles.retain(|item| item.id != node.id);
            }
            Self::Move { id, before, .. } => {
                if let Some(node) = document.get_mut(*id) { node.rect = *before; }
            }
        }
    }
}

pub struct Editor {
    pub document: Document,
    pub selected: Option<NodeId>,
    pub tool: Tool,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            document: Document::default(), selected: None, tool: Tool::Select,
            undo_stack: Vec::new(), redo_stack: Vec::new(),
        }
    }
}

impl Editor {
    fn execute(&mut self, command: Command) {
        command.apply(&mut self.document);
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn add_rectangle(&mut self, rect: Rect) -> Option<NodeId> {
        if rect.width < 2.0 || rect.height < 2.0 { return None; }
        let id = self.document.allocate_id();
        let node = RectangleNode { id, name: format!("Rectangle {id}"), rect };
        let index = self.document.rectangles.len();
        self.execute(Command::Add { node, index });
        self.selected = Some(id);
        Some(id)
    }

    pub fn select_at(&mut self, point: Point) {
        self.selected = self.document.hit_test(point);
    }

    pub fn move_selected(&mut self, before: Rect, after: Rect) {
        if before == after { return; }
        if let Some(id) = self.selected {
            if self.document.get(id).is_some_and(|node| node.rect == before) {
                self.execute(Command::Move { id, before, after });
            }
        }
    }

    pub fn undo(&mut self) {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(&mut self.document);
            self.redo_stack.push(command);
            self.selected = self.selected.filter(|id| self.document.get(*id).is_some());
        }
    }

    pub fn redo(&mut self) {
        if let Some(command) = self.redo_stack.pop() {
            command.apply(&mut self.document);
            self.undo_stack.push(command);
        }
    }

    pub fn select_id(&mut self, id: Option<NodeId>) {
        self.selected = id.filter(|id| self.document.get(*id).is_some());
    }

    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_move_undo_redo_and_branch() {
        let mut editor = Editor::default();
        let original = Rect { x: 10.0, y: 20.0, width: 30.0, height: 40.0 };
        let id = editor.add_rectangle(original).unwrap();
        let moved = original.translated(15.0, 5.0);
        editor.move_selected(original, moved);
        assert_eq!(editor.document.get(id).unwrap().rect, moved);
        editor.undo();
        assert_eq!(editor.document.get(id).unwrap().rect, original);
        editor.undo();
        assert!(editor.document.get(id).is_none());
        editor.redo();
        assert_eq!(editor.document.get(id).unwrap().rect, original);
        editor.add_rectangle(original);
        assert!(!editor.can_redo());
    }
}

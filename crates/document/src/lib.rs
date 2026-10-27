#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn from_points(a: Point, b: Point) -> Self {
        Self {
            x: a.x.min(b.x), y: a.y.min(b.y),
            width: (b.x - a.x).abs(), height: (b.y - a.y).abs(),
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x <= self.x + self.width
            && p.y >= self.y && p.y <= self.y + self.height
    }

    pub fn translated(self, dx: f64, dy: f64) -> Self {
        Self { x: self.x + dx, y: self.y + dy, ..self }
    }
}

pub type NodeId = u64;

#[derive(Clone, Debug, PartialEq)]
pub struct RectangleNode {
    pub id: NodeId,
    pub name: String,
    pub rect: Rect,
}

#[derive(Debug)]
pub struct Document {
    pub width: f64,
    pub height: f64,
    pub rectangles: Vec<RectangleNode>,
    next_id: NodeId,
}

impl Default for Document {
    fn default() -> Self {
        Self { width: 800.0, height: 600.0, rectangles: Vec::new(), next_id: 1 }
    }
}

impl Document {
    pub fn allocate_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).expect("node ID exhausted");
        id
    }

    pub fn get(&self, id: NodeId) -> Option<&RectangleNode> {
        self.rectangles.iter().find(|node| node.id == id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut RectangleNode> {
        self.rectangles.iter_mut().find(|node| node.id == id)
    }

    pub fn hit_test(&self, point: Point) -> Option<NodeId> {
        self.rectangles.iter().rev()
            .find(|node| node.rect.contains(point))
            .map(|node| node.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_drag_in_any_direction() {
        let r = Rect::from_points(Point::new(20.0, 30.0), Point::new(5.0, 7.0));
        assert_eq!(r, Rect { x: 5.0, y: 7.0, width: 15.0, height: 23.0 });
    }

    #[test]
    fn hit_test_uses_topmost_rectangle() {
        let mut doc = Document::default();
        doc.rectangles.push(RectangleNode { id: 1, name: "Back".into(), rect: Rect::from_points(Point::new(0.0, 0.0), Point::new(20.0, 20.0)) });
        doc.rectangles.push(RectangleNode { id: 2, name: "Front".into(), rect: Rect::from_points(Point::new(0.0, 0.0), Point::new(20.0, 20.0)) });
        assert_eq!(doc.hit_test(Point::new(10.0, 10.0)), Some(2));
    }
}

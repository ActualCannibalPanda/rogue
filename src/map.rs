use std::default;

const DEBUG_SEED: i32 = 420_69;

pub struct Map {
    root: Node,
}

pub struct MapBuilder {
    seed: i32,
    depth: i32,
}

#[derive(Default)]
pub enum Split {
    Horizontal(f32),
    Vertical(f32),
    #[default]
    None,
}

pub struct Node {
    root: bool,
    split: Split,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            root: false,
            split: Default::default(),
            left: None,
            right: None,
        }
    }
}

impl Node {
    pub fn root() -> Self {
        Self {
            root: true,
            split: Default::default(),
            left: None,
            right: None,
        }
    }

    fn recur(mut self, depth: i32) -> Option<Box<Self>> {
        if depth <= 0 {
            return None;
        }
        self.split = Split::Horizontal(0.5);
        self.left = Node::default().recur(depth - 1);
        self.right = Node::default().recur(depth - 1);
        Some(Box::new(self))
    }

    pub fn build(mut self, depth: i32) -> Self {
        self.split = Split::Horizontal(0.5);
        self.left = Node::default().recur(depth);
        self.right = Node::default().recur(depth);
        self
    }
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            seed: DEBUG_SEED,
            depth: 0,
        }
    }
}

impl MapBuilder {
    pub fn seed(mut self, s: i32) -> MapBuilder {
        self.seed = s;
        self
    }

    pub fn depth(mut self, depth: i32) -> MapBuilder {
        self.depth = depth;
        self
    }

    pub fn build(self) -> Map {
        Map {
            root: Node::root().build(self.depth),
        }
    }
}

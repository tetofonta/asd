use std::cmp::Ordering;

pub struct OpenNode<T> {
    heuristic: f64,
    node: T,
    time: usize,
}

impl<T> Eq for OpenNode<T> {}

impl<T> PartialEq<Self> for OpenNode<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.heuristic == other.heuristic && self.time == other.time; // && self.node == other.node
    }
}

impl<T> PartialOrd<Self> for OpenNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T> Ord for OpenNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.heuristic == other.heuristic { self.time.cmp(&other.time) } else { if self.heuristic < other.heuristic { Ordering::Less } else { Ordering::Greater } };
        // return if self.heuristic == other.heuristic {Ordering::Equal} else {if self.heuristic < other.heuristic {Ordering::Less} else {Ordering::Greater}}
    }
}

impl<T> OpenNode<T> {
    pub fn new(heuristic: f64, data: T, time: usize) -> Self {
        return OpenNode {
            heuristic,
            node: data,
            time,
        };
    }

    pub fn node(&self) -> &T {
        return &self.node;
    }

    pub fn heuristic(&self) -> f64 {
        return self.heuristic;
    }

    pub fn time(&self) -> usize {
        return self.time;
    }
}
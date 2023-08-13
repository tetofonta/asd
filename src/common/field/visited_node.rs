use std::collections::BTreeMap;

type Link<T> = Option<Box<T>>;

pub struct VisitedNode{
    location: (usize, usize),
    timeline: BTreeMap<usize, (f64, Link<VisitedNode>)>
}


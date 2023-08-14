pub mod field;
pub mod visited_node;
pub mod open_node;
mod neighbor_iterator;

pub fn weight(a: &(usize, usize), b: &(usize, usize)) -> f64 {
    // if a == b{
    //     return 0.0;
    // }
    return if (a.0 as i64 - b.0 as i64) == 0 || (a.1 as i64 - b.1 as i64) == 0 { 1.0 } else { f64::sqrt(2.0) };
}

pub fn heuristic(node: &(usize, usize), goal: &(usize, usize)) -> f64 {
    return (node.0 as i64 - goal.0 as i64).pow(2) as f64 + (node.1 as i64 - goal.1 as i64).pow(2) as f64;
}

#[cfg(test)]
mod field_test {
    use crate::field::field::RandomField;
    use crate::noise::perlin::PerlinNoise;

    #[test]
    fn simple_case() {
        let f = RandomField::new(
            PerlinNoise::new(Some(42), Some(3), None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5),
            6,
        );

        let mut it = f.iter_neighbors(3, 1);
        println!("{}", f);
        // .....
        // ....#
        // ...##
        // ...##
        // ....#

        assert_eq!(Some((2, 0)), it.next());
        assert_eq!(Some((3, 0)), it.next());
        assert_eq!(Some((4, 0)), it.next());
        assert_eq!(Some((2, 1)), it.next());
        assert_eq!(Some((3, 1)), it.next());
        assert_eq!(Some((2, 2)), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn underflow() {
        let f = RandomField::new(
            PerlinNoise::new(Some(42), Some(3), None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5),
            6,
        );

        let mut it = f.iter_neighbors(0, 0);
        println!("{}", f);
        // .....
        // ....#
        // ...##
        // ...##
        // ....#

        assert_eq!(Some((0, 0)), it.next());
        assert_eq!(Some((1, 0)), it.next());
        assert_eq!(Some((0, 1)), it.next());
        assert_eq!(Some((1, 1)), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn overflow() {
        let f = RandomField::new(
            PerlinNoise::new(Some(42), Some(3), None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5),
            6,
        );

        let mut it = f.iter_neighbors(4, 4);
        println!("{}", f);
        // .....
        // ....#
        // ...##
        // ...##
        // ....#

        assert_eq!(Some((3, 4)), it.next());
        assert_eq!(None, it.next());
    }
}

// 1976371185, 9

#[cfg(test)]
mod visited_node_test {
    use std::collections::HashMap;
    use crate::agent::agent_manager::AgentManager;
    use crate::field::field::RandomField;
    use crate::field::visited_node::VisitedNode;
    use crate::noise::perlin::PerlinNoise;

    #[test]
    fn simple_case() {
        let mut map = HashMap::new();
        let am = AgentManager::new(vec![]);


        let mut node = VisitedNode::new((0, 0));
        node.set(0, 0.0, None, &am);
        map.insert((0, 0), node);


        let mut new = VisitedNode::new((0, 1));
        new.set(1, 1.0, Some((0, 0)), &am);
        map.insert((0, 1), new);

        println!("{:?}", map)
    }
}
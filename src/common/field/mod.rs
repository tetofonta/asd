pub mod field;
mod neighbor_iterator;

#[cfg(test)]
mod field_test{
    use crate::field::field::Field;
    use crate::noise::perlin::PerlinNoise;

    #[test]
    fn simple_case() {
        let f = Field::new(
            PerlinNoise::new(Some(42), Some(3),None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5)
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
        let f = Field::new(
            PerlinNoise::new(Some(42), Some(3),None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5)
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
        let f = Field::new(
            PerlinNoise::new(Some(42), Some(3),None, None, None, None, Some(5)),
            1976371185,
            9,
            (5, 5)
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
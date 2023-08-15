use std::cmp::Ordering;

pub struct NoiseValue {
    pub value: u32,
    pub cell: usize,
}

impl Eq for NoiseValue {}

impl PartialEq<Self> for NoiseValue {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd<Self> for NoiseValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for NoiseValue {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.value.cmp(&other.value) {
            Ordering::Equal => {
                self.cell.cmp(&other.cell)
            }
            Ordering::Greater => {
                Ordering::Greater
            }
            Ordering::Less => {
                Ordering::Less
            }
        };
    }
}
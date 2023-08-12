use crate::field::field::Field;

pub struct NeighborIterator<'a>{
    field: &'a Field,
    base_point: (usize, usize),
    next: Option<(i64, i64)>
}

impl<'a> NeighborIterator<'a>{
    pub fn new(field: &'a Field, start_point: (usize, usize)) -> Self{
        return NeighborIterator{
            field,
            base_point: start_point,
            next: Some((start_point.0 as i64 - 1, start_point.1 as i64 - 1))
        };
    }

    fn calc_next(point: (i64, i64), base: (usize, usize)) -> Option<(i64, i64)>{
        let mut p = point;
        p = (p.0 + 1, p.1);
        if p.0 > base.0 as i64 + 1{
            p = (base.0 as i64 - 1, p.1 + 1)
        }
        if p.1 > base.1 as i64 + 1{
            return None
        }
        return Some(p);
    }
}

impl<'a> Iterator for NeighborIterator<'a>{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.next;
        if nxt.is_none(){
            return None;
        }

        let mut next_ret = nxt.unwrap();
        let mut next = Some(next_ret);

        while next.is_some(){
            let n = next.take().unwrap();
            self.next = NeighborIterator::calc_next(n, self.base_point);
            next = self.next;

            if next_ret.0 >= 0 && next_ret.1 >= 0 && self.field.exists(next_ret.0 as usize, next_ret.1 as usize) && !self.field.is_obstacle(next_ret.0 as usize, next_ret.1 as usize){
                return Some((next_ret.0 as usize, next_ret.1 as usize));
            } else {
                if self.next.is_none(){
                    return None;
                }
                next_ret = self.next.take().unwrap();
            }
        }

        todo!()
    }
}
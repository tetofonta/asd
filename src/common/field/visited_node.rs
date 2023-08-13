use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use crate::agent::agent::Agent;
use crate::agent::agent_manager::AgentManager;
use crate::field::weight;

type Link<T> = Option<Rc<RefCell<T>>>;

pub struct VisitedNode{
    location: (usize, usize),
    timeline: BTreeMap<usize, (f64, Link<VisitedNode>)>,
    best: (usize, f64)
}

impl VisitedNode{

    pub fn new(location: (usize, usize)) -> Self{
        return VisitedNode{
            location,
            timeline: BTreeMap::new(),
            best: (usize::MAX, f64::MAX)
        }
    }

    fn get_last_entry_before(&self, time: usize) -> Option<(usize, f64, Link<VisitedNode>)>{
        let ret = self.timeline.range(..time).next_back();
        if ret.is_none(){
            return None
        }
        let (t, (w, p)) = ret.unwrap();
        return Some((t.clone(), w.clone(), p.as_ref().cloned()))
    }

    pub fn set(&mut self, time: usize, weight: f64, parent: Link<VisitedNode>, agents: &AgentManager){
        if self.best.1 > weight || (self.best.1 == weight && self.best.0 > time){
            self.best = (time, weight)
        }

        if let Some(old) = self.get_last_entry_before(time){
            let (t, w, p) = old;
            if p.is_none() || parent.is_none(){
                self.timeline.insert(time, (weight, parent));
                return;
            }
            let last_parent = p.as_ref().unwrap();
            let cur_parent = parent.as_ref().unwrap();
            if last_parent.borrow().location == cur_parent.borrow().location && self.weight(time, agents) > weight{
                self.timeline.insert(time, (weight, parent));
            }
        } else {
            self.timeline.insert(time, (weight, parent));
        }
    }

    pub fn weight(&self, time: usize, agents: &AgentManager) -> f64{
        if let Some(last) = self.get_last_entry_before(time){
            let (t, w, p) = last;
            if t == time{
                return w;
            }

            for tt in t..time{
                if !agents.can_stay(self.location, tt){
                    return f64::MAX
                }
            }

            return w + (time as f64 - t as f64) * weight(&self.location, &self.location);
        }

        return f64::MAX;
    }

    pub fn parent(&self, time: usize) -> Link<VisitedNode>{
        if let Some(last) = self.get_last_entry_before(time){
            let (t, _, p) = last;
            if t == time{
                return p;
            }
            return None; // i cannot return my reference as an RC =( I should clone myself and this is bad
        }
        return None;
    }
}


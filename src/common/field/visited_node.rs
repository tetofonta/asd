use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use crate::agent::agent::Agent;
use crate::agent::agent_manager::AgentManager;
use crate::field::weight;

#[derive(Debug)]
pub struct VisitedNode{
    location: (usize, usize),
    timeline: BTreeMap<usize, (f64, Option<(usize, usize)>)>,
    best: (usize, f64)
}

impl VisitedNode{

    pub fn new(location: (usize, usize)) -> Self{
        let cell = VisitedNode{
            location,
            timeline: BTreeMap::new(),
            best: (usize::MAX, f64::MAX)
        };
        return cell
    }

    fn get_last_entry_before(&self, time: usize) -> Option<(usize, f64, Option<(usize, usize)>)>{
        let ret = self.timeline.range(..time+1).next_back();
        if ret.is_none(){
            return None
        }
        let (t, (w, p)) = ret.unwrap();
        return Some((t.clone(), w.clone(), p.as_ref().cloned()))
    }

    pub fn set(&mut self, time: usize, weight: f64, parent: Option<(usize, usize)>, agents: &AgentManager){
        if self.best.1 > weight || (self.best.1 == weight && self.best.0 > time){
            self.best = (time, weight)
        }

        if let Some(old) = self.get_last_entry_before(time){
            let (_, _, p) = old;
            if p.is_none() || parent.is_none(){
                self.timeline.insert(time, (weight, parent));
                return;
            }
            let last_parent = p.as_ref().unwrap();
            let cur_parent = parent.as_ref().unwrap();
            if last_parent == cur_parent{
                if self.weight(time, agents) > weight {
                    self.timeline.insert(time, (weight, parent));
                }
                return;
            }
        }
        self.timeline.insert(time, (weight, parent));
    }

    pub fn weight(&self, time: usize, agents: &AgentManager) -> f64{
        if let Some(last) = self.get_last_entry_before(time){
            let (t, w, _) = last;
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

    pub fn parent(&self, time: usize) -> Option<(usize, usize)>{
        if let Some(last) = self.get_last_entry_before(time){
            let (t, _, p) = last;
            if t == time{
                return p;
            }
            return Some(self.location);
        }
        return None;
    }

    pub fn best_time(&self) -> usize{
        return self.best.0
    }
    pub fn best_weight(&self) -> f64{
        return self.best.1
    }
    pub fn node(&self) -> (usize, usize){
        return self.location;
    }
}


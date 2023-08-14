use crate::agent::agent::Agent;

pub struct AgentManager {
    agents: Vec<Agent>,
}

impl AgentManager {
    pub fn new(agents: Vec<Agent>) -> Self {
        return AgentManager { agents };
    }

    pub fn is_traversable(&self, frm: (usize, usize), to: (usize, usize), time: usize) -> bool {
        for a in &self.agents {
            if a.get_pos(time + 1) == to || (a.get_pos(time + 1) == frm && a.get_pos(time) == to) {
                return false;
            }
        }
        return true;
    }

    pub fn can_stay(&self, pos: (usize, usize), time: usize) -> bool {
        for a in &self.agents {
            if a.get_pos(time) == pos {
                return false;
            }
        }
        return true;
    }
}
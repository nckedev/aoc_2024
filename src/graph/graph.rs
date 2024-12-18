use std::collections::HashMap;
use std::hash::Hash;

pub(crate) struct Graph<Vid, V = (), E = ()> {
    vert: HashMap<Vid, V>,
    adj: HashMap<Vid, Vec<(V, E)>>,
}

impl<Tid, E> Graph<Tid, (), E>
where
    Tid: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            vert: HashMap::new(),
            adj: HashMap::new(),
        }
    }

    pub fn push_vertex(&mut self, id: Tid) {
        self.vert.insert(id, ());
    }

    pub fn push_edge(&mut self, id: Tid, edge: E) {}
}

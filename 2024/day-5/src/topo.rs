use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use miette::Result;


pub type Graph<T> = HashMap<T, HashSet<T>>;

pub struct State<T> {
    depends_on: Graph<T>,
    dependents: Graph<T>,
    no_deps: Vec<T>,
}

pub fn add_edge<T>(graph: &mut Graph<T>, from: T, to: T)
where
    T: Eq + Hash + Copy,
{
    graph
        .entry(from)
        .and_modify(|pointees| {
            pointees.insert(to);
        })
        .or_insert_with(|| {
            let mut set = HashSet::new();
            set.insert(to);
            set
        });
}

impl<T> State<T>
where
    T: Eq + Hash,
{
    pub fn get_dependents(&self, dependency: &T) -> Option<&HashSet<T>> {
        self.dependents.get(dependency)
    }

    pub fn is_resolved(&self) -> bool {
        self.depends_on.is_empty()
    }
}

impl<T> State<T>
where
    T: Copy + Eq + Hash,
{
    pub fn resolve(&mut self, dependent: &T, dependency: &T) {
        if let Some(dependencies) = self.depends_on.get_mut(dependent) {
            dependencies.remove(dependency);
            if dependencies.is_empty() {
                self.no_deps.push(*dependent);
                self.depends_on.remove(dependent);
            }
        }
    }
}

impl From<Graph<i32>> for State<i32> {
    fn from(rules: Graph<i32>) -> Self {
        let mut st = State::default();
        let mut nodes: HashSet<i32> = HashSet::new();

        // Interpret rules as adjacency: prereq -> { dependents }
        for (prereq, dependents) in rules {
            nodes.insert(prereq);
            for dep in dependents {
                nodes.insert(dep);
                // indegree: dep depends on prereq
                add_edge(&mut st.depends_on, dep, prereq);
                // out-adj: prereq has dependent dep
                add_edge(&mut st.dependents, prereq, dep);
            }
        }

        // Seed zero-indegree nodes (no incoming edges in depends_on)
        for n in nodes {
            if st.depends_on.get(&n).map_or(true, |s| s.is_empty()) {
                st.no_deps.push(n);
            }
        }

        st
    }
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self {
            depends_on: HashMap::new(),
            dependents: HashMap::new(),
            no_deps: vec![],
        }
    }
}

pub fn toposort<T, U>(deps: T) -> Result<Vec<U>, String>
where
    U: Eq + Hash + Copy,
    State<U>: From<T>,
{
    let mut res = vec![];
    let mut state = State::from(deps);
    while let Some(node) = state.no_deps.pop() {
        res.push(node);
        if let Some(dependents) = state.get_dependents(&node) {
            for dependent in dependents.clone() {
                state.resolve(&dependent, &node);
            }
        }
    }
    if !state.is_resolved() {
        Err("Detected cyclical graph".into())
    } else {
        Ok(res)
    }
}

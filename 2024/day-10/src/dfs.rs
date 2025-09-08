use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

// Define a Generic Trait that takes some graph and size and return a vector of self
pub trait Neighbors<G, S>: Sized {
    fn get_neighbors(&self, graph: &G, size: &S) -> Vec<Self>;
}

// Define a Generic Trait that takes some goal and return a bool if it matches self
pub trait Goal<V>: Sized {
    fn is_goal(&self, goal: &V) -> bool;
}

// DFS semi-generic implementation on a sized input
pub fn depth_first_search<G, V, O, S>(graph: &G, root: &V, goal: &O, size: &S) -> HashSet<V>
where
    // The generic type V needs two traits, one for neighbors the other for goal validation
    // It also needs to be Hashable and clonable.
    V: Hash + Eq + Neighbors<G, S> + Goal<O> + Clone,
{
    // Set of unique solutions
    let mut unique_solutions: HashSet<V> = HashSet::new();
    // A double ended queue used as a stack by pushing and poping from the same end.
    // Allows for simple BFS conversion by pushing back and poping front
    let mut queue = VecDeque::new();
    queue.push_back(root.to_owned());

    // While there is an element in the queue get the first element
    while let Some(current_vertex) = queue.pop_front() {
        // if the element is a valid goal, add it to the solutions and continue
        if current_vertex.is_goal(goal) {
            unique_solutions.insert(current_vertex);
            continue;
        }

        // For each neighbors of current vertex
        for neighbor in current_vertex.get_neighbors(graph, size).into_iter() {
            // add them to the Stack queue
            queue.push_front(neighbor);
        }
    }
    // Return the unique solutions
    unique_solutions
}

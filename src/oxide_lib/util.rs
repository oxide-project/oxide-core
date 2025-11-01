use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn topo_sort<T: Hash + Eq + Clone + PartialEq>(
    map: HashMap<T, HashSet<T>>,
) -> Option<VecDeque<T>> {
    let mut indegrees = map
        .keys()
        .map(|it| (it.to_owned(), 0))
        .collect::<HashMap<_, _>>();
    for dep in map.values() {
        for d in dep {
            if let Some(v) = indegrees.get_mut(d) {
                *v += 1;
            };
        }
    }
    let mut q = map
        .keys()
        .filter(|it| indegrees[*it] == 0)
        .cloned()
        .collect::<VecDeque<_>>();
    let mut final_order: VecDeque<T> = Default::default();

    while let Some(curr) = q.pop_front() {
        final_order.push_back(curr.clone());
        for dep in &map[&curr] {
            if let Some(v) = indegrees.get_mut(&dep.clone()) {
                *v -= 1;
                if *v == 0 {
                    q.push_back(dep.clone());
                }
            }
        }
    }

    if final_order.len() == map.len() {
        Some(final_order)
    } else {
        None
    }
}

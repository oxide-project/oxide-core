use bebe::Component;
use linkme::distributed_slice;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[distributed_slice]
pub static ALL_BEANS: [ComponentDef] = [..];

#[derive(Debug)]
pub struct ComponentDef {
    pub bean_type: fn() -> TypeId,
    pub name: &'static str,
    pub deps: &'static [fn() -> TypeId], // зависимости
    pub ctor: fn(&Context) -> Box<dyn Any>,
}

#[derive(Debug)]
pub struct Context {
    beans: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            beans: HashMap::new(),
        }
    }

    fn build_dependency_graph(&self) -> HashMap<TypeId, HashSet<TypeId>> {
        let mut graph = HashMap::new();
        for def in ALL_BEANS {
            let key = (def.bean_type)();
            let deps = def.deps.iter().map(|f| f()).collect::<HashSet<_>>();
            graph.insert(key, deps);
        }
        graph
    }

    pub fn init(&mut self) {
        // создаём все компоненты в порядке регистрации
        let graph = self.build_dependency_graph();
        let result = topo_sort(graph).expect("Cycle detected while building dependency graph!");
        for def in result.into_iter().rev() {
            if let Some(def) = ALL_BEANS.iter().find(|d| (d.bean_type)() == def) {
                let instance = (def.ctor)(self);
                self.beans.insert((def.bean_type)(), instance);
            }
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.beans
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.beans
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut())
    }
}

#[derive(Component, Clone)]
struct SmallService;

impl SmallService {
    fn call1(&self) {
        println!("small service executed");
    }
}
#[derive(Component)]
struct BigService {
    #[wired]
    small_service: SmallService,
}

impl BigService {
    fn call2(&self) {
        println!("big service started");
        self.small_service.call1();
        println!("big service finished");
    }
}


fn main() {
    let mut ctx = Context::new();
    ctx.init();
    ctx.get::<BigService>().unwrap().call2();
}

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

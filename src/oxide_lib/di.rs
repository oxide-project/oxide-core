use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use linkme::distributed_slice;
use crate::oxide_lib::util::topo_sort;

#[distributed_slice]
pub static ALL_BEANS: [ComponentDef] = [..];

#[derive(Debug, Clone)]
pub struct ComponentDef {
    pub bean_type: fn() -> TypeId,
    pub name: &'static str,
    pub deps: &'static [fn() -> TypeId], // зависимости
    pub ctor: fn(&Context) -> Box<dyn Any>,
}
impl ComponentDef {
    fn b_type(&self) -> TypeId {
        (self.bean_type)()
    }
}

impl PartialEq for ComponentDef {
    fn eq(&self, other: &Self) -> bool {
        self.b_type() == other.b_type() && self.name == other.name
    }
}
impl Eq for ComponentDef {}

impl Hash for ComponentDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
        self.name.hash(state);
    }
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
        let beans = ALL_BEANS
            .iter()
            .map(|it| (it.b_type(), it.clone()))
            .collect::<HashMap<TypeId, _>>();

        for def in result.into_iter().rev() {
            if let Some(def) = beans.get(&def) {
                let instance = (def.ctor)(self);
                self.beans.insert(def.b_type(), instance);
            }
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.beans
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }

}

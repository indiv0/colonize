use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use util::RustcSerializeWrapper;

pub type Command<'a> = Box<FnMut() + 'a>;

pub trait BindingMap<B> {
    fn get_command_from_binding(&mut self, binding: &B) -> Option<Command>;
}

pub trait BindingStore {
    type Binding;
    type Action;

    fn add_binding(mut self, binding: Self::Binding, action: Self::Action) -> Self;

    fn get_action_from_binding(&self, binding: &Self::Binding) -> Option<&Self::Action>;
}

pub trait UnwrapBindings<S>
    where S: BindingStore,
{
    fn unwrap_bindings(&self) -> S;
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BindingsHashMap<B, A>
    where B: Eq + Hash,
{
    bindings: HashMap<B, A>,
}

impl<B, A> BindingsHashMap<B, A>
    where B: Eq + Hash,
{
    pub fn new() -> Self {
        BindingsHashMap::default()
    }
}

impl<B, A> BindingStore for BindingsHashMap<B, A>
    where B: Eq + Hash,
{
    type Binding = B;
    type Action = A;

    fn add_binding(mut self, binding: B, action: A) -> Self {
        self.bindings.insert(binding, action);
        self
    }

    fn get_action_from_binding(&self, binding: &B) -> Option<&A> {
        self.bindings.get(binding)
    }
}

impl<S, K, V> UnwrapBindings<S> for BindingsHashMap<RustcSerializeWrapper<K>, V>
    where S: BindingStore<Binding=K, Action=V> + Default,
          K: Clone + Eq + Hash,
          V: Clone,
{
    fn unwrap_bindings(&self) -> S {
        let mut new_bindings_map = S::default();

        for (key, value) in &self.bindings {
            new_bindings_map = new_bindings_map.add_binding((*(*key)).clone(), value.clone());
        }

        new_bindings_map
    }
}

impl<B, A> Default for BindingsHashMap<B, A>
    where B: Eq + Hash,
{
    fn default() -> Self {
        BindingsHashMap {
            bindings: HashMap::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BindingsBTreeMap<B, A>
    where B: Ord,
{
    bindings: BTreeMap<B, A>,
}

impl<B, A> BindingsBTreeMap<B, A>
    where B: Ord,
{
    pub fn new() -> Self {
        BindingsBTreeMap::default()
    }
}

impl<B, A> BindingStore for BindingsBTreeMap<B, A>
    where B: Ord,
{
    type Binding = B;
    type Action = A;

    fn add_binding(mut self, binding: B, action: A) -> Self {
        self.bindings.insert(binding, action);
        self
    }

    fn get_action_from_binding(&self, binding: &B) -> Option<&A> {
        self.bindings.get(binding)
    }
}

impl<B, A> Default for BindingsBTreeMap<B, A>
    where B: Ord,
{
    fn default() -> Self {
        BindingsBTreeMap {
            bindings: BTreeMap::new(),
        }
    }
}

impl<S, K, V> UnwrapBindings<S> for BindingsBTreeMap<RustcSerializeWrapper<K>, V>
    where S: BindingStore<Binding=K, Action=V> + Default,
          K: Clone + Ord,
          V: Clone,
{
    fn unwrap_bindings(&self) -> S {
        let mut new_bindings_map = S::default();

        for (key, value) in &self.bindings {
            new_bindings_map = new_bindings_map.add_binding((*(*key)).clone(), value.clone());
        }

        new_bindings_map
    }
}

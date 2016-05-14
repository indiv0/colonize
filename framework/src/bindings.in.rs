#[derive(Clone, Deserialize, Serialize)]
pub struct BindingsHashMap<B, A>
    where B: Eq + Hash,
{
    bindings: HashMap<B, A>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BindingsBTreeMap<B, A>
    where B: Ord,
{
    bindings: BTreeMap<B, A>,
}

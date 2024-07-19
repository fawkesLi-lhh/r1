use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub trait TreeNode<T>
where
    Self: Sized,
    T: Debug,
{
    fn get_key(&self) -> T;
    fn get_parent_key(&self) -> T;
    fn get_sub_item_mut_ref(&mut self) -> &mut Vec<Self>;
    fn take_sub_item(&mut self) -> Vec<Self>;
}

pub fn trans_vec_item_to_tree<T, K>(items: Vec<T>) -> Vec<T>
where
    T: TreeNode<K>,
    K: Clone + Hash + Ord + Debug,
{
    let mut roots = items
        .iter()
        .map(|v| v.get_parent_key())
        .collect::<HashSet<K>>();
    let mut edges: HashMap<K, Vec<K>> = HashMap::new();
    for i in &items {
        roots.remove(&i.get_key());
        let entry = edges.entry(i.get_parent_key()).or_default();
        entry.push(i.get_key());
    }
    let mut queue = roots.into_iter().collect::<Vec<K>>();
    let mut index: usize = 0;
    while index < queue.len() {
        let mut vv = edges.remove(&queue[index]).unwrap_or_default();
        vv.reverse();
        queue.extend(vv);
        index += 1;
    }
    queue.reverse();
    let mut map = items
        .into_iter()
        .map(|v| (v.get_key(), v))
        .collect::<BTreeMap<K, T>>();
    for k in queue {
        if let Some(v) = map.remove(&k) {
            if let Some(p) = map.get_mut(&v.get_parent_key()) {
                p.get_sub_item_mut_ref().push(v);
            } else {
                map.insert(k, v);
            }
        }
    }
    map.into_values().collect::<Vec<T>>()
}

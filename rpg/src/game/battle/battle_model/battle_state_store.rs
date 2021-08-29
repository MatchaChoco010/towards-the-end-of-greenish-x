use std::collections::HashMap;

/// ## Built-in
/// - bi-turn-count ターン数
///
/// ## Item
///
/// ## Player-0
///
/// ## Player-1
///
pub(super) struct BattleStateStore<'a> {
    hash_map: HashMap<&'a str, i32>,
}
impl<'a> BattleStateStore<'a> {
    pub(super) fn new() -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert("bi-turn-count", 0);

        Self { hash_map }
    }

    pub(super) fn add(&mut self, key: &'a str, value: i32) {
        let entry = self.hash_map.entry(key);
        entry.and_modify(|v| *v += value).or_insert(value);
    }

    pub(super) fn get(&self, key: &'a str) -> i32 {
        self.hash_map.get(key).copied().unwrap_or(0)
    }
}

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

pub struct Namespace<T> {
    names: BTreeMap<String, BinaryHeap<NameEntry<T>>>,
    next_id: usize,
}

impl<T> Namespace<T> {
    pub fn new() -> Self {
        Namespace {
            names: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, priority: usize, name: String, value: T) -> usize {
        self.names.entry(name).or_default().push(NameEntry {
            id: self.next_id,
            priority,
            value,
        });

        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn into_lookup_table(self) -> impl Iterator<Item = (String, T)> {
        let mut table = Vec::new();

        // In order to avoid requiring T: Clone, we have to do this loop instead
        // of vec![None; self.next_id] ...
        for _ in 0..self.next_id {
            table.push(None);
        }

        for (key, mut heap) in self.names {
            if let Some(first_entry) = heap.pop() {
                table[first_entry.id] = Some((key.clone(), first_entry.value));
            }

            let mut index = 0;
            while let Some(entry) = heap.pop() {
                table[entry.id] = Some((format!("{}{}", key, index), entry.value));
                index += 1;
            }
        }

        table.into_iter().map(Option::unwrap)
    }
}

struct NameEntry<T> {
    id: usize,
    priority: usize,
    value: T,
}

impl<T> Ord for NameEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T> PartialOrd for NameEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for NameEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for NameEntry<T> {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_namespace() {
        let mut ns = super::Namespace::new();

        // A name without contention.
        ns.insert(0, "foo".to_owned(), 0);

        // A name without contention, with lower priority.
        ns.insert(1, "bar".to_owned(), 1);

        // A name with contention, inserted in order of priority.
        ns.insert(0, "baz".to_owned(), 2);
        ns.insert(1, "baz".to_owned(), 3);
        ns.insert(2, "baz".to_owned(), 4);

        // A name with contention, inserted not in priority-order.
        ns.insert(2, "quux".to_owned(), 5);
        ns.insert(0, "quux".to_owned(), 6);
        ns.insert(1, "quux".to_owned(), 7);

        assert_eq!(
            ns.into_lookup_table().collect::<Vec<_>>(),
            vec![
                ("foo".to_owned(), 0),
                ("bar".to_owned(), 1),
                ("baz".to_owned(), 2),
                ("baz0".to_owned(), 3),
                ("baz1".to_owned(), 4),
                ("quux1".to_owned(), 5),
                ("quux".to_owned(), 6),
                ("quux0".to_owned(), 7),
            ]
        );
    }
}

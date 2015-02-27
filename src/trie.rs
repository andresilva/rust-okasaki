use std::collections::HashMap;

use map::Map;

#[derive(Clone, Debug)]
pub enum PatriciaTrie<T> {
    Tip,
    Node { key: String, value: Option<T>, children: HashMap<char, PatriciaTrie<T>> }
}

use trie::PatriciaTrie::{Tip, Node};

fn longest_common_prefix(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).take_while(|t| t.0 == t.1).count()
}

// taken from: http://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl<T: Clone> Map<String, T> for PatriciaTrie<T> {
    fn empty() -> PatriciaTrie<T> {
        Tip
    }

    fn bind(&self, k: String, v: T) -> PatriciaTrie<T> {
        fn add_children<T: Clone>(t: &PatriciaTrie<T>, k: String, v: T) -> PatriciaTrie<T> {
            match *t {
                Tip => panic!("undefined"),
                Node { ref key, ref value, ref children } =>
                    match children.get(&k.char_at(0)) {
                        Some(n) => n.bind(k, v),
                        None => Node { key: k, value: Some(v), children: hashmap![] }
                    }
            }
        }

        match *self {
            Tip => Node { key: k, value: Some(v), children: hashmap![] },
            Node { ref key, ref value, ref children } => {
                let i = longest_common_prefix(&k, &key);

                // update an already existing key
                if i == k.len() && i == key.len() {
                    Node { key: k, value: Some(v), children: children.clone() }
                }
                // the existing key is contained in the new key
                else if i == key.len() {
                    let k1 = &k[i..];

                    let mut children = children.clone();
                    children.insert(k1.char_at(0), add_children(self, k1.to_string(), v));

                    Node { key: key.clone(), value: value.clone(), children: children }
                }
                // the new key is contained in the existing key
                else if i == k.len() {
                    let k1 = &key[i..];
                    let children = hashmap![
                        k1.char_at(0) => Node { key: k1.to_string(), value: value.clone(), children: children.clone() }];
                    Node { key: k, value: Some(v), children: children }
                }
                // split at longest common prefix
                else {
                    let common = &k[..i];

                    let k1 = &key[i..];
                    let k2 = &k[i..];

                    let children = hashmap![
                        k1.char_at(0) => Node { key: k1.to_string(), value: value.clone(), children: children.clone() },
                        k2.char_at(0) => Node { key: k2.to_string(), value: Some(v), children: hashmap![] }];

                    Node { key: common.to_string(), value: None, children: children }
                }
            }
        }
    }

    fn lookup(&self, k: String) -> T {
        match *self {
            Tip => panic!("lookup on empty tree."),
            Node { ref key, ref value, ref children } => {
                if k == *key {
                    match *value {
                        Some(ref v) => v.clone(),
                        None => panic!("element does not exist"),
                    }
                } else if k.starts_with(key) {
                    match children.get(&k.char_at(key.len())) {
                        Some(t) => t.lookup(k[key.len()..].to_string()),
                        None => panic!("element does not exist"),
                    }
                } else {
                    panic!("element does not exist")
                }
            }
        }
    }
}

#[test]
fn patricia_trie() {
    let t: PatriciaTrie<usize> = Map::empty();
    let t2 = t.bind("test".to_string(), 0)
        .bind("slow".to_string(), 1)
        .bind("water".to_string(), 2)
        .bind("slower".to_string(), 3)
        .bind("tester".to_string(), 4)
        .bind("te".to_string(), 5);

    assert_eq!(t2.lookup("test".to_string()), 0);
    assert_eq!(t2.lookup("slow".to_string()), 1);
    assert_eq!(t2.lookup("water".to_string()), 2);
    assert_eq!(t2.lookup("slower".to_string()), 3);
    assert_eq!(t2.lookup("tester".to_string()), 4);
    assert_eq!(t2.lookup("te".to_string()), 5);
}

use std::sync::Mutex;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;

use futures_intrusive::channel::{shared::StateSender, shared::StateReceiver};

pub use futures_intrusive::channel::StateId;

use valerie::prelude::{Node, execute};
use futures_intrusive::channel::shared::state_broadcast_channel;

pub trait Mutator<V> {
    fn mutate(self, v: &V) -> V;
}

#[derive(Copy, Clone)]
pub enum Ready {
    Ready,
    _Loading,
}

pub trait Relation<K: 'static + Eq + Hash + Copy + Debug, V: 'static + Clone + Send> {
    type Store;

    fn store() -> &'static Mutex<HashMap<K, (V, Ready, StateSender<(V, Ready)>, StateReceiver<(V, Ready)>)>>;

    fn get(id: K) -> Option<V> {
        info!("get: {:?}", id);
        let store = Self::store();
        let lock = store.lock().unwrap();
        (*lock).get(&id).map(|(v, _, _, _)| (*v).clone())
    }

    fn insert(id: K, value: V) {
        let (tx, rx) = state_broadcast_channel();
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        if let Some(_) = lock.insert(id, (value, Ready::Ready, tx, rx)) {
            assert!(false);
        }
    }

    fn mutate(id: K, m: impl Mutator<V>) {
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let (v, tx, rx) = {
            let (v, _, tx, rx) = lock.get(&id).unwrap();
            (v, (*tx).clone(), (*rx).clone())
        };
        let mutated_value = m.mutate(v);
        let new_value = (mutated_value.clone(), Ready::Ready, tx.clone(), rx.clone());
        lock.insert(id, new_value);
        let _res = tx.send((mutated_value, Ready::Ready));
    }

    fn subscribe(id: K) -> StateReceiver<(V, Ready)> {
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (_, _, _, rx) = lock.get(&id).unwrap();
        (*rx).clone()
    }

    fn formatted(id: K, f: fn(v: V) -> String) -> Node {
        info!("formatted: {:?}", &id);

        let store = Self::store();
        let lock = store.lock().unwrap();
        let (v, _, _, rx) = lock.get(&id).expect("id not found to format");
        let elem: Node = f((*v).clone()).into();

        let elem_clone = elem.clone();
        let rx_clone = (*rx).clone();

        let formatter = async move || {
            info!("formatter");
            let mut old = StateId::new();
            while let Some((new, _value)) = rx_clone.receive(old).await {
                info!("formatting");
                let v = Self::get(id).unwrap();
                let s = f(v);
                elem_clone.as_ref().set_node_value(Some(s.as_str()));
                old = new;
            }
        };

        execute(formatter());
        elem
    }
}

pub trait Singleton<V: 'static + Clone, const K: &'static str> {
    type Store;

    fn store() -> &'static Mutex<(V, Ready, StateSender<Ready>, StateReceiver<Ready>)>;

    fn get() -> V {
        let store = Self::store();
        let lock = store.lock().unwrap();
        (*lock).0.clone()
    }

    fn mutate(m: impl Mutator<V>) {
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let (v, tx, rx) = {
            let (v, _, tx, rx) = &(*lock);
            (v, (*tx).clone(), (*rx).clone())
        };
        (*lock) = (m.mutate(v), Ready::Ready, tx.clone(), rx);
        let _res = tx.send(Ready::Ready);
    }

    fn subscribe() -> StateReceiver<Ready> {
        let store = Self::store();
        let lock = store.lock().unwrap();
        (*lock).3.clone()
    }

    fn notify() {
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (_, ready, tx, _) = &(*lock);
        let _res = tx.send(*ready);
    }

    fn formatted(f: fn(v: V) -> String) -> Node {
        info!("formatted");
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (v, _, _, rx) = &(*lock);
        let elem: Node = f(v.clone()).into();

        let elem_clone = elem.clone();
        let rx_clone = (*rx).clone();

        let formatter = async move || {
            info!("formatter");
            let mut old = StateId::new();
            while let Some((new, _value)) = rx_clone.receive(old).await {
                info!("formatting");
                let s = f(Self::get());
                elem_clone.as_ref().set_node_value(Some(s.as_str()));
                old = new;
            }
        };

        execute(formatter());
        elem
    }
}

#[macro_export]
macro_rules! singleton {
    ($ExT:ident, $name:expr, $InT:ty) => {
        pub struct $ExT {}

        impl crate::state::Singleton<$InT, $name> for $ExT {
            type Store = ($InT, crate::state::Ready, futures_intrusive::channel::shared::StateSender<crate::state::Ready>, futures_intrusive::channel::shared::StateReceiver<crate::state::Ready>);

            fn store() -> &'static std::sync::Mutex<($InT, crate::state::Ready, futures_intrusive::channel::shared::StateSender<crate::state::Ready>, futures_intrusive::channel::shared::StateReceiver<crate::state::Ready>)> {
                lazy_static! {
                    static ref STORE: std::sync::Mutex<($InT, crate::state::Ready, futures_intrusive::channel::shared::StateSender<crate::state::Ready>, futures_intrusive::channel::shared::StateReceiver<crate::state::Ready>)> = {
                        let (tx, rx) = futures_intrusive::channel::shared::state_broadcast_channel();
                        std::sync::Mutex::new((<$InT>::default(), crate::state::Ready::Ready, tx, rx))
                    };
                }
                &STORE
            }
        }
    };
}

#[macro_export]
macro_rules! relation {
    ($ExT:ident, $K:ty, $V:ty) => {
        impl crate::state::Relation<$K, $V> for $ExT {
            type Store = HashMap<$V, ($V, crate::state::Ready, futures_intrusive::channel::shared::StateSender<($V, crate::state::Ready)>, futures_intrusive::channel::shared::StateReceiver<($V, crate::state::Ready)>)>;

            fn store() -> &'static std::sync::Mutex<HashMap<$K, ($V, crate::state::Ready, futures_intrusive::channel::shared::StateSender<($V, crate::state::Ready)>, futures_intrusive::channel::shared::StateReceiver<($V, crate::state::Ready)>)>> {
                lazy_static! {
                    static ref STORE: std::sync::Mutex<HashMap<$K, ($V, crate::state::Ready, futures_intrusive::channel::shared::StateSender<($V, crate::state::Ready)>, futures_intrusive::channel::shared::StateReceiver<($V, crate::state::Ready)>)>> = {
                        std::sync::Mutex::new(HashMap::new())
                    };
                }
                &STORE
            }
        }
    };
}
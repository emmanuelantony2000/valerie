use std::sync::Mutex;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;

use futures_intrusive::channel::{shared::StateSender, shared::StateReceiver};

pub use futures_intrusive::channel::StateId;

#[derive(Copy, Clone)]
pub enum Ready {
    Ready,
    _Loading,
}

pub trait Relation<K: 'static + Eq + Hash, V: 'static + Clone> {
    type Store;

    fn store() -> &'static Mutex<HashMap<K, (V, Ready, StateSender<(V, Ready)>, StateReceiver<(V, Ready)>)>>;

    fn get(id: K) -> Option<V> {
        let store = Self::store();
        let lock = store.lock().unwrap();
        (*lock).get(&id).map(|(v, _, _, _)| (*v).clone())
    }

    fn insert(id: K, value: V, tx: StateSender<(V, Ready)>, rx: StateReceiver<(V, Ready)>) {
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        if let Some(_) = lock.insert(id, (value, Ready::Ready, tx, rx)) {
            assert!(false);
        }
    }

    fn update(id: K, value: V) {
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let (tx, rx) = {
            let (_, _, tx, rx) = lock.get(&id).unwrap();
            ((*tx).clone(), (*rx).clone())
        };
        let new_value = (value.clone(), Ready::Ready, tx.clone(), rx.clone());
        lock.insert(id, new_value);
        let _res = tx.send((value, Ready::Ready));
    }

    fn subscribe(id: K) -> StateReceiver<(V, Ready)> {
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (_, _, _, rx) = lock.get(&id).unwrap();
        (*rx).clone()
    }

    fn display(_id: K) -> &'static str {
        "display"
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

    fn set(v: V) {
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let (tx, rx) = {
            let (_, _, tx, rx) = &(*lock);
            ((*tx).clone(), (*rx).clone())
        };
        (*lock) = (v, Ready::Ready, tx.clone(), rx);
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

    fn formatted(_s: fn(v: V) -> String) -> &'static str {
        "formatted"
    }

    fn display() -> &'static str {
        "display"
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
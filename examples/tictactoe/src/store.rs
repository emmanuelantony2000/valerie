use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
pub use futures_intrusive::channel::StateId;

use valerie::prelude::{execute, Node};

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Mutex;

pub trait Mutator<V> {
    fn mutate(&self, v: &V) -> V;
}

#[derive(Copy, Clone)]
pub enum Ready {
    Ready,
    Loading,
    _Editing,
    _Dirty,
    _Saving,
    _Error,
}

pub trait Relation<K: 'static + Copy + Eq + Hash + Debug, V: 'static + Clone + Default + Send> {
    type Store;

    fn store(
    ) -> &'static Mutex<HashMap<K, (V, Ready, StateSender<(V, Ready)>, StateReceiver<(V, Ready)>)>>;

    fn get(id: K, template: &V) -> (V, Ready);

    fn insert(id: K, value: V) {
        info!("insert: {:?}", id);
        let (tx, rx) = state_broadcast_channel();
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let value = (value, Ready::_Dirty, tx, rx);
        let res = lock.insert(id, value);
        assert!(res.is_none());
    }

    fn mutate(id: K, m: &impl Mutator<V>);

    fn subscribe(id: K) -> StateReceiver<(V, Ready)> {
        info!("subscribe: {:?}", id);
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (_, _, _, rx) = lock.get(&id).unwrap();
        (*rx).clone()
    }

    fn notify(id: K) {
        info!("notify: {:?}", id);
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (v, r, tx, _) = lock.get(&id).unwrap();
        let _res = tx.send((v.clone(), *r));
    }

    // fn formatter(id: K, f: fn(v: V, r: Ready) -> Node) -> Formatter<V> {
    //     let (v, r) = Self::get(id, &V::default());
    //     let rx = Self::subscribe(id);
    //     Formatter::new(v, r, rx, f)
    // }

    fn formatted(id: K, f: fn(v: V, r: Ready) -> String) -> Node {
        info!("formatted: {:?}", id);

        let (v, r) = Self::get(id, &V::default());
        let elem: Node = f(v, r).into();
        let elem_clone = elem.clone();
        let rx_clone = Self::subscribe(id).clone();

        let formatter = async move || {
            info!("formatter");
            let mut old = StateId::new();
            while let Some((new, _value)) = rx_clone.receive(old).await {
                info!("formatting");
                let (v, r) = Self::get(id, &V::default());
                let s = f(v, r);
                elem_clone.as_ref().set_node_value(Some(s.as_str()));
                old = new;
            }
        };

        execute(formatter());
        elem
    }
}

pub trait Local<K: 'static + Copy + Eq + Hash + Debug, V: 'static + Clone + Default + Send>:
    Relation<K, V>
{
    fn get(id: K, template: &V) -> (V, Ready) {
        info!("get: {:?}", id);
        {
            let store = Self::store();
            let lock = store.lock().unwrap();
            let res = (*lock).get(&id);
            if let Some((v, r, _, _)) = res {
                return ((*v).clone(), *r);
            };
        }
        let result = V::clone(template);
        Self::insert(id, result.clone());
        (result, Ready::_Dirty)
    }

    fn mutate(id: K, m: &impl Mutator<V>) {
        info!("mutate: {:?}", id);
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
}

pub trait Singleton<V: 'static + Clone + Default> {
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
    ($ExT:ident, $InT:ty) => {
        pub struct $ExT {}

        impl crate::store::Singleton<$InT> for $ExT {
            type Store = (
                $InT,
                crate::store::Ready,
                futures_intrusive::channel::shared::StateSender<crate::store::Ready>,
                futures_intrusive::channel::shared::StateReceiver<crate::store::Ready>,
            );

            fn store() -> &'static std::sync::Mutex<(
                $InT,
                crate::store::Ready,
                futures_intrusive::channel::shared::StateSender<crate::store::Ready>,
                futures_intrusive::channel::shared::StateReceiver<crate::store::Ready>,
            )> {
                lazy_static! {
                    static ref STORE: std::sync::Mutex<(
                        $InT,
                        crate::store::Ready,
                        futures_intrusive::channel::shared::StateSender<crate::store::Ready>,
                        futures_intrusive::channel::shared::StateReceiver<crate::store::Ready>
                    )> = {
                        let (tx, rx) =
                            futures_intrusive::channel::shared::state_broadcast_channel();
                        std::sync::Mutex::new((
                            <$InT>::default(),
                            crate::store::Ready::Ready,
                            tx,
                            rx,
                        ))
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
        impl crate::store::Relation<$K, $V> for $ExT {
            type Store = HashMap<
                $V,
                (
                    $V,
                    crate::store::Ready,
                    futures_intrusive::channel::shared::StateSender<($V, crate::store::Ready)>,
                    futures_intrusive::channel::shared::StateReceiver<($V, crate::store::Ready)>,
                ),
            >;

            fn store() -> &'static std::sync::Mutex<
                HashMap<
                    $K,
                    (
                        $V,
                        crate::store::Ready,
                        futures_intrusive::channel::shared::StateSender<($V, crate::store::Ready)>,
                        futures_intrusive::channel::shared::StateReceiver<(
                            $V,
                            crate::store::Ready,
                        )>,
                    ),
                >,
            > {
                lazy_static! {
                    static ref STORE: std::sync::Mutex<
                        HashMap<
                            $K,
                            (
                                $V,
                                crate::store::Ready,
                                futures_intrusive::channel::shared::StateSender<(
                                    $V,
                                    crate::store::Ready
                                )>,
                                futures_intrusive::channel::shared::StateReceiver<(
                                    $V,
                                    crate::store::Ready
                                )>
                            ),
                        >,
                    > = { std::sync::Mutex::new(HashMap::new()) };
                }
                &STORE
            }
        }
    };
}

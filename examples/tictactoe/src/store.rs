use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
pub use futures_intrusive::channel::StateId;

use valerie::prelude::{execute, Node};

pub trait Mutator<V> {
    fn mutate(&self, v: &V) -> V;
}

#[derive(Copy, Clone)]
pub enum Ready {
    Ready,
    _Loading,
    _Editing,
    _Dirty,
    _Saving,
    _Error,
}

pub trait Watch<K, V: Clone> {
    fn subscribe(_id: K) -> StateReceiver<(V, Ready)> {
        unimplemented!()
    }
    fn notify(_id: K) {}
}

pub trait Relation<K: Eq + Hash + Copy, V: Clone + Default> {
    fn get(_id: K, _template: &V) -> (V, Ready) {
        unimplemented!()
    }
    fn new(_id: K) {}
    fn mutate(_id: K, _mutation: impl Mutator<V>) {}
}

pub trait Format<K: 'static + Copy + Eq + Hash, V: 'static + Clone + Default>:
    Watch<K, V> + Relation<K, V>
{
    fn formatted(id: K, f: fn(v: V, r: Ready) -> String) -> Node {
        info!("formatted");

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

pub trait Local<K: 'static + Copy + Eq + Hash, V: 'static + Clone + Default + Send>:
    Watch<K, V> + Relation<K, V> + Format<K, V>
{
    type Store;

    fn store(
    ) -> &'static Mutex<HashMap<K, (V, Ready, StateSender<(V, Ready)>, StateReceiver<(V, Ready)>)>>;

    fn get(id: K, template: &V) -> (V, Ready) {
        let store = Self::store();
        let lock = store.lock().unwrap();
        let res = (*lock).get(&id);
        match res {
            Some((v, r, _, _)) => ((*v).clone(), *r),
            None => (V::clone(template), Ready::_Loading),
        }
    }

    fn insert(id: K, value: V) {
        let (tx, rx) = state_broadcast_channel();
        let store = Self::store();
        let mut lock = store.lock().unwrap();
        let value = (value, Ready::Ready, tx, rx);
        let res = lock.insert(id, value);
        assert!(res.is_none());
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

    fn notify(id: K) {
        let store = Self::store();
        let lock = store.lock().unwrap();
        let (v, r, tx, _) = lock.get(&id).unwrap();
        let _res = tx.send((v.clone(), *r));
    }

    fn formatted(id: K, f: fn(v: V, r: Ready) -> String) -> Node {
        info!("formatted");

        let store = Self::store();
        let lock = store.lock().unwrap();
        let (v, r, _, rx) = lock.get(&id).expect("id not found to format");
        let elem: Node = f((*v).clone(), *r).into();

        let elem_clone = elem.clone();
        let rx_clone = (*rx).clone();

        let formatter = async move || {
            info!("formatter");
            let mut old = StateId::new();
            while let Some((new, _value)) = rx_clone.receive(old).await {
                info!("formatting");
                let (v, r): (V, Ready) = <Self as Local<K, V>>::get(id, &V::default());
                let s = f(v, r);
                elem_clone.as_ref().set_node_value(Some(s.as_str()));
                old = new;
            }
        };

        execute(formatter());
        elem
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
    ($ExT:ident, $K:ty, $V:ty, $F:ty) => {
        impl crate::store::Local<$K, $V> for $ExT {
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

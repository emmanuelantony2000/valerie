use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
pub use futures_intrusive::channel::StateId;

pub use hashbrown::HashMap;
pub use parking_lot::Mutex;

use crate::prelude::{execute, Node};

use alloc::string::String;
use core::cmp::Eq;
use core::fmt::Debug;
use core::hash::Hash;

/// a serialisable object which can modify an instance  
pub trait Mutator<V> {
    /// apply the mutation to an instance of the type
    fn mutate(&self, v: &V) -> V;
}

/// The state in the lifecycle of a monitored object
#[derive(Copy, Clone)]
pub enum Ready {
    /// Object is latest
    Ready,
    /// Object is being loaded from server
    Loading,
    /// Object is in edit form state
    _Editing,
    /// Object has been changed since last load from server
    _Dirty,
    /// Object is being saved to server
    _Saving,
    /// Previous attempt to communicate with server failed
    _Error,
}

/// Object stored in static in-memory database keyed by an id and can be presented by a formatter
pub trait Relation<K: 'static + Copy + Eq + Hash + Debug, V: 'static + Clone + Default + Send> {
    /// type of the underlying in-memory database
    type Store;

    /// get a reference to the database
    fn store(
    ) -> &'static Mutex<HashMap<K, (V, Ready, StateSender<(V, Ready)>, StateReceiver<(V, Ready)>)>>;

    /// get the current state of the value associated with id
    fn get(id: K, template: &V) -> (V, Ready);

    /// insert a new key-value pair
    fn insert(id: K, value: V) {
        let (tx, rx) = state_broadcast_channel();
        let store = Self::store();
        let mut lock = store.lock();
        let value = (value, Ready::_Dirty, tx, rx);
        let res = lock.insert(id, value);
        assert!(res.is_none());
    }

    /// apply a mutation to the value associated with an id and notify
    fn mutate(id: K, m: &impl Mutator<V>);

    /// get a receiver channel for the object associated with an id
    fn subscribe(id: K) -> StateReceiver<(V, Ready)> {
        let store = Self::store();
        let lock = store.lock();
        let (_, _, _, rx) = lock.get(&id).unwrap();
        (*rx).clone()
    }

    /// notify receivers of the object associated with an id
    fn notify(id: K) {
        let store = Self::store();
        let lock = store.lock();
        let (v, r, tx, _) = lock.get(&id).unwrap();
        let _res = tx.send((v.clone(), *r));
    }

    // fn formatter(id: K, f: fn(v: V, r: Ready) -> Node) -> Formatter<V> {
    //     let (v, r) = Self::get(id, &V::default());
    //     let rx = Self::subscribe(id);
    //     Formatter::new(v, r, rx, f)
    // }

    /// a node which will display the object associated with an id, using a closure
    fn formatted(id: K, f: fn(v: V, r: Ready) -> String) -> Node {
        let (v, r) = Self::get(id, &V::default());
        let elem: Node = f(v, r).into();
        let rx = Self::subscribe(id).clone();
        let elem_clone = elem.clone();

        let formatter = async move || {
            let mut old = StateId::new();
            while let Some((new, _value)) = rx.receive(old).await {
                let (v, r) = Self::get(id, &V::default());
                let s = f(v, r);
                elem_clone.as_ref().set_node_value(Some(&s));
                old = new;
            }
        };

        execute(formatter());
        elem
    }
}

/// A relation with local storage and without a backing service
pub trait Local<K: 'static + Copy + Eq + Hash + Debug, V: 'static + Clone + Default + Send>:
    Relation<K, V>
{
    /// get the current state of the value associated with id from the local store
    fn get(id: K, template: &V) -> (V, Ready) {
        {
            let store = Self::store();
            let lock = store.lock();
            let res = (*lock).get(&id);
            if let Some((v, r, _, _)) = res {
                return ((*v).clone(), *r);
            };
        }
        let result = V::clone(template);
        Self::insert(id, result.clone());
        (result, Ready::_Dirty)
    }

    /// apply a mutation to the value associated with an id and notify
    fn mutate(id: K, m: &impl Mutator<V>) {
        let store = Self::store();
        let mut lock = store.lock();
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

/// define type which enables an object to be rendered by a formatter
pub trait Singleton<V: 'static + Clone + Default> {
    /// type of the in-memory store
    type Store;

    /// get a reference to the in-memory store
    fn store() -> &'static Mutex<(V, Ready, StateSender<Ready>, StateReceiver<Ready>)>;

    /// get the current state of the singleton value from the local store
    fn get() -> V {
        let store = Self::store();
        let lock = store.lock();
        (*lock).0.clone()
    }

    /// apply a mutation to the singleton value and notify
    fn mutate(m: impl Mutator<V>) {
        let store = Self::store();
        let mut lock = store.lock();
        let (v, tx, rx) = {
            let (v, _, tx, rx) = &(*lock);
            (v, (*tx).clone(), (*rx).clone())
        };
        (*lock) = (m.mutate(v), Ready::Ready, tx.clone(), rx);
        let _res = tx.send(Ready::Ready);
    }

    /// get a receiver channel for the singleton object
    fn subscribe() -> StateReceiver<Ready> {
        let store = Self::store();
        let lock = store.lock();
        (*lock).3.clone()
    }

    /// notify receivers of the singleton object
    fn notify() {
        let store = Self::store();
        let lock = store.lock();
        let (_, ready, tx, _) = &(*lock);
        let _res = tx.send(*ready);
    }

    /// a node which will display the singleton object, using a closure
    fn formatted(f: fn(v: V) -> String) -> Node {
        let store = Self::store();
        let lock = store.lock();
        let (v, _, _, rx) = &(*lock);
        let elem: Node = f(v.clone()).into();

        let elem_clone = elem.clone();
        let rx_clone = (*rx).clone();

        let formatter = async move || {
            let mut old = StateId::new();
            while let Some((new, _value)) = rx_clone.receive(old).await {
                let v = Self::get();
                let s = f(v);
                elem_clone.as_ref().set_node_value(Some(&s));
                old = new;
            }
        };

        execute(formatter());
        elem
    }
}

#[macro_export]
/// Define a singleton class.
macro_rules! singleton {
    ($ExT:ident, $InT:ty) => {
        pub struct $ExT {}

        impl valerie::store::Singleton<$InT> for $ExT {
            type Store = (
                $InT,
                valerie::store::Ready,
                futures_intrusive::channel::shared::StateSender<valerie::store::Ready>,
                futures_intrusive::channel::shared::StateReceiver<valerie::store::Ready>,
            );

            fn store() -> &'static valerie::store::Mutex<(
                $InT,
                valerie::store::Ready,
                futures_intrusive::channel::shared::StateSender<valerie::store::Ready>,
                futures_intrusive::channel::shared::StateReceiver<valerie::store::Ready>,
            )> {
                lazy_static! {
                    static ref STORE: valerie::store::Mutex<(
                        $InT,
                        valerie::store::Ready,
                        futures_intrusive::channel::shared::StateSender<valerie::store::Ready>,
                        futures_intrusive::channel::shared::StateReceiver<valerie::store::Ready>
                    )> = {
                        let (tx, rx) =
                            futures_intrusive::channel::shared::state_broadcast_channel();
                        valerie::store::Mutex::new((
                            <$InT>::default(),
                            valerie::store::Ready::Ready,
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
/// Define a class as a relation with update channels
macro_rules! relation {
    ($ExT:ident, $K:ty, $V:ty) => {
        impl valerie::store::Relation<$K, $V> for $ExT {
            type Store = valerie::store::HashMap<
                $V,
                (
                    $V,
                    valerie::store::Ready,
                    futures_intrusive::channel::shared::StateSender<($V, valerie::store::Ready)>,
                    futures_intrusive::channel::shared::StateReceiver<($V, valerie::store::Ready)>,
                ),
            >;

            fn store() -> &'static valerie::store::Mutex<
                valerie::store::HashMap<
                    $K,
                    (
                        $V,
                        valerie::store::Ready,
                        futures_intrusive::channel::shared::StateSender<(
                            $V,
                            valerie::store::Ready,
                        )>,
                        futures_intrusive::channel::shared::StateReceiver<(
                            $V,
                            valerie::store::Ready,
                        )>,
                    ),
                >,
            > {
                lazy_static! {
                    static ref STORE: valerie::store::Mutex<
                        valerie::store::HashMap<
                            $K,
                            (
                                $V,
                                valerie::store::Ready,
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
                    > = { valerie::store::Mutex::new(valerie::store::HashMap::new()) };
                }
                &STORE
            }
        }
    };
}

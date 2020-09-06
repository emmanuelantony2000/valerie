use crate::prelude::execute;
use crate::store::{Mutator, Ready, Relation};

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::Debug;
use core::hash::Hash;

/// Type is `relation` and can be fetched from remote service
pub trait Remote<
    'de,
    K: 'static + Copy + Eq + Hash + Debug + Serialize + Deserialize<'de>,
    V: 'static + Clone + Default + Send + Deserialize<'de>,
>: Relation<K, V>
{
    /// endpoint url for fetch
    fn endpoint(id: K) -> String;

    /// modify the local database with the results of a fetch
    fn update(id: K, value: V) {
        let store = Self::store();
        let mut lock = store.lock();
        let (tx, rx) = {
            let (_, _, tx, rx) = lock.get(&id).unwrap();
            ((*tx).clone(), (*rx).clone())
        };
        let new_value = (value.clone(), Ready::Ready, tx.clone(), rx.clone());
        lock.insert(id, new_value);
        let _res = tx.send((value, Ready::Ready));
    }

    /// spawn remote request to fetch latest view of object from service
    fn fetch(id: K) {
        let callback = move |_js_val: JsValue| {
            let then = Closure::wrap(Box::new(move || {
                let (value, _) = Self::get(id, &V::default());
                Self::update(id, value);
            }) as Box<dyn Fn()>);
            let _res = window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    then.as_ref().unchecked_ref(),
                    1000,
                );
            then.forget();
        };
        let window: web_sys::Window = window().unwrap();
        let url = Self::endpoint(id);
        let p = window.fetch_with_str(&url);
        let _res = execute(do_fetch(p, Box::new(callback)));
    }

    /// propagate a mutation to the service
    fn relay(id: K, _m: &impl Mutator<V>) {
        let callback = move |_js_val: JsValue| {
            let then = Closure::wrap(Box::new(move || {
                let (value, _) = Self::get(id, &V::default());
                Self::update(id, value);
            }) as Box<dyn Fn()>);
            let _res = window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    then.as_ref().unchecked_ref(),
                    1000,
                );
            then.forget();
        };
        let window: web_sys::Window = window().unwrap();
        let url = Self::endpoint(id);
        let p = window.fetch_with_str(&url);
        let _res = execute(do_fetch(p, Box::new(callback)));
    }
}

#[allow(dead_code)]
async fn do_fetch(p: Promise, callback: Box<dyn Fn(JsValue)>) {
    let f = JsFuture::from(p).await.ok();
    callback(f.unwrap());
}

#[macro_export]
/// Implement fetch from remote for type
macro_rules! remote {
    ($ExT:ident, $K:ty, $V:ty, $e:expr) => {
        impl<'de> valerie::fetch::Remote<'de, $K, $V> for $ExT {
            fn endpoint(_id: $K) -> String {
                $e.to_string()
            }
        }
        impl<'de> valerie::store::Relation<$K, $V> for $ExT {
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
                                    valerie::store::Ready
                                )>,
                                futures_intrusive::channel::shared::StateReceiver<(
                                    $V,
                                    valerie::store::Ready
                                )>
                            ),
                        >,
                    > = { valerie::store::Mutex::new(valerie::store::HashMap::new()) };
                }
                &STORE
            }

            fn get(id: $K, template: &$V) -> ($V, valerie::store::Ready) {
                {
                    let store = Self::store();
                    let lock = store.lock();
                    let res = (*lock).get(&id);
                    if let Some((v, r, _, _)) = res {
                        return ((*v).clone(), *r);
                    };
                }
                let result = <$V>::clone(template);
                Self::insert(id, result.clone());
                <Self as valerie::fetch::Remote<'de, $K, $V>>::fetch(id);
                (result, valerie::store::Ready::Loading)
            }

            fn mutate(id: $K, m: &impl Mutator<$V>) {
                let store = Self::store();
                let mut lock = store.lock();
                let (v, tx, rx) = {
                    let (v, _, tx, rx) = lock.get(&id).unwrap();
                    (v, (*tx).clone(), (*rx).clone())
                };
                let mutated_value = m.mutate(v);
                let new_value = (
                    mutated_value.clone(),
                    valerie::store::Ready::_Saving,
                    tx.clone(),
                    rx.clone(),
                );
                lock.insert(id, new_value);
                let _res = tx.send((mutated_value, valerie::store::Ready::_Saving));
                <Self as valerie::fetch::Remote<'de, $K, $V>>::relay(id, m);
            }
        }
    };
}

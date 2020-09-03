use crate::store::{Mutator, Ready, Relation};

use valerie::prelude::execute;

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use std::fmt::Debug;
use std::hash::Hash;

pub trait Remote<
    'de,
    K: 'static + Copy + Eq + Hash + Debug + Serialize + Deserialize<'de>,
    V: 'static + Clone + Default + Send + Deserialize<'de>,
>: Relation<K, V>
{
    fn endpoint(id: K) -> String;

    fn update(id: K, value: V) {
        info!("remote::update: {:?}", id);
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

    fn fetch(id: K) {
        info!("remote::fetch: {:?}", id);
        let callback = move |js_val: JsValue| {
            info!("fetch callback: {:?}", js_val);
            let then = Closure::wrap(Box::new(move || {
                info!("fetch delay complete");
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
        info!("through");
    }

    fn relay(id: K, _m: &impl Mutator<V>) {
        info!("remote::relay: {:?}", id);
        info!("fetch::mutate");
        let callback = move |js_val: JsValue| {
            info!("relay callback {:?}", js_val);
            let then = Closure::wrap(Box::new(move || {
                info!("relay delay complete");
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
        info!("through");
    }
}

#[allow(dead_code)]
async fn do_fetch(p: Promise, callback: Box<dyn Fn(JsValue)>) {
    info!("do_fetch");
    let f = JsFuture::from(p).await.ok();
    info!("response!: {:?}", f);
    callback(f.unwrap());
}

#[macro_export]
macro_rules! remote {
    ($ExT:ident, $K:ty, $V:ty, $e:expr) => {
        impl<'de> crate::fetch::Remote<'de, $K, $V> for $ExT {
            fn endpoint(_id: $K) -> String {
                $e.to_string()
            }
        }
        impl<'de> crate::store::Relation<$K, $V> for $ExT {
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

            fn get(id: $K, template: &$V) -> ($V, crate::store::Ready) {
                info!("remote::get: {:?}", id);
                {
                    let store = Self::store();
                    let lock = store.lock().unwrap();
                    let res = (*lock).get(&id);
                    if let Some((v, r, _, _)) = res {
                        return ((*v).clone(), *r);
                    };
                }
                let result = <$V>::clone(template);
                Self::insert(id, result.clone());
                <Self as crate::fetch::Remote<'de, $K, $V>>::fetch(id);
                (result, crate::store::Ready::Loading)
            }

            fn mutate(id: $K, m: &impl Mutator<$V>) {
                info!("remote::mutate: {:?}", id);
                let store = Self::store();
                let mut lock = store.lock().unwrap();
                let (v, tx, rx) = {
                    let (v, _, tx, rx) = lock.get(&id).unwrap();
                    (v, (*tx).clone(), (*rx).clone())
                };
                let mutated_value = m.mutate(v);
                let new_value = (
                    mutated_value.clone(),
                    crate::store::Ready::_Saving,
                    tx.clone(),
                    rx.clone(),
                );
                lock.insert(id, new_value);
                let _res = tx.send((mutated_value, crate::store::Ready::_Saving));
                <Self as crate::fetch::Remote<'de, $K, $V>>::relay(id, m);
            }
        }
    };
}

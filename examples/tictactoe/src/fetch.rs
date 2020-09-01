use crate::store::{Local, Mutator, Ready};

use valerie::prelude::execute;

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use std::hash::Hash;

pub trait Update<K: 'static + Copy + Eq + Hash, V: 'static + Clone + Default + Send>:
    Local<K, V>
{
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
}

pub trait Fetch<
    'de,
    K: 'static + Copy + Eq + Hash + Serialize + Deserialize<'de>,
    V: 'static + Clone + Default + Send + Deserialize<'de>,
>: Update<K, V>
{
    fn fetch(id: K) {
        #[allow(dead_code)]
        async fn do_fetch(p: Promise, callback: Box<dyn Fn(JsValue)>) {
            info!("do_fetch");
            let f = JsFuture::from(p).await.ok();
            info!("response!: {:?}", f);
            callback(f.unwrap());
        }

        let window: web_sys::Window = window().unwrap();
        let callback = move |js_val: JsValue| {
            info!("{:?}", js_val);
            let value = V::default();
            Self::update(id, value)
        };
        let p = window.fetch_with_str("https://jsonplaceholder.typicode.com/todos/1");
        let _res = execute(do_fetch(p, Box::new(callback)));
        info!("through");
    }
    fn relay(id: K, _m: impl Mutator<V>) {
        #[allow(dead_code)]
        async fn do_fetch(p: Promise, callback: Box<dyn Fn(JsValue)>) {
            info!("do_fetch");
            let f = JsFuture::from(p).await.ok();
            info!("response!: {:?}", f);
            callback(f.unwrap());
        }

        let window: web_sys::Window = window().unwrap();
        info!("fetch::mutate");
        let callback = move |js_val: JsValue| {
            info!("{:?}", js_val);
            let value: V = V::default();
            Self::update(id, value);
        };
        let p = window.fetch_with_str("https://jsonplaceholder.typicode.com/todos/1");
        let _res = execute(do_fetch(p, Box::new(callback)));
        info!("through");
    }
}

pub trait Remote<
    'de,
    K: 'static + Copy + Eq + Hash + Deserialize<'de> + Serialize,
    V: 'static + Clone + Default + Send + Deserialize<'de>,
>: Fetch<'de, K, V>
{
}

#[macro_export]
macro_rules! remote {
    ($ExT:ident, $K:ty, $V:ty) => {
        impl crate::store::Watch<$K, $V> for $ExT {}
        impl crate::store::Format<$K, $V> for $ExT {}
        impl crate::store::Relation<$K, $V> for $ExT {}
        impl crate::store::Local<$K, $V> for $ExT {
            type Store = HashMap<
                $K,
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
        impl crate::fetch::Update<$K, $V> for $ExT {}
        impl<'de> crate::fetch::Fetch<'de, $K, $V> for $ExT {}
        impl<'de> crate::fetch::Remote<'de, $K, $V> for $ExT {}
    };
}

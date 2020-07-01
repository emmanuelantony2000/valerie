// use crate::channel::Channel;
// use crate::component::Component;

// use alloc::boxed::Box;
// use alloc::string::String;
// use alloc::sync::Arc;
// use core::ops::Deref;
// use futures_intrusive::channel::shared::StateReceiver;
// use futures_intrusive::channel::StateId;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

// pub struct Event {
//     event: &'static str,
//     callback: Box<dyn FnMut()>,
// }

// impl Event {
//     pub fn new(event: &'static str, callback: Box<dyn FnMut()>) -> Self {
//         Self { event, callback }
//     }
// }

// pub enum FunctionType {
//     Element(&'static str),
//     Text(String),
// }

// #[derive(Clone)]
// pub enum FunctionStore {
//     Element(Element),
//     Text(Text),
// }

// #[derive(Clone)]
// pub struct Function {
//     node: FunctionStore,
// }

// impl Function {
//     pub fn new(function_type: FunctionType) -> Self {
//         match function_type {
//             FunctionType::Element(x) => Self {
//                 node: FunctionStore::Element(create_element(x)),
//             },
//             FunctionType::Text(x) => Self {
//                 node: FunctionStore::Text(document().create_text_node(&x)),
//             },
//         }
//     }

//     pub fn push_child<T>(&self, child: &T)
//     where
//         T: Deref<Target = Node>,
//     {
//         if let FunctionStore::Element(x) = &self.node {
//             x.append_child(&child).unwrap();
//         }
//     }

//     pub fn add_event(&self, event: Event) {
//         let Event { event, callback } = event;
//         let x = Closure::wrap(callback);
//         self.node()
//             .add_event_listener_with_callback(event, x.as_ref().unchecked_ref())
//             .unwrap();
//         x.forget();
//     }

//     pub fn node(&self) -> &Node {
//         match &self.node {
//             FunctionStore::Element(x) => &x,
//             FunctionStore::Text(x) => &x,
//         }
//     }

//     pub fn element(&self) -> Option<Element> {
//         match &self.node {
//             FunctionStore::Element(x) => Some(x.clone()),
//             _ => None,
//         }
//     }

//     // pub fn set_id(&self, id: &'static str) {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         x.set_id(id);
//     //     }
//     // }

//     // pub fn get_id(&self) -> Option<String> {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         Some(x.id())
//     //     } else {
//     //         None
//     //     }
//     // }

//     // pub fn set_class(&self, class: &'static str) {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         x.set_class_name(class);
//     //     }
//     // }

//     // pub fn get_class(&self) -> Option<String> {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         Some(x.class_name())
//     //     } else {
//     //         None
//     //     }
//     // }

//     // pub fn set_attribute(&self, name: &'static str, value: &'static str) {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         x.set_attribute(name, value).unwrap();
//     //     }
//     // }

//     // pub fn get_attribute(&self, name: &'static str) -> Option<String> {
//     //     if let FunctionStore::Element(x) = &self.node {
//     //         x.get_attribute(name)
//     //     } else {
//     //         None
//     //     }
//     // }
// }

// impl Deref for Function {
//     type Target = Node;

//     fn deref(&self) -> &Self::Target {
//         self.node()
//     }
// }

// impl Component for Function {
//     fn view(self) -> Self {
//         self
//     }
// }

// pub async fn change(node: Arc<Node>, rx: Arc<StateReceiver<Channel>>) {
//     let mut old = StateId::new();
//     while let Some((new, value)) = rx.receive(old).await {
//         node.set_node_value(Some(&value));
//         old = new;
//     }
// }

pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}

pub fn create_element(name: &'static str) -> web_sys::Element {
    document()
        .create_element(name)
        .expect("Cannot create the element needed")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("Document should have a body")
}

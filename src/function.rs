pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}

pub fn create_element(name: impl AsRef<str>) -> web_sys::Element {
    document()
        .create_element(name.as_ref())
        .expect("Cannot create the element needed")
}

pub fn create_text_element(text: impl AsRef<str>) -> web_sys::Text {
    document().create_text_node(text.as_ref())
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("Document should have a body")
}

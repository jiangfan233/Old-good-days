use web_sys::{Document, Element, Window, HtmlElement};


pub struct DomElements {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
}

pub fn create_div(document: &Document, text: &str) -> Element {
    let val = document.create_element("div").unwrap();
    val.set_text_content(Some(text));
    val
}



pub fn init_dom() -> DomElements {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let body = document.body().unwrap();
    

    DomElements {
        window,
        document,
        body,
    }

}
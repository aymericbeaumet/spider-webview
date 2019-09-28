// todo: warn when dataurl length >= 65000? (https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs#Common_problems)
// todo: implement relevant Window properties/methods (https://developer.mozilla.org/en-US/docs/Web/API/Window)
// - window.history.length;
// - window.history.back();
// - window.history.forward();
// - window.history.go();
// - window.history.pushState();
// - window.history.replaceState();
// - window.location = "https://google.com"
// - window.full_screen
// - window.screen_x
// - window.screen_y
// - window.alert();
// - window.blur();
// - window.close();
// - window.confirm();
// - window.find();
// - window.focus();
// - window.get_selection();
// - window.minimize();
// - window.move_by();
// - window.move_to();
// - window.post_message();
// - window.prompt();
// - window.resize_by();
// - window.resize_to();
// - window.scroll_by();
// - window.scroll_to();
// - window.scroll();
// - window.stop();

mod bridge;

use std::ffi::CString;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct DataURL<'a>(pub &'a str, pub &'a str);

impl<'a> ToString for DataURL<'a> {
    fn to_string(&self) -> String {
        let mediatype = self.0;
        let data = base64::encode(self.1);
        format!("data:{};base64,{}", mediatype, data)
    }
}

static ID: AtomicU32 = AtomicU32::new(0);

extern "C" fn handler(window: *const Window) {
    let window = unsafe { *window };
    window.handler();
}

pub type Window = bridge::Window;

impl Default for Window {
    fn default() -> Self {
        Window {
            id: 0,
            left: 200.,
            top: 400.,
            width: 500.,
            height: 300.,
        }
    }
}

impl Window {
    // pub fn post_message(&self, message: &str) {
    //     bridge::post_message(self.id, message);
    // }

    // pub fn close(&self) {
    //     bridge::unregister(self.id);
    // }

    pub fn run(&self) {
        unsafe {
            bridge::run();
        };
    }

    fn handler(&self) {
        println!("window.handler!");
    }
}

pub struct WindowBuilder {
    window: Window,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            window: Window::default(),
        }
    }

    pub fn open(mut self, url: &str) -> Result<Window, Box<dyn std::error::Error>> {
        self.window.id = ID.fetch_add(1, Ordering::SeqCst);
        unsafe {
            bridge::window_open(&self.window, CString::new(url)?.as_ptr(), Some(handler));
        };
        Ok(self.window)
    }

    pub fn left(mut self, left: f64) -> Self {
        self.window.left = left;
        self
    }

    pub fn top(mut self, top: f64) -> Self {
        self.window.top = top;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.window.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.window.height = height;
        self
    }
}

#[derive(Debug)]
pub struct WebViewError(String);

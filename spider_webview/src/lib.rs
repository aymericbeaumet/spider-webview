use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicU32, Ordering};

mod bridge;

static ID: AtomicU32 = AtomicU32::new(0);

pub struct DataURL<'a>(pub &'a str, pub &'a str);

impl<'a> ToString for DataURL<'a> {
    fn to_string(&self) -> String {
        let mediatype = self.0;
        let data = base64::encode(self.1);
        format!("data:{};base64,{}", mediatype, data)
    }
}

extern "C" fn handler(window: *const Window, message: *const i8) {
    let window = unsafe { *window };
    let message = unsafe { CStr::from_ptr(message) }
        .to_string_lossy()
        .into_owned();
    window.handler(&message);
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
    pub fn run(&self) {
        unsafe {
            bridge::run();
        };
    }

    fn handler(&self, message: &str) {
        let value: serde_json::Value = serde_json::from_str(message).unwrap();
        println!("{:?}", value);
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

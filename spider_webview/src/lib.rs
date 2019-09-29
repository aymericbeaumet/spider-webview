use std::collections::HashMap;
use std::ffi::{CStr, CString};

mod bridge;

/// DataURL stores a mediatype and data, it supports generating a string
/// representation
///
/// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
pub struct DataURL<'a>(pub &'a str, pub &'a str);

impl<'a> From<DataURL<'a>> for Vec<u8> {
    fn from(data_url: DataURL<'a>) -> Self {
        data_url.to_string().into()
    }
}

impl<'a> ToString for DataURL<'a> {
    fn to_string(&self) -> String {
        let mediatype = self.0;
        let data = base64::encode(self.1);
        format!("data:{};base64,{}", mediatype, data)
    }
}

/// Event represents the event interface.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Event
pub struct Event {
    pub data: serde_json::Value,
}

/// Window is a C struct generated from the bridge bindings. It implements the
/// builder design pattern to give a high-level abstraction to some raw values.
pub type Window = bridge::Window;

impl Default for Window {
    fn default() -> Self {
        Self {
            top: 400.,
            left: 200.,
            width: 500.,
            height: 300.,
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Window::default()
    }

    pub fn top(mut self, top: f64) -> Self {
        self.top = top;
        self
    }

    pub fn left(mut self, left: f64) -> Self {
        self.left = left;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn open<'a, T: Into<Vec<u8>>>(
        self,
        url: T,
    ) -> Result<Box<OpenWindow<'a>>, Box<dyn std::error::Error>> {
        let mut open_window = Box::new(OpenWindow::new(self));
        unsafe {
            bridge::window_open(
                &(*open_window).window,
                CString::new(url)?.as_ptr(),
                Some(open_window_handle_message_callback),
                &mut *open_window as *mut _ as *mut std::ffi::c_void,
            );
        };
        Ok(open_window)
    }
}

/// OpenWindow wraps a window which has been open.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Window
pub struct OpenWindow<'a> {
    window: Window,
    listeners: HashMap<&'a str, Vec<&'a dyn Fn(&Event)>>,
}

impl<'a> OpenWindow<'a> {
    fn new(window: Window) -> Self {
        OpenWindow {
            window,
            listeners: HashMap::new(),
        }
    }
}

impl<'a> OpenWindow<'a> {
    pub fn add_event_listener(&mut self, event_type: &'a str, listener: &'a dyn Fn(&Event)) {
        let entry = self.listeners.entry(event_type).or_insert(vec![]);
        entry.push(listener);
    }

    pub fn post_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe { bridge::window_post_message(&self.window, CString::new(message)?.as_ptr()) };
        Ok(())
    }

    fn handle(&self, event_type: &str, data: &str) {
        if let Some(listeners) = self.listeners.get(event_type) {
            let event = Event {
                data: serde_json::from_str(data).unwrap(),
            };
            listeners.iter().for_each(|listener| listener(&event));
        }
    }
}

extern "C" fn open_window_handle_message_callback(
    context: *mut std::ffi::c_void,
    message: *const i8,
) {
    let open_window: &OpenWindow = unsafe { &*(context as *mut OpenWindow) };
    let message = unsafe { CStr::from_ptr(message) }
        .to_string_lossy()
        .into_owned();
    open_window.handle("message", &message);
}

pub fn step() {
    unsafe {
        bridge::step();
    };
}

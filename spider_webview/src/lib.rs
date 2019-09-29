use std::collections::HashMap;
use std::ffi::{CStr, CString};

mod bridge;

/// Represents a DataURL.
pub struct DataURL<'a>(pub &'a str, pub &'a str);

impl<'a> ToString for DataURL<'a> {
    fn to_string(&self) -> String {
        let mediatype = self.0;
        let data = base64::encode(self.1);
        format!("data:{};base64,{}", mediatype, data)
    }
}

/// Represents an event.
pub struct Event {
    pub data: serde_json::Value,
}

/// Represents a running window.
pub struct OpenWindow<'a> {
    bridge_window: bridge::Window,
    listeners: HashMap<&'a str, Vec<Box<dyn Fn(&Event)>>>,
}

impl<'a> Default for OpenWindow<'a> {
    fn default() -> Self {
        Self {
            bridge_window: bridge::Window {
                left: 200.,
                top: 400.,
                width: 500.,
                height: 300.,
            },
            listeners: HashMap::new(),
        }
    }
}

impl<'a> OpenWindow<'a> {
    pub fn run(&self) {
        unsafe {
            bridge::run();
        };
    }

    pub fn add_event_listener(&mut self, event_type: &'a str, listener: Box<dyn Fn(&Event)>) {
        let entry = self.listeners.entry(event_type).or_insert(vec![]);
        entry.push(listener);
    }

    fn handler(&self, data: &str) {
        if let Some(listeners) = self.listeners.get("message") {
            let event = Event {
                data: serde_json::from_str(data).unwrap(),
            };
            for listener in listeners {
                listener(&event);
            }
        }
    }
}

extern "C" fn window_handler(context: *mut std::ffi::c_void, message: *const i8) {
    let open_window: &mut OpenWindow = unsafe { &mut *(context as *mut OpenWindow) };
    let message = unsafe { CStr::from_ptr(message) }
        .to_string_lossy()
        .into_owned();
    open_window.handler(&message);
}

/// Allows to build a Window.
pub struct WindowConfiguration<'a> {
    open_window: OpenWindow<'a>,
}

impl<'a> WindowConfiguration<'a> {
    pub fn new() -> Self {
        Self {
            open_window: OpenWindow::default(),
        }
    }

    pub fn open(&mut self, url: &str) -> Result<&'a mut OpenWindow, Box<dyn std::error::Error>> {
        unsafe {
            bridge::window_open(
                &self.open_window.bridge_window,
                CString::new(url)?.as_ptr(),
                Some(window_handler),
                &mut self.open_window as *mut _ as *mut std::ffi::c_void,
            );
        };
        Ok(&mut self.open_window)
    }

    pub fn left(mut self, left: f64) -> Self {
        self.open_window.bridge_window.left = left;
        self
    }

    pub fn top(mut self, top: f64) -> Self {
        self.open_window.bridge_window.top = top;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.open_window.bridge_window.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.open_window.bridge_window.height = height;
        self
    }
}

/// Represents an error.
#[derive(Debug)]
pub struct WindowError(String);

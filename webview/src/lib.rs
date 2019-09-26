use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSMenu, NSMenuItem,
    NSRunningApplication, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{nil, selector, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};

pub fn run(webviews: &[WebView]) {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        let _windows: Vec<_> = webviews
            .iter()
            .map(|webview| {
                let mut style_mask = NSWindowStyleMask::NSBorderlessWindowMask;
                if webview.title_bar {
                    style_mask |= NSWindowStyleMask::NSTitledWindowMask
                }
                if webview.closable {
                    style_mask |= NSWindowStyleMask::NSClosableWindowMask
                }
                if webview.miniaturizable {
                    style_mask |= NSWindowStyleMask::NSMiniaturizableWindowMask
                }
                if webview.resizable {
                    style_mask |= NSWindowStyleMask::NSResizableWindowMask
                }
                let window = NSWindow::alloc(nil)
                    .initWithContentRect_styleMask_backing_defer_(
                        NSRect::new(
                            NSPoint::new(webview.x, webview.y),
                            NSSize::new(webview.width, webview.height),
                        ),
                        style_mask,
                        NSBackingStoreBuffered,
                        NO,
                    )
                    .autorelease();
                window.canBecomeKeyWindow();
                window.canBecomeMainWindow();
                if let Some(ref title) = webview.title {
                    let title = NSString::alloc(nil).init_str(title);
                    window.setTitle_(title);
                }
                window.makeKeyAndOrderFront_(nil);
            })
            .collect();

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}

pub struct WebView {
    title: Option<String>,
    title_bar: bool,
    // size and position
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    // capacity
    closable: bool,
    miniaturizable: bool,
    resizable: bool,
}

impl Default for WebView {
    fn default() -> Self {
        WebView {
            title: None,
            title_bar: true,
            width: 100.,
            height: 100.,
            x: 0.,
            y: 0.,
            closable: true,
            miniaturizable: true,
            resizable: true,
        }
    }
}

pub struct WebViewBuilder {
    webview: WebView,
}

impl WebViewBuilder {
    pub fn new() -> Self {
        WebViewBuilder {
            webview: WebView::default(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.webview.title = Some(title.to_string());
        self
    }

    pub fn title_bar(mut self, title_bar: bool) -> Self {
        self.webview.title_bar = title_bar;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.webview.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.webview.height = height;
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.webview.x = x;
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.webview.y = y;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.webview.closable = closable;
        self
    }

    pub fn miniaturizable(mut self, miniaturizable: bool) -> Self {
        self.webview.miniaturizable = miniaturizable;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.webview.resizable = resizable;
        self
    }

    pub fn build(self) -> WebView {
        self.webview
    }
}

#[derive(Debug)]
pub struct WebViewError(String);

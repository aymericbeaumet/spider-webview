#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "lib.h"
#import "lib-macos-webkit.h"

@implementation MessageHandler

- (id)initWithHandler:(handler_t)handler
{
    self = [super init];
    if (self) {
        self.handler = handler;
    }
    return self;
}

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message {
    self.handler();
}

@end

void window_open(const Window* self, const char* url, handler_t handler) {
    [NSApplication sharedApplication];

    NSWindow * window = [[NSWindow alloc] initWithContentRect:NSMakeRect(self->left, self->top, self->width, self->height)
                                                    styleMask:NSWindowStyleMaskTitled
                                                      backing:NSBackingStoreBuffered
                                                        defer:NO];
    [window autorelease];

    WKUserContentController *userContentController = [[WKUserContentController alloc] init];
    [userContentController addScriptMessageHandler:[[MessageHandler alloc] initWithHandler:handler]
                                              name:@"spider_webview"];
    [userContentController addUserScript:[[WKUserScript alloc] initWithSource:@"window.postMessage = window.webkit.messageHandlers.spider_webview.postMessage.bind(window.webkit.messageHandlers.spider_webview)"
                                                                injectionTime:WKUserScriptInjectionTimeAtDocumentStart
                                                             forMainFrameOnly:YES]];

    WKWebViewConfiguration *configuration = [[WKWebViewConfiguration alloc] init];
    configuration.userContentController = userContentController;

    WKWebView *webView = [[WKWebView alloc] initWithFrame:window.contentView.frame configuration:configuration];

    NSURL *nsurl=[NSURL URLWithString:@(url)];
    NSURLRequest *nsrequest=[NSURLRequest requestWithURL:nsurl];
    [webView loadRequest:nsrequest];

    [window.contentView addSubview:webView];
    [window orderFrontRegardless];
}

void run() {
    [NSApp run];
}

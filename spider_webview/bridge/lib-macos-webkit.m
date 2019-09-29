#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "lib.h"
#import "lib-macos-webkit.h"

void window_open(const Window* self, const char* url, handler_t handler, void* context) {
    [NSApplication sharedApplication];

    NSWindow * window = [[NSWindow alloc] initWithContentRect:NSMakeRect(self->left, self->top, self->width, self->height)
                                                    styleMask:NSWindowStyleMaskTitled
                                                      backing:NSBackingStoreBuffered
                                                        defer:NO];
    [window autorelease];

    WKUserContentController *userContentController = [[WKUserContentController alloc] init];
    [userContentController addScriptMessageHandler:[[MessageHandler alloc] initWithHandler:handler withContext:context]
                                              name:@"spider_webview"];
    [userContentController addUserScript:[[WKUserScript alloc] initWithSource:@"window.postMessage = (message, targetOrigin, transfer) => window.webkit.messageHandlers.spider_webview.postMessage(JSON.stringify(message), targetOrigin, transfer)"
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

@implementation MessageHandler

- (id)initWithHandler:(handler_t)handler withContext:(void*)context
{
    self = [super init];
    if (self) {
        self.handler = handler;
        self.context = context;
    }
    return self;
}

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message {
    // we know message.body is a string (search for JSON.stringify in this file)
    self.handler(self.context, [(NSString *)message.body UTF8String]);
}

@end

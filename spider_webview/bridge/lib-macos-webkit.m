#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "lib.h"
#import "lib-macos-webkit.h"

void window_open(const Window* self, const char* url, MessageHandlerCallback callback, void* context) {
    [NSApplication sharedApplication];

    NSWindow * window = [[NSWindow alloc] initWithContentRect:NSMakeRect(self->left, self->top, self->width, self->height)
                                                    styleMask:NSWindowStyleMaskTitled
                                                      backing:NSBackingStoreBuffered
                                                        defer:NO];
    [window autorelease];

    WKUserContentController *userContentController = [[WKUserContentController alloc] init];
    [userContentController addScriptMessageHandler:[[MessageHandler alloc] initWithCallback:callback withContext:context]
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

void step() {
    for (;;) {
        NSEvent *event = [NSApp nextEventMatchingMask:NSEventMaskAny
                                            untilDate:[NSDate distantPast]
                                               inMode:NSDefaultRunLoopMode
                                              dequeue:YES];
        if (event == nil) {
            break;
        }
        [NSApp sendEvent:event];
    }
}

@implementation MessageHandler

- (id)initWithCallback:(MessageHandlerCallback)callback withContext:(void*)context
{
    self = [super init];
    if (self) {
        self.callback = callback;
        self.context = context;
    }
    return self;
}

- (void)userContentController:(WKUserContentController *) __unused userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message {
    // we know message.body is a string (search for JSON.stringify in this file)
    self.callback(self.context, [(NSString *)message.body UTF8String]);
}

@end

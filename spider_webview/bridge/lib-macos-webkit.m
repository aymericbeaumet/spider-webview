#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "lib.h"
#import "lib-macos-webkit.h"

@implementation MessageHandler

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message {
    NSLog(@"Message received");
}

@end

void window_open(const Window* self, const char* url) {
    [NSApplication sharedApplication];

    NSUInteger windowStyle = NSWindowStyleMaskTitled;

    NSRect windowRect = NSMakeRect(self->left, self->top, self->width, self->height);
    NSWindow * window = [[NSWindow alloc] initWithContentRect:windowRect
                                          styleMask:windowStyle
                                          backing:NSBackingStoreBuffered
                                          defer:NO];
    [window autorelease];

    WKUserContentController *userContentController = [[WKUserContentController alloc] init];
    [userContentController addUserScript:[[WKUserScript alloc] initWithSource:@"window.postMessage = window.webkit.messageHandlers.spider_webview.postMessage.bind(window.webkit.messageHandlers.spider_webview)"
                 injectionTime:WKUserScriptInjectionTimeAtDocumentStart
              forMainFrameOnly:YES]];
    [userContentController addScriptMessageHandler:[[MessageHandler alloc] init]
                           name:@"spider_webview"];

    WKWebViewConfiguration *configuration = [[WKWebViewConfiguration alloc] init];
    configuration.userContentController = userContentController;

    WKWebView *webView = [[WKWebView alloc] initWithFrame:window.contentView.frame configuration:configuration];

    NSURL *nsurl=[NSURL URLWithString:@(url)];
    NSURLRequest *nsrequest=[NSURLRequest requestWithURL:nsurl];
    [webView loadRequest:nsrequest];

    [window.contentView addSubview:webView];
    [window orderFrontRegardless];

    [NSApp run];
}

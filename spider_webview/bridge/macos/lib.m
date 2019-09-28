#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "lib.h"

void window_open(const Window* self, const char* url) {
    [NSApplication sharedApplication];

    NSUInteger windowStyle = NSWindowStyleMaskTitled;

    NSRect windowRect = NSMakeRect(self->left, self->top, self->width, self->height);
    NSWindow * window = [[NSWindow alloc] initWithContentRect:windowRect
                                          styleMask:windowStyle
                                          backing:NSBackingStoreBuffered
                                          defer:NO];
    [window autorelease];

    WKWebViewConfiguration *theConfiguration = [[WKWebViewConfiguration alloc] init];
    WKWebView *webView = [[WKWebView alloc] initWithFrame:window.contentView.frame configuration:theConfiguration];
    NSURL *nsurl=[NSURL URLWithString:@(url)];
    NSURLRequest *nsrequest=[NSURLRequest requestWithURL:nsurl];
    [webView loadRequest:nsrequest];
    [window.contentView addSubview:webView];

    [window orderFrontRegardless];

    [NSApp run];
}

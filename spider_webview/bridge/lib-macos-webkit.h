#ifndef LIB_MACOS_WEBKIT_H_
#define LIB_MACOS_WEBKIT_H_

#import <Foundation/Foundation.h>
#import <WebKit/WebKit.h>
#import "lib.h"

@interface MessageHandler : NSObject <WKScriptMessageHandler>

@property const Window* window;
@property handler_t handler;

- (id)initWithWindow:(const Window*)window withHandler:(handler_t)handler;

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message;

@end

#endif /* !LIB_MACOS_WEBKIT_H_ */

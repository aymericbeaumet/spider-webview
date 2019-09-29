#ifndef LIB_MACOS_WEBKIT_H_
#define LIB_MACOS_WEBKIT_H_

#import <Foundation/Foundation.h>
#import <WebKit/WebKit.h>
#import "lib.h"

@interface MessageHandler : NSObject <WKScriptMessageHandler>

@property handler_t handler;
@property void* context;

- (id)initWithHandler:(handler_t)handler withContext:(void*)context;

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message;

@end

#endif /* !LIB_MACOS_WEBKIT_H_ */

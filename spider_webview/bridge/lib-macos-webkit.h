#ifndef LIB_MACOS_WEBKIT_H_
#define LIB_MACOS_WEBKIT_H_

#import <Foundation/Foundation.h>
#import <WebKit/WebKit.h>
#import "lib.h"

@interface MessageHandler : NSObject <WKScriptMessageHandler>

@property MessageHandlerCallback callback;
@property void* context;

- (id)initWithCallback:(MessageHandlerCallback)callback withContext:(void*)context;

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message;

@end

#endif /* !LIB_MACOS_WEBKIT_H_ */

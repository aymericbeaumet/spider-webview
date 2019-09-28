#import <Foundation/Foundation.h>
#import <WebKit/WebKit.h>

#ifndef LIB_MACOS_WEBKIT_H_
#define LIB_MACOS_WEBKIT_H_

@interface MessageHandler : NSObject <WKScriptMessageHandler>

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message;

@end

#endif /* !LIB_MACOS_WEBKIT_H_ */

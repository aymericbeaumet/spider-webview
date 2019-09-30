#ifndef LIB_H_
#define LIB_H_

typedef struct
{
    double left;
    double top;
    double height;
    double width;
} Window;

/*
 * return codes:
 *   * 0 ok
 */
typedef int (*MessageHandlerCallback)(void *context, const char *data);

/*
 * return codes:
 *   * 0 ok
 */
int step();

/*
 * return codes:
 *   * 0 ok
 *   * -1 you must run window_open from the main thread
 */
int window_open(const Window *self, const char *url, MessageHandlerCallback callback, void *context);

/*
 * return codes:
 *   * 0 ok
 */
int window_post_message(const Window *self, const char *message);

#endif /* !LIB_H_ */

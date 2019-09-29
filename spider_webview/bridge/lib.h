#ifndef LIB_H_
#define LIB_H_

typedef struct
{
    double left;
    double top;
    double height;
    double width;
} Window;

typedef void (*MessageHandlerCallback)(void *context, const char *data);

void step();

void window_open(const Window *self, const char *url, MessageHandlerCallback callback, void *context);
void window_post_message(const Window *self, const char *message);

#endif /* !LIB_H_ */

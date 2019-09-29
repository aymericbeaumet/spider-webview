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

void window_open(const Window *self, const char *url, MessageHandlerCallback callback, void *context);

void step();

#endif /* !LIB_H_ */

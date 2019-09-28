#ifndef LIB_H_
#define LIB_H_

typedef struct
{
    double left;
    double top;
    double height;
    double width;
} Window;

typedef void (*handler_t)(const Window *, const char *);

void window_open(const Window *self, const char *url, handler_t handler);
void run();

#endif /* !LIB_H_ */

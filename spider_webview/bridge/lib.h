#ifndef LIB_H_
#define LIB_H_

typedef struct
{
    unsigned int id;
    double left;
    double top;
    double height;
    double width;
} Window;

void window_open(const Window *self, const char *url);

#endif /* !LIB_H_ */

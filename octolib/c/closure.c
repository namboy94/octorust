#include <stdio.h>

typedef void (*ilet_func) (void*);
typedef int (*dosom) (int, void*);

/*
void handle_closure(int function, void* args) {
    function(args);
}*/

void do_something(dosom func, void* param) {
    printf("START");
    printf("%d", func(0, param));
    printf("END");
}

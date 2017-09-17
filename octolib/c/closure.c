#include <stdio.h>

typedef void (*rust_closure_function) (void*, void*);

void handle_closure_in_c(rust_closure_function func, void* closure_data, void* params) {
    func(closure_data, params);
}

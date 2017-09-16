typedef void (*ilet_func) (void*);

void handle_closure(ilet_func function, void* args) {
    function(args);
}
#include "arena.h"
#include "platform.h"

#define EXPORT __attribute__((used))

// Called after growing the memory buffer so js typed array wrappers stay up to
// date.
extern void js_resetMemoryViews();
extern void js_echoInt(int i);
extern void js_putc(u8 c);

// Arena
extern unsigned long __builtin_wasm_memory_grow(int, unsigned long);
void *platform_alloc_page() {
    int page = __builtin_wasm_memory_grow(0, 1);
    js_resetMemoryViews();
    return (u8 *)0 + (page * PAGE_SIZE);
}

#include "metvan.c"

EXPORT int test() { return 101; }

EXPORT void *init() {
    Arena _arena;
    Platform *platform = arena_push_type(&_arena, Platform);
    platform->arena = _arena;
    platform->renderer.render_rects_count = 0;
    platform->t = 0.0;
    platform->input = (Input){0};
    platform->gamestate = NULL;
    return platform;
}

EXPORT void update_and_render(Platform *platform) {
    game_update_and_render(platform);
    // char s[11] = {'h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'};
    // for (int i = 0; i < 11; i++) {
    //     js_putc(s[i]);
    // }
    // js_putc('\n');
}
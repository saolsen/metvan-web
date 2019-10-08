#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define EXPORT __attribute__((used))

// Called after growing the memory buffer so js typed array wrappers stay up to
// date.
extern void js_resetArrays();

typedef uint8_t u8;
typedef uint32_t u32;
typedef uint64_t u64;
typedef int8_t i8;
typedef int32_t i32;
typedef int64_t i64;
typedef float f32;
typedef double f64;

// Arena
#define PAGE_SIZE 65536

extern unsigned long __builtin_wasm_memory_grow(int, unsigned long);
void *platform_alloc_page() {
    int page = __builtin_wasm_memory_grow(0, 1);
    js_resetArrays();
    return (u8 *)0 + (page * PAGE_SIZE);
}

static void *platform_free_pages = NULL;

typedef struct PageFooter {
    struct PageFooter *prev;
} PageFooter;

typedef struct {
    void *page;
    size_t used;
} Arena;

#define arena_push_type(a, t) (t *)arena_push_size(a, sizeof(t))
void *arena_push_size(Arena *arena, size_t size) {
    if (size > PAGE_SIZE - sizeof(PageFooter)) {
        // error("size > maximum allocable");
        return NULL;
    }
    if (!arena->page || arena->used + size > PAGE_SIZE) {
        // Allocate a new page.
        void *new_page = NULL;
        if (platform_free_pages != NULL) {
            new_page = platform_free_pages;
            platform_free_pages = *(void **)new_page;
        } else {
            new_page = platform_alloc_page();
        }
        PageFooter *footer = (PageFooter *)((uint8_t *)new_page +
                                            (PAGE_SIZE - sizeof(PageFooter)));
        footer->prev = (PageFooter *)arena->page;
        arena->page = new_page;
        arena->used = 0;
    }
    if (size > (PAGE_SIZE - sizeof(PageFooter)) - arena->used) {
        // error("No room for some reason???");
        return NULL;
    }
    void *result = arena->page + arena->used;
    arena->used += size;
    return result;
}

void arena_free(Arena *arena) {
    void *page = arena->page;
    while (page) {
        PageFooter *footer =
            (PageFooter *)((uint8_t *)page + (PAGE_SIZE - sizeof(PageFooter)));
        *(void **)page = platform_free_pages;
        platform_free_pages = page;
        page = footer->prev;
    }
    arena->page = NULL;
    arena->used = 0;
}

// End Arena

EXPORT int test() { return 100; }

typedef struct {
    Arena arena;
} GameState;

typedef struct {
    u8 magic;
    GameState *gamestate;
} Platform;

EXPORT void *update_and_render(Platform *platform) {
    // First call is initialization stuff only.
    if (!platform) {
        Arena arena;
        Platform *platform = arena_push_type(&arena, Platform);
        GameState *gamestate = arena_push_type(&arena, GameState);
        platform->gamestate = gamestate;
        gamestate->arena = arena;

        platform->magic = 99;
        return platform;
    }
    return platform;
}
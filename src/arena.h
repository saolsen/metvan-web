#pragma once
#include "platform.h"

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
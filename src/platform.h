#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef uint8_t u8;
typedef uint32_t u32;
typedef uint64_t u64;
typedef int8_t i8;
typedef int32_t i32;
typedef int64_t i64;
typedef float f32;
typedef double f64;

#define PAGE_SIZE 65536
void *platform_alloc_page();
static void *platform_free_pages = NULL;

typedef struct {
    u32 up;
    u32 down;
    u32 left;
    u32 right;
    u32 jump;

    u32 view_map;
} Input;

typedef enum {
    DebugPink,
    Black,
    DarkPurple,
    DarkBlue,
    DarkGray,
    Gray,
    MediumBlue,
    LightBlue,
    White,
    LightSand,
    MediumSand,
    DarkSand,
    Rock,
    DarkRock,
    Red,
    Green,
    Blue,
} Color;

typedef struct {
    f64 world_center_x;
    f64 world_center_y;
    f64 world_extent_x;
    f64 world_extent_y;
    u32 color;
} RenderRect;

typedef struct {
    RenderRect render_rects[128];
    u32 render_rects_count;
} Renderer;

typedef struct PageFooter {
    struct PageFooter *prev;
} PageFooter;

typedef struct {
    void *page;
    u32 used;
} Arena;

typedef struct {
    Arena arena;
    Renderer renderer;
    f64 t; // seconds
    Input input;
    void *gamestate;
} Platform;

#define GAME_UPDATE_AND_RENDER(name) void name(Platform *platform)
typedef GAME_UPDATE_AND_RENDER(GameUpdateAndRender);
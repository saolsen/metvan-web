#include "metvan.h"

typedef struct {
    f32 player_x;
    f32 player_y;
} GameState;

void game_update_and_render(Platform *platform) {
    GameState *gamestate = (GameState *)platform->gamestate;
    if (!gamestate) {
        platform->gamestate = arena_push_type(&platform->arena, GameState);
        gamestate = (GameState *)platform->gamestate;

        gamestate->player_x = 5.0;
        gamestate->player_y = 7.0;
    }

    Renderer *renderer = &platform->renderer;

    renderer->render_rects[0].world_center_x = gamestate->player_x;
    renderer->render_rects[0].world_center_y = gamestate->player_y + 1.0;
    renderer->render_rects[0].world_extent_x = 0.5;
    renderer->render_rects[0].world_extent_y = 1.0;
    renderer->render_rects[0].color = LightBlue;
    renderer->render_rects_count = 1;
}

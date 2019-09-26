// What's the best way to start out the levels? Is it to design some levels manually?
// Is it to do something else? Powerups or a goal or something?

// I sort of think I should fix jump first.
// Jump at and a timer and checking if I'm on the ground.
use js_sys;
use std::collections::HashMap;

pub struct Orb {
    pub pos: glm::Vec2,
    pub level: u8,
}

pub const EMPTY_ROOM: [u8; 32 * 18] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];
pub struct World {
    pub rooms: HashMap<(i32, i32), [u8; 32 * 18]>,
    pub room_entities: HashMap<(i32, i32), Vec<Orb>>,
    pub doors: HashMap<((i32, i32), (i32, i32)), u8>,
}

impl World {
    pub fn new() -> Self {
        // so, how do I generate a random graph with some properties?
        // pcg is fucking awesome i'm just gonna jam that after this.

        // need to place rooms in such a way that we can get to them?
        // just fill rooms out and add existing kinds of doors.
        // generate a randome new powerup in some rooms.
        // then we can use that kind of door on future fills.
        // at the end is the anti sun.
        // this will make an extremely shitty but functional map.

        // So
        // Generate rooms, just the grid we'll use.
        // Generate doors between the rooms. Use the powerups somehow.
        // Build the map.

        // Lets start with a static map.

        let mut rooms = HashMap::new();
        let mut room_entities = HashMap::new();
        let mut doors = HashMap::new();

        // walk around at different levels?
        // @TODO: Not portable.
        enum Move {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        }

        fn move_to(x: &mut i32, y: &mut i32, step: Move) {
            match step {
                Move::UP => *y -= 1,
                Move::DOWN => *y += 1,
                Move::LEFT => *x -= 1,
                Move::RIGHT => *x += 1,
            };
        }

        let steps = 5;

        rooms.insert((0, 0), EMPTY_ROOM);

        let room_e = room_entities.entry((0, 0)).or_insert(vec![]);
        room_e.push(Orb {
            pos: glm::vec2(3.0, 3.0),
            level: 1,
        });

        let mut pos_x = 0;
        let mut pos_y = 0;
        for step in 0..steps {
            // This is totally not right.
            let f = f64::round(js_sys::Math::random() * 4.0);
            let dir = match f {
                0.0 => Move::UP,
                1.0 => Move::DOWN,
                2.0 => Move::LEFT,
                3.0 => Move::RIGHT,
                _ => Move::RIGHT,
            };
            let prev_x = pos_x;
            let prev_y = pos_y;

            move_to(&mut pos_x, &mut pos_y, dir);
            rooms.insert((pos_x, pos_y), EMPTY_ROOM);

            // this is hacky but they either have matching x's or matching y's so this works for now
            doors.insert(
                (
                    (i32::min(prev_x, pos_x), i32::min(prev_y, pos_y)),
                    (i32::max(prev_x, pos_x), i32::max(prev_y, pos_y)),
                ),
                1,
            );
        }

        let room_e = room_entities.entry((pos_x, pos_y)).or_insert(vec![]);
        room_e.push(Orb {
            pos: glm::vec2(3.0, 3.0),
            level: 2,
        });

        pos_x = 0;
        pos_y = 0;
        for step in 0..steps {
            // This is totally not right.
            let f = f64::round(js_sys::Math::random() * 4.0);
            let dir = match f {
                0.0 => Move::UP,
                1.0 => Move::DOWN,
                2.0 => Move::LEFT,
                3.0 => Move::RIGHT,
                _ => Move::RIGHT,
            };
            let prev_x = pos_x;
            let prev_y = pos_y;

            move_to(&mut pos_x, &mut pos_y, dir);
            rooms.insert((pos_x, pos_y), EMPTY_ROOM);

            // this is hacky but they either have matching x's or matching y's so this works for now
            if !doors.contains_key(&(
                (i32::min(prev_x, pos_x), i32::min(prev_y, pos_y)),
                (i32::max(prev_x, pos_x), i32::max(prev_y, pos_y)),
            )) {
                doors.insert(
                    (
                        (i32::min(prev_x, pos_x), i32::min(prev_y, pos_y)),
                        (i32::max(prev_x, pos_x), i32::max(prev_y, pos_y)),
                    ),
                    2,
                );
            }
        }

        let room_e = room_entities.entry((pos_x, pos_y)).or_insert(vec![]);
        room_e.push(Orb {
            pos: glm::vec2(3.0, 3.0),
            level: 3,
        });

        pos_x = 0;
        pos_y = 0;
        for _step in 0..steps {
            // This is totally not right.
            let f = f64::round(js_sys::Math::random() * 4.0);
            let dir = match f {
                0.0 => Move::UP,
                1.0 => Move::DOWN,
                2.0 => Move::LEFT,
                3.0 => Move::RIGHT,
                _ => Move::RIGHT,
            };
            let prev_x = pos_x;
            let prev_y = pos_y;

            move_to(&mut pos_x, &mut pos_y, dir);
            rooms.insert((pos_x, pos_y), EMPTY_ROOM);

            // this is hacky but they either have matching x's or matching y's so this works for now
            if !doors.contains_key(&(
                (i32::min(prev_x, pos_x), i32::min(prev_y, pos_y)),
                (i32::max(prev_x, pos_x), i32::max(prev_y, pos_y)),
            )) {
                doors.insert(
                    (
                        (i32::min(prev_x, pos_x), i32::min(prev_y, pos_y)),
                        (i32::max(prev_x, pos_x), i32::max(prev_y, pos_y)),
                    ),
                    3,
                );
            }
        }

        let room_e = room_entities.entry((pos_x, pos_y)).or_insert(vec![]);
        room_e.push(Orb {
            pos: glm::vec2(3.0, 3.0),
            level: 4,
        });

        // for x in -3..=3 {
        //     for y in -3..=3 {
        //         rooms.insert((x, y), EMPTY_ROOM);
        //     }
        // }

        // for x in -3..=3 {
        //     for y in -3..=3 {
        //         if x != 3 {
        //             doors.insert(((x, y), (x + 1, y)), 1);
        //         }
        //         if y != 3 {
        //             doors.insert(((x, y), (x, y + 1)), 1);
        //         }
        //     }
        // }

        let room_e = room_entities.entry((0, 0)).or_insert(vec![]);
        room_e.push(Orb {
            pos: glm::vec2(3.0, 3.0),
            level: 1,
        });

        // Add doors
        for (&(x, y), room) in rooms.iter_mut() {
            // Left
            if let Some(door) = doors.get(&((x - 1, y), (x, y))) {
                room[32 * 14] = *door;
                room[32 * 15] = *door;
                room[32 * 16] = *door;
            }
            // Right
            if let Some(door) = doors.get(&((x, y), (x + 1, y))) {
                room[32 * 14 + 31] = *door;
                room[32 * 15 + 31] = *door;
                room[32 * 16 + 31] = *door;
            }
            // Top
            if let Some(door) = doors.get(&((x, y), (x, y + 1))) {
                room[14] = *door;
                room[15] = *door;
                room[16] = *door;
                room[17] = *door;
                // We also need to be able to get up there.
                room[32 * 1 + 13] = 4;
                room[32 * 2 + 14] = 4;

                room[32 * 5 + 13] = 4;
                room[32 * 5 + 14] = 4;
                room[32 * 5 + 15] = 4;
                room[32 * 5 + 16] = 4;

                room[32 * 9 + 9] = 4;
                room[32 * 9 + 10] = 4;
                room[32 * 9 + 11] = 4;

                room[32 * 13 + 5] = 4;
                room[32 * 13 + 6] = 4;
                room[32 * 13 + 7] = 4;
            }
            // Bottom
            if let Some(door) = doors.get(&((x, y - 1), (x, y))) {
                room[32 * 17 + 14] = *door;
                room[32 * 17 + 15] = *door;
                room[32 * 17 + 16] = *door;
                room[32 * 17 + 17] = *door;
            }
        }

        return World {
            rooms,
            room_entities,
            doors,
        };

        // somehow pick a tile on the boarder.
        // @NOTE: making it easier again, just doing x!
        // couple things, for one I'd like to see the map!

        // let mut level = 0;
        // for i in 0..10 {
        //     let y = 0;
        //     let x = if i % 2 == 0 { min_x - 1 } else { max_x + 1 };
        //     if (i + 1) % 3 == 0 {
        //         // add an orb that activates the next level? LOL OR SOMETHING.
        //         let room_e = room_entities.entry((x, y)).or_insert(vec![]);
        //         room_e.push(Orb {
        //             pos: glm::vec2(10.0, 3.0),
        //             level: level + 1,
        //         });
        //     }
        //     if i % 3 == 0 {
        //         level = level + 1;
        //     }

        //     rooms.insert((x, y), level);
        //     min_x = i32::min(x, min_x);
        //     max_x = i32::max(x, max_x);
        // }

        // // So right now this is building a door if there's a room on the other side.
        // // What I want to do though is build a door if there's an edge between them
        // // and that edge will have a color value.
        // // So what I need to generate is the dependency graph between rooms.

        // // we can do like an always going up thing first, even though that's not quite what we're getting at.

        // let mut map = HashMap::new();
        // for ((x, y), room_level) in &rooms {
        //     let mut new_room = EMPTY_ROOM;

        //     if let Some(next_room_level) = rooms.get(&(x - 1, *y)) {
        //         let door = u8::max(*room_level, *next_room_level);
        //         room_doors.insert(((*x - 1, *y), (*x, *y)), door);
        //         // add door to the left
        //         new_room[32 * 14] = door;
        //         new_room[32 * 15] = door;
        //         new_room[32 * 16] = door;
        //     }
        //     if let Some(next_room_level) = rooms.get(&(x + 1, *y)) {
        //         let door = u8::max(*room_level, *next_room_level);
        //         room_doors.insert(((*x, *y), (*x + 1, *y)), door);
        //         // add door to the right
        //         new_room[32 * 14 + 31] = door;
        //         new_room[32 * 15 + 31] = door;
        //         new_room[32 * 16 + 31] = door;
        //     }

        //     if *room_level == 4 {
        //         new_room[32 * 6 + 15] = 5;
        //         new_room[32 * 6 + 16] = 5;
        //         new_room[32 * 6 + 17] = 5;
        //         new_room[32 * 5 + 15] = 5;
        //         new_room[32 * 5 + 16] = 5;
        //         new_room[32 * 5 + 17] = 5;
        //         new_room[32 * 4 + 15] = 5;
        //         new_room[32 * 4 + 16] = 5;
        //         new_room[32 * 4 + 17] = 5;
        //     }

        //     map.insert((*x, *y), new_room);
        // }

        // room_doors.insert(((0, 0), (0, 1)), 3);

        // // rooms.insert((0, 0), STANDARD_ROOM);
        // // rooms.insert((1, 0), NEXT_ROOM);
        // // rooms.insert((1, 1), ABOVE_ROOM);
        // // rooms.insert((0, 1), ROLLTHRU_ROOM);
        // // rooms.insert((-1, 1), OVERHANG_ROOM);
        // // rooms.insert((-1, 0), SUN_ROOM);
        // Self {
        //     rooms: map,
        //     room_entities,
        //     room_levels: rooms,
        //     room_doors,
        // }
    }
}

// almost ported to c again! Really need to watch out for that steve.

// So, what's the way to do this?
/*
So, I can represent the ways to get from room to room as a graph.
The rooms can be nodes and the edges can be doors between them.
Those doors could have dependencies, like normal (just walk through)
* Red (you need the red powerup to get through it)

So one strategy could be to generate a graph with the sort of dependencies that we want.
The graph would have to be laid out in the plane or whatever (or maybe not...)
Then we'd generate rooms with the right kind of doors?

what is the 100% simplest version of this?

I could have an empty room and then 4 room templates that could be applied.
All these would do is add doors to the room.


Design space.
Get really specific about the thing I'm generating.

Artist in a box (make levels like a level designer, learn that from those videos the most.)

additive and subtractive methods


Tiles
- tile based map generation
- break it into regions, this is sorta what I'm doing.
-
grammars
- recursive generation of stuff

do I randomly generate new nodes onto the graph, how do I keep the graph playable?



*/

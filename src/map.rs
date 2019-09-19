// What's the best way to start out the levels? Is it to design some levels manually?
// Is it to do something else? Powerups or a goal or something?

// I sort of think I should fix jump first.
// Jump at and a timer and checking if I'm on the ground.

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

        let mut rooms = HashMap::new();
        rooms.insert((0, 0), 0);
        let mut min_x = 0;
        let mut max_x = 0;

        // We now need to be able to place entities into rooms.
        let mut room_entities = HashMap::new();
        room_entities.insert(
            (0, 0),
            vec![Orb {
                pos: glm::vec2(10.0, 3.0),
                level: 1,
            }],
        );

        // somehow pick a tile on the boarder.
        // @NOTE: making it easier again, just doing x!
        let mut level = 0;
        for i in 0..10 {
            let y = 0;
            let x = if i % 2 == 0 { min_x - 1 } else { max_x + 1 };
            if (i + 1) % 3 == 0 {
                // add an orb that activates the next level? LOL OR SOMETHING.
                let room_e = room_entities.entry((x, y)).or_insert(vec![]);
                room_e.push(Orb {
                    pos: glm::vec2(10.0, 3.0),
                    level: level + 1,
                });
            }
            if i % 3 == 0 {
                level = level + 1;
            }
            rooms.insert((x, y), level);
            min_x = i32::min(x, min_x);
            max_x = i32::max(x, max_x);
        }

        // So right now this is building a door if there's a room on the other side.
        // What I want to do though is build a door if there's an edge between them
        // and that edge will have a color value.
        // So what I need to generate is the dependency graph between rooms.

        // we can do like an always going up thing first, even though that's not quite what we're getting at.

        let mut map = HashMap::new();
        for ((x, y), room_level) in &rooms {
            let mut new_room = EMPTY_ROOM;

            if let Some(next_room_level) = rooms.get(&(x - 1, *y)) {
                let door = u8::max(*room_level, *next_room_level);
                // add door to the left
                new_room[32 * 14] = door;
                new_room[32 * 15] = door;
                new_room[32 * 16] = door;
            }
            if let Some(next_room_level) = rooms.get(&(x + 1, *y)) {
                let door = u8::max(*room_level, *next_room_level);
                // add door to the right
                new_room[32 * 14 + 31] = door;
                new_room[32 * 15 + 31] = door;
                new_room[32 * 16 + 31] = door;
            }

            map.insert((*x, *y), new_room);
        }

        // rooms.insert((0, 0), STANDARD_ROOM);
        // rooms.insert((1, 0), NEXT_ROOM);
        // rooms.insert((1, 1), ABOVE_ROOM);
        // rooms.insert((0, 1), ROLLTHRU_ROOM);
        // rooms.insert((-1, 1), OVERHANG_ROOM);
        // rooms.insert((-1, 0), SUN_ROOM);
        Self {
            rooms: map,
            room_entities,
        }
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

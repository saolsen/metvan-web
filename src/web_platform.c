#include <stdint.h>
#define NULL 0
#define PAGE_SIZE 65536

// Called after growing the memory buffer so js typed array wrappers stay up to
// date.
extern void js_reset_arrays();

typedef uint8_t u8;
typedef uint32_t u32;
typedef uint64_t u64;
typedef int8_t i8;
typedef int32_t i32;
typedef int64_t i64;
typedef float f32;
typedef double f64;

// size_t is 32 bits in wasm but we'll still just use size_t.

int test() { return 69; }
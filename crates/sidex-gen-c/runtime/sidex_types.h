#ifndef SIDEX_TYPES_H
#define SIDEX_TYPES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SIDEX_ALIGNOF(type)                                                                        \
	offsetof(                                                                                  \
	    struct {                                                                               \
		    char sidex_padding;                                                            \
		    type sidex_value;                                                              \
	    },                                                                                     \
	    sidex_value)

/** Common status codes returned by the Sidex C runtime. */
typedef enum sidex_status {
	SIDEX_OK = 0,
	SIDEX_ERR_EOF = -1,
	SIDEX_ERR_SYNTAX = -2,
	SIDEX_ERR_EXPECTED = -3,
	SIDEX_ERR_OVERFLOW = -4,
	SIDEX_ERR_NO_SPACE = -5,
	SIDEX_ERR_UTF8 = -6,
	SIDEX_ERR_UNSUPPORTED = -7,
	SIDEX_ERR_CALLBACK = -8,
	SIDEX_ERR_DEPTH = -9,
} sidex_status;

/** Borrowed UTF-8 string. The bytes are not NUL terminated. */
typedef struct sidex_str {
	const uint8_t *ptr;
	size_t len;
} sidex_str;

/**
 * Caller-owned bump arena used by decoders when data cannot be borrowed.
 *
 * Decoded values may borrow from this storage. Reset it only after those
 * values are no longer used.
 */
typedef struct sidex_arena {
	uint8_t *ptr;
	size_t cap;
	size_t len;
} sidex_arena;

/** Creates a borrowed string from raw bytes. The bytes must be valid UTF-8. */
static inline sidex_str sidex_str_from_parts(const uint8_t *ptr, size_t len)
{
	sidex_str str;

	str.ptr = ptr;
	str.len = len;
	return str;
}

/** Creates a borrowed string from a NUL-terminated UTF-8 C string. */
static inline sidex_str sidex_str_from_cstr(const char *ptr)
{
	return sidex_str_from_parts((const uint8_t *)ptr, strlen(ptr));
}

/** Compares two borrowed strings byte-for-byte. */
static inline bool sidex_str_eq(sidex_str lhs, sidex_str rhs)
{
	if (lhs.len != rhs.len) {
		return false;
	}
	if (lhs.len == 0) {
		return true;
	}
	return memcmp(lhs.ptr, rhs.ptr, lhs.len) == 0;
}

/** Creates an empty arena from a caller-owned byte buffer. */
static inline sidex_arena sidex_arena_from_parts(uint8_t *ptr, size_t cap)
{
	sidex_arena arena;

	arena.ptr = ptr;
	arena.cap = cap;
	arena.len = 0;
	return arena;
}

/** Resets an arena, invalidating decoded values that borrow from it. */
static inline void sidex_arena_reset(sidex_arena *arena)
{
	arena->len = 0;
}

/** Allocates aligned memory from an arena. */
void *sidex_arena_alloc(sidex_arena *arena, size_t size, size_t align);

/** Allocates an aligned array from an arena, checking for size overflow. */
void *sidex_arena_alloc_array(sidex_arena *arena, size_t count, size_t elem_size, size_t align);

/** Returns true if the byte slice is well-formed UTF-8. */
bool sidex_utf8_validate(const uint8_t *ptr, size_t len);

#ifdef __cplusplus
}
#endif

#endif

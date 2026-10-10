#include "sidex_types.h"

static bool sidex_is_power_of_two(size_t value)
{
	return value != 0 && (value & (value - 1)) == 0;
}

void *sidex_arena_alloc(sidex_arena *arena, size_t size, size_t align)
{
	uintptr_t base;
	uintptr_t current;
	uintptr_t aligned;
	size_t padding;

	if (arena == NULL || arena->ptr == NULL || !sidex_is_power_of_two(align)) {
		return NULL;
	}
	if (arena->len > arena->cap) {
		return NULL;
	}

	base = (uintptr_t)arena->ptr;
	current = base + arena->len;
	aligned = (current + (uintptr_t)align - 1u) & ~((uintptr_t)align - 1u);
	padding = (size_t)(aligned - current);

	if (padding > arena->cap - arena->len || size > arena->cap - arena->len - padding) {
		return NULL;
	}

	arena->len += padding + size;
	return (void *)aligned;
}

void *sidex_arena_alloc_array(sidex_arena *arena, size_t count, size_t elem_size, size_t align)
{
	if (elem_size != 0 && count > (SIZE_MAX / elem_size)) {
		return NULL;
	}
	return sidex_arena_alloc(arena, count * elem_size, align);
}

static bool sidex_utf8_is_continuation(uint8_t ch)
{
	return (ch & 0xc0u) == 0x80u;
}

bool sidex_utf8_validate(const uint8_t *ptr, size_t len)
{
	size_t i = 0;

	/*
	 * Implements the valid UTF-8 byte ranges from RFC 3629 section 4.
	 * The special E0/ED/F0/F4 cases reject overlong encodings, surrogate
	 * code points, and code points above U+10FFFF.
	 */
	while (i < len) {
		uint8_t b0 = ptr[i];

		if (b0 <= 0x7fu) {
			i++;
			continue;
		}
		if (b0 >= 0xc2u && b0 <= 0xdfu) {
			if (i + 1 >= len || !sidex_utf8_is_continuation(ptr[i + 1])) {
				return false;
			}
			i += 2;
			continue;
		}
		if (b0 == 0xe0u) {
			if (i + 2 >= len || ptr[i + 1] < 0xa0u || ptr[i + 1] > 0xbfu ||
			    !sidex_utf8_is_continuation(ptr[i + 2])) {
				return false;
			}
			i += 3;
			continue;
		}
		if ((b0 >= 0xe1u && b0 <= 0xecu) || (b0 >= 0xeeu && b0 <= 0xefu)) {
			if (i + 2 >= len || !sidex_utf8_is_continuation(ptr[i + 1]) ||
			    !sidex_utf8_is_continuation(ptr[i + 2])) {
				return false;
			}
			i += 3;
			continue;
		}
		if (b0 == 0xedu) {
			if (i + 2 >= len || ptr[i + 1] < 0x80u || ptr[i + 1] > 0x9fu ||
			    !sidex_utf8_is_continuation(ptr[i + 2])) {
				return false;
			}
			i += 3;
			continue;
		}
		if (b0 == 0xf0u) {
			if (i + 3 >= len || ptr[i + 1] < 0x90u || ptr[i + 1] > 0xbfu ||
			    !sidex_utf8_is_continuation(ptr[i + 2]) ||
			    !sidex_utf8_is_continuation(ptr[i + 3])) {
				return false;
			}
			i += 4;
			continue;
		}
		if (b0 >= 0xf1u && b0 <= 0xf3u) {
			if (i + 3 >= len || !sidex_utf8_is_continuation(ptr[i + 1]) ||
			    !sidex_utf8_is_continuation(ptr[i + 2]) ||
			    !sidex_utf8_is_continuation(ptr[i + 3])) {
				return false;
			}
			i += 4;
			continue;
		}
		if (b0 == 0xf4u) {
			if (i + 3 >= len || ptr[i + 1] < 0x80u || ptr[i + 1] > 0x8fu ||
			    !sidex_utf8_is_continuation(ptr[i + 2]) ||
			    !sidex_utf8_is_continuation(ptr[i + 3])) {
				return false;
			}
			i += 4;
			continue;
		}
		return false;
	}
	return true;
}

#ifndef SIDEX_JSON_READER_H
#define SIDEX_JSON_READER_H

#include "sidex_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/** Token kinds visible to the low-level streaming JSON reader. */
typedef enum sidex_json_token_kind {
	SIDEX_JSON_TOKEN_END = 0,
	SIDEX_JSON_TOKEN_OBJECT_BEGIN,
	SIDEX_JSON_TOKEN_OBJECT_END,
	SIDEX_JSON_TOKEN_ARRAY_BEGIN,
	SIDEX_JSON_TOKEN_ARRAY_END,
	SIDEX_JSON_TOKEN_NAME_SEPARATOR,
	SIDEX_JSON_TOKEN_VALUE_SEPARATOR,
	SIDEX_JSON_TOKEN_STRING,
	SIDEX_JSON_TOKEN_NUMBER,
	SIDEX_JSON_TOKEN_TRUE,
	SIDEX_JSON_TOKEN_FALSE,
	SIDEX_JSON_TOKEN_NULL,
} sidex_json_token_kind;

#define SIDEX_JSON_DEFAULT_MAX_DEPTH ((size_t)128u)

/** Streaming reader over a caller-owned JSON byte slice. */
typedef struct sidex_json_reader {
	const uint8_t *ptr;
	size_t len;
	size_t pos;
} sidex_json_reader;

/** Initializes a reader over a non-NUL-terminated JSON byte slice. */
void sidex_json_reader_init(sidex_json_reader *reader, const uint8_t *ptr, size_t len);

/** Peeks at the next token without consuming it, ignoring leading whitespace. */
int sidex_json_peek(sidex_json_reader *reader, sidex_json_token_kind *kind);

/** Consumes the next token if it matches `kind`. */
int sidex_json_expect(sidex_json_reader *reader, sidex_json_token_kind kind);

/** Reads a JSON boolean value. */
int sidex_json_read_bool(sidex_json_reader *reader, bool *out);

/** Reads a JSON null value. */
int sidex_json_read_null(sidex_json_reader *reader);

/** Reads a JSON integer into a signed 64-bit value. Fractions are unsupported. */
int sidex_json_read_i64(sidex_json_reader *reader, int64_t *out);

/** Reads a JSON integer into an unsigned 64-bit value. Fractions are unsupported. */
int sidex_json_read_u64(sidex_json_reader *reader, uint64_t *out);

/** Reads a JSON number lexeme. */
int sidex_json_read_number(sidex_json_reader *reader, sidex_str *out);

/**
 * Reads a JSON string.
 *
 * Unescaped strings are borrowed from the input. Escaped strings are decoded
 * into `arena`, and the returned string borrows from it.
 */
int sidex_json_read_str(sidex_json_reader *reader, sidex_str *out, sidex_arena *arena);

/**
 * Consumes one complete JSON value without materializing it.
 *
 * `max_depth` bounds recursive value nesting. The top-level value consumes one
 * level, so a scalar requires at least 1 and `[0]` requires at least 2.
 */
int sidex_json_skip_value(sidex_json_reader *reader, size_t max_depth);

#ifdef __cplusplus
}
#endif

#endif

#ifndef SIDEX_JSON_VALUE_H
#define SIDEX_JSON_VALUE_H

#include "sidex_json_reader.h"
#include "sidex_json_writer.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct sidex_json_value sidex_json_value;
typedef struct sidex_json_entry sidex_json_entry;

/** JSON value kind used by the arbitrary JSON representation. */
typedef enum sidex_json_value_kind {
	SIDEX_JSON_VALUE_NULL = 0,
	SIDEX_JSON_VALUE_BOOL,
	SIDEX_JSON_VALUE_NUMBER,
	SIDEX_JSON_VALUE_STRING,
	SIDEX_JSON_VALUE_ARRAY,
	SIDEX_JSON_VALUE_OBJECT,
} sidex_json_value_kind;

/** Array storage for arbitrary JSON values. */
typedef struct sidex_json_array {
	sidex_json_value *items;
	size_t len;
} sidex_json_array;

/** Object storage for arbitrary JSON values. */
typedef struct sidex_json_object {
	sidex_json_entry *entries;
	size_t len;
} sidex_json_object;

/**
 * Arbitrary JSON value.
 *
 * Strings and numbers may borrow from the original JSON input. Strings with
 * escapes are decoded into the arena. Arrays and objects are stored in the
 * arena. The input buffer and arena must both outlive values decoded from them.
 */
struct sidex_json_value {
	sidex_json_value_kind kind;
	union {
		bool boolean;
		sidex_str number;
		sidex_str string;
		sidex_json_array array;
		sidex_json_object object;
	} as;
};

/** Object entry storage for arbitrary JSON values. */
struct sidex_json_entry {
	sidex_str key;
	sidex_json_value value;
};

/**
 * Decodes one JSON value into an arbitrary JSON representation.
 *
 * `max_depth` bounds recursive value nesting. The top-level value consumes one
 * level, so a scalar requires at least 1 and `[0]` requires at least 2.
 */
int sidex_json_read_value(sidex_json_reader *reader, sidex_json_value *out, sidex_arena *arena,
                          size_t max_depth);

/**
 * Encodes one arbitrary JSON value.
 *
 * `max_depth` bounds recursive value nesting. The top-level value consumes one
 * level, so a scalar requires at least 1 and `[0]` requires at least 2.
 */
int sidex_json_write_value(sidex_json_writer *writer, const sidex_json_value *value,
                           size_t max_depth);

#ifdef __cplusplus
}
#endif

#endif

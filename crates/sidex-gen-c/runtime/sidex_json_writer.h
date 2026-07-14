#ifndef SIDEX_JSON_WRITER_H
#define SIDEX_JSON_WRITER_H

#include "sidex_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/** Callback used by the streaming JSON writer to emit bytes. */
typedef int (*sidex_json_write_fn)(void *ctx, const uint8_t *ptr, size_t len);

/** Streaming JSON writer backed by a caller-provided callback. */
typedef struct sidex_json_writer {
	sidex_json_write_fn write;
	void *ctx;
} sidex_json_writer;

/** Fixed-size output buffer adapter for sidex_json_writer. */
typedef struct sidex_json_buffer {
	uint8_t *ptr;
	size_t cap;
	size_t len;
} sidex_json_buffer;

/** Initializes a fixed-size output buffer. */
void sidex_json_buffer_init(sidex_json_buffer *buffer, uint8_t *ptr, size_t cap);

/** Creates a writer that appends to a fixed-size output buffer. */
sidex_json_writer sidex_json_writer_for_buffer(sidex_json_buffer *buffer);

/** Emits raw bytes through the writer callback. */
int sidex_json_write_raw(sidex_json_writer *writer, const uint8_t *ptr, size_t len);

/** Writes a JSON null value. */
int sidex_json_write_null(sidex_json_writer *writer);

/** Writes a JSON boolean value. */
int sidex_json_write_bool(sidex_json_writer *writer, bool value);

/** Writes a JSON signed integer value. */
int sidex_json_write_i64(sidex_json_writer *writer, int64_t value);

/** Writes a JSON unsigned integer value. */
int sidex_json_write_u64(sidex_json_writer *writer, uint64_t value);

/** Writes a JSON string, validating UTF-8 and escaping JSON control bytes. */
int sidex_json_write_str(sidex_json_writer *writer, sidex_str value);

/** Writes `{`. */
int sidex_json_write_object_begin(sidex_json_writer *writer);

/** Writes `}`. */
int sidex_json_write_object_end(sidex_json_writer *writer);

/** Writes `[`. */
int sidex_json_write_array_begin(sidex_json_writer *writer);

/** Writes `]`. */
int sidex_json_write_array_end(sidex_json_writer *writer);

/** Writes `:`. */
int sidex_json_write_name_separator(sidex_json_writer *writer);

/** Writes `,`. */
int sidex_json_write_value_separator(sidex_json_writer *writer);

/** Writes a JSON object key followed by `:`. */
int sidex_json_write_object_key(sidex_json_writer *writer, sidex_str key);

#ifdef __cplusplus
}
#endif

#endif

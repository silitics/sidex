#include "sidex_json.h"

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#ifndef SIDEX_FUZZ_MAX_INPUT
#define SIDEX_FUZZ_MAX_INPUT 16384u
#endif

#ifndef SIDEX_FUZZ_ARENA_CAP
#define SIDEX_FUZZ_ARENA_CAP 65536u
#endif

static void require(bool condition)
{
	if (!condition) {
		abort();
	}
}

static void exercise_skip_value(const uint8_t *data, size_t size)
{
	sidex_json_reader reader;
	sidex_json_token_kind kind;
	int rc;

	sidex_json_reader_init(&reader, data, size);
	rc = sidex_json_skip_value(&reader, SIDEX_JSON_DEFAULT_MAX_DEPTH);
	if (rc == SIDEX_OK) {
		(void)sidex_json_peek(&reader, &kind);
	}
}

static void exercise_value_round_trip(const uint8_t *data, size_t size)
{
	uint8_t *arena_storage;
	uint8_t *second_arena_storage;
	uint8_t *output;
	uint8_t *second_output;
	size_t output_cap;
	sidex_arena arena;
	sidex_arena second_arena;
	sidex_json_reader reader;
	sidex_json_reader second_reader;
	sidex_json_value value;
	sidex_json_value second_value;
	sidex_json_buffer buffer;
	sidex_json_buffer second_buffer;
	sidex_json_writer writer;
	sidex_json_writer second_writer;
	sidex_json_token_kind kind;
	int rc;

	arena_storage = (uint8_t *)malloc(SIDEX_FUZZ_ARENA_CAP);
	second_arena_storage = (uint8_t *)malloc(SIDEX_FUZZ_ARENA_CAP);
	output_cap = size * 8u + 256u;
	output = (uint8_t *)malloc(output_cap);
	second_output = (uint8_t *)malloc(output_cap);
	if (arena_storage == NULL || second_arena_storage == NULL || output == NULL ||
	    second_output == NULL) {
		goto done;
	}

	arena = sidex_arena_from_parts(arena_storage, SIDEX_FUZZ_ARENA_CAP);
	sidex_json_reader_init(&reader, data, size);
	rc = sidex_json_read_value(&reader, &value, &arena, SIDEX_JSON_DEFAULT_MAX_DEPTH);
	if (rc != SIDEX_OK) {
		goto done;
	}

	rc = sidex_json_peek(&reader, &kind);
	if (rc != SIDEX_OK) {
		goto done;
	}
	if (kind != SIDEX_JSON_TOKEN_END) {
		goto done;
	}

	sidex_json_buffer_init(&buffer, output, output_cap);
	writer = sidex_json_writer_for_buffer(&buffer);
	rc = sidex_json_write_value(&writer, &value, SIDEX_JSON_DEFAULT_MAX_DEPTH);
	if (rc != SIDEX_OK) {
		goto done;
	}

	second_arena = sidex_arena_from_parts(second_arena_storage, SIDEX_FUZZ_ARENA_CAP);
	sidex_json_reader_init(&second_reader, output, buffer.len);
	rc = sidex_json_read_value(&second_reader, &second_value, &second_arena,
	                           SIDEX_JSON_DEFAULT_MAX_DEPTH);
	require(rc == SIDEX_OK);
	rc = sidex_json_peek(&second_reader, &kind);
	require(rc == SIDEX_OK);
	require(kind == SIDEX_JSON_TOKEN_END);

	sidex_json_buffer_init(&second_buffer, second_output, output_cap);
	second_writer = sidex_json_writer_for_buffer(&second_buffer);
	rc = sidex_json_write_value(&second_writer, &second_value, SIDEX_JSON_DEFAULT_MAX_DEPTH);
	require(rc == SIDEX_OK);
	require(second_buffer.len == buffer.len);
	require(memcmp(second_output, output, buffer.len) == 0);

done:
	free(second_output);
	free(output);
	free(second_arena_storage);
	free(arena_storage);
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
	if (size > SIDEX_FUZZ_MAX_INPUT) {
		return 0;
	}

	(void)sidex_utf8_validate(data, size);
	exercise_skip_value(data, size);
	exercise_value_round_trip(data, size);
	return 0;
}

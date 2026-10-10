#include "sidex_json.h"
#include "sidex_json_reader.h"
#include "sidex_json_writer.h"

#include <stdio.h>
#include <string.h>

#define CHECK(condition)                                                                           \
	do {                                                                                       \
		if (!(condition)) {                                                                \
			return __LINE__;                                                           \
		}                                                                                  \
	} while (0)

#define CHECK_OK(expression)                                                                       \
	do {                                                                                       \
		int rc_ = (expression);                                                            \
		if (rc_ != SIDEX_OK) {                                                             \
			return __LINE__;                                                           \
		}                                                                                  \
	} while (0)

#define CHECK_RC(expression, expected)                                                             \
	do {                                                                                       \
		int rc_ = (expression);                                                            \
		if (rc_ != (expected)) {                                                           \
			return __LINE__;                                                           \
		}                                                                                  \
	} while (0)

static bool bytes_eq(sidex_str actual, const uint8_t *expected, size_t expected_len)
{
	if (actual.len != expected_len) {
		return false;
	}
	if (expected_len == 0) {
		return true;
	}
	return memcmp(actual.ptr, expected, expected_len) == 0;
}

static bool str_eq_cstr(sidex_str actual, const char *expected)
{
	return bytes_eq(actual, (const uint8_t *)expected, strlen(expected));
}

static bool ptr_in_range(const uint8_t *ptr, const uint8_t *start, size_t len)
{
	uintptr_t address = (uintptr_t)ptr;
	uintptr_t range_start = (uintptr_t)start;

	return address >= range_start && address - range_start < len;
}

static int read_key(sidex_json_reader *reader, const char *expected)
{
	uint8_t arena_storage[32];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_str key;

	CHECK_OK(sidex_json_read_str(reader, &key, &arena));
	CHECK(str_eq_cstr(key, expected));
	CHECK_OK(sidex_json_expect(reader, SIDEX_JSON_TOKEN_NAME_SEPARATOR));
	return 0;
}

static int test_utf8(void)
{
	static const uint8_t ascii[] = {'a'};
	static const uint8_t min_2[] = {0xc2u, 0x80u};
	static const uint8_t max_2[] = {0xdfu, 0xbfu};
	static const uint8_t min_3[] = {0xe0u, 0xa0u, 0x80u};
	static const uint8_t before_surrogate[] = {0xedu, 0x9fu, 0xbfu};
	static const uint8_t after_surrogate[] = {0xeeu, 0x80u, 0x80u};
	static const uint8_t min_4[] = {0xf0u, 0x90u, 0x80u, 0x80u};
	static const uint8_t max_4[] = {0xf4u, 0x8fu, 0xbfu, 0xbfu};
	static const uint8_t lone_continuation[] = {0x80u};
	static const uint8_t overlong[] = {0xc0u, 0x80u};
	static const uint8_t overlong_3[] = {0xe0u, 0x80u, 0x80u};
	static const uint8_t surrogate[] = {0xedu, 0xa0u, 0x80u};
	static const uint8_t overlong_4[] = {0xf0u, 0x80u, 0x80u, 0x80u};
	static const uint8_t above_max[] = {0xf4u, 0x90u, 0x80u, 0x80u};
	static const uint8_t invalid_starter[] = {0xf5u, 0x80u, 0x80u, 0x80u};
	static const uint8_t truncated[] = {0xe2u, 0x82u};

	CHECK(sidex_utf8_validate(ascii, sizeof(ascii)));
	CHECK(sidex_utf8_validate(min_2, sizeof(min_2)));
	CHECK(sidex_utf8_validate(max_2, sizeof(max_2)));
	CHECK(sidex_utf8_validate(min_3, sizeof(min_3)));
	CHECK(sidex_utf8_validate(before_surrogate, sizeof(before_surrogate)));
	CHECK(sidex_utf8_validate(after_surrogate, sizeof(after_surrogate)));
	CHECK(sidex_utf8_validate(min_4, sizeof(min_4)));
	CHECK(sidex_utf8_validate(max_4, sizeof(max_4)));
	CHECK(!sidex_utf8_validate(lone_continuation, sizeof(lone_continuation)));
	CHECK(!sidex_utf8_validate(overlong, sizeof(overlong)));
	CHECK(!sidex_utf8_validate(overlong_3, sizeof(overlong_3)));
	CHECK(!sidex_utf8_validate(surrogate, sizeof(surrogate)));
	CHECK(!sidex_utf8_validate(overlong_4, sizeof(overlong_4)));
	CHECK(!sidex_utf8_validate(above_max, sizeof(above_max)));
	CHECK(!sidex_utf8_validate(invalid_starter, sizeof(invalid_starter)));
	CHECK(!sidex_utf8_validate(truncated, sizeof(truncated)));
	return 0;
}

static int test_writer(void)
{
	static const uint8_t zero[] = {'a', 0, 'b'};
	static const char expected[] =
	    "{\"name\":\"Sidex\",\"line\":\"a\\nb\",\"zero\":\"a\\u0000b\","
	    "\"ok\":true,\"n\":-42,\"u\":42}";
	uint8_t storage[160];
	sidex_json_buffer buffer;
	sidex_json_writer writer;
	uint8_t small_storage[4];
	sidex_json_buffer small_buffer;
	sidex_json_writer small_writer;
	static const uint8_t invalid_utf8[] = {0xc0u, 0x80u};

	sidex_json_buffer_init(&buffer, storage, sizeof(storage));
	writer = sidex_json_writer_for_buffer(&buffer);

	CHECK_OK(sidex_json_write_object_begin(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("name")));
	CHECK_OK(sidex_json_write_str(&writer, sidex_str_from_cstr("Sidex")));
	CHECK_OK(sidex_json_write_value_separator(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("line")));
	CHECK_OK(sidex_json_write_str(&writer, sidex_str_from_cstr("a\nb")));
	CHECK_OK(sidex_json_write_value_separator(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("zero")));
	CHECK_OK(sidex_json_write_str(&writer, sidex_str_from_parts(zero, sizeof(zero))));
	CHECK_OK(sidex_json_write_value_separator(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("ok")));
	CHECK_OK(sidex_json_write_bool(&writer, true));
	CHECK_OK(sidex_json_write_value_separator(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("n")));
	CHECK_OK(sidex_json_write_i64(&writer, -42));
	CHECK_OK(sidex_json_write_value_separator(&writer));
	CHECK_OK(sidex_json_write_object_key(&writer, sidex_str_from_cstr("u")));
	CHECK_OK(sidex_json_write_u64(&writer, 42));
	CHECK_OK(sidex_json_write_object_end(&writer));

	CHECK(buffer.len == strlen(expected));
	CHECK(memcmp(storage, expected, buffer.len) == 0);

	sidex_json_buffer_init(&small_buffer, small_storage, sizeof(small_storage));
	small_writer = sidex_json_writer_for_buffer(&small_buffer);
	CHECK_RC(sidex_json_write_str(&small_writer, sidex_str_from_cstr("abcd")),
	         SIDEX_ERR_NO_SPACE);
	CHECK_RC(
	    sidex_json_write_str(&writer, sidex_str_from_parts(invalid_utf8, sizeof(invalid_utf8))),
	    SIDEX_ERR_UTF8);
	return 0;
}

static int test_reader(void)
{
	static const char input[] =
	    "{\"name\":\"Sidex\",\"line\":\"a\\nb\","
	    "\"unicode\":\"\\u00df\\ud83d\\ude80\",\"zero\":\"a\\u0000b\","
	    "\"n\":-42,\"u\":42,\"ok\":true,"
	    "\"ignored\":{\"x\":[1,2.5,null,\"long escaped \\u00df string\"]}}";
	static const uint8_t unicode_expected[] = {0xc3u, 0x9fu, 0xf0u, 0x9fu, 0x9au, 0x80u};
	static const uint8_t zero_expected[] = {'a', 0, 'b'};
	sidex_json_reader reader;
	sidex_json_token_kind kind;
	uint8_t arena_storage[64];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_str value;
	int64_t n;
	uint64_t u;
	bool ok;

	sidex_json_reader_init(&reader, (const uint8_t *)input, strlen(input));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_OBJECT_BEGIN));

	CHECK_OK(read_key(&reader, "name"));
	CHECK_OK(sidex_json_read_str(&reader, &value, &arena));
	CHECK(str_eq_cstr(value, "Sidex"));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "line"));
	CHECK_OK(sidex_json_read_str(&reader, &value, &arena));
	CHECK(str_eq_cstr(value, "a\nb"));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "unicode"));
	CHECK_OK(sidex_json_read_str(&reader, &value, &arena));
	CHECK(bytes_eq(value, unicode_expected, sizeof(unicode_expected)));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "zero"));
	CHECK_OK(sidex_json_read_str(&reader, &value, &arena));
	CHECK(bytes_eq(value, zero_expected, sizeof(zero_expected)));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "n"));
	CHECK_OK(sidex_json_read_i64(&reader, &n));
	CHECK(n == -42);
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "u"));
	CHECK_OK(sidex_json_read_u64(&reader, &u));
	CHECK(u == 42);
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "ok"));
	CHECK_OK(sidex_json_read_bool(&reader, &ok));
	CHECK(ok);
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));

	CHECK_OK(read_key(&reader, "ignored"));
	CHECK_OK(sidex_json_skip_value(&reader, SIDEX_JSON_DEFAULT_MAX_DEPTH));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_OBJECT_END));
	CHECK_OK(sidex_json_peek(&reader, &kind));
	CHECK(kind == SIDEX_JSON_TOKEN_END);
	return 0;
}

static int test_reader_errors(void)
{
	static const uint8_t invalid_utf8_string[] = {'"', 0xc0u, 0x80u, '"'};
	static const char fractional_integer[] = "1.25";
	static const char bad_surrogate[] = "\"\\ud83d\"";
	sidex_json_reader reader;
	uint8_t arena_storage[16];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_str value;
	int64_t number;

	sidex_json_reader_init(&reader, invalid_utf8_string, sizeof(invalid_utf8_string));
	CHECK_RC(sidex_json_read_str(&reader, &value, &arena), SIDEX_ERR_UTF8);

	sidex_json_reader_init(&reader, (const uint8_t *)fractional_integer,
	                       strlen(fractional_integer));
	CHECK_RC(sidex_json_read_i64(&reader, &number), SIDEX_ERR_UNSUPPORTED);

	sidex_json_reader_init(&reader, (const uint8_t *)bad_surrogate, strlen(bad_surrogate));
	CHECK_RC(sidex_json_skip_value(&reader, SIDEX_JSON_DEFAULT_MAX_DEPTH), SIDEX_ERR_UTF8);
	return 0;
}

static int test_arena(void)
{
	static const char input[] = "[\"a\\nb\",\"c\\td\"]";
	sidex_json_reader reader;
	uint8_t arena_storage[16];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_str first;
	sidex_str second;

	sidex_json_reader_init(&reader, (const uint8_t *)input, strlen(input));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_ARRAY_BEGIN));
	CHECK_OK(sidex_json_read_str(&reader, &first, &arena));
	CHECK(first.ptr == arena_storage);
	CHECK(first.len == 3);
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR));
	CHECK_OK(sidex_json_read_str(&reader, &second, &arena));
	CHECK(second.ptr == arena_storage + first.len);
	CHECK(second.len == 3);
	CHECK(arena.len == first.len + second.len);
	CHECK(str_eq_cstr(first, "a\nb"));
	CHECK(str_eq_cstr(second, "c\td"));
	CHECK_OK(sidex_json_expect(&reader, SIDEX_JSON_TOKEN_ARRAY_END));

	sidex_arena_reset(&arena);
	CHECK(arena.len == 0);
	return 0;
}

static int test_json_value(void)
{
	static const char input[] =
	    "{\"a\":[true,null,{\"b\":\"x\\ny\"}],\"n\":-12.5e+2,\"s\":\"\\u00df\"}";
	static const char expected[] =
	    "{\"a\":[true,null,{\"b\":\"x\\ny\"}],\"n\":-12.5e+2,\"s\":\"\xc3\x9f\"}";
	uint8_t arena_storage[512];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_json_reader reader;
	sidex_json_value value;
	sidex_json_buffer buffer;
	sidex_json_writer writer;
	sidex_json_token_kind kind;
	uint8_t output[160];

	sidex_json_reader_init(&reader, (const uint8_t *)input, strlen(input));
	CHECK_OK(sidex_json_read_value(&reader, &value, &arena, SIDEX_JSON_DEFAULT_MAX_DEPTH));
	CHECK(value.kind == SIDEX_JSON_VALUE_OBJECT);
	CHECK(value.as.object.len == 3);
	CHECK(str_eq_cstr(value.as.object.entries[0].key, "a"));
	CHECK(value.as.object.entries[0].value.kind == SIDEX_JSON_VALUE_ARRAY);
	CHECK(value.as.object.entries[0].value.as.array.len == 3);
	CHECK(value.as.object.entries[0].value.as.array.items[0].kind == SIDEX_JSON_VALUE_BOOL);
	CHECK(value.as.object.entries[0].value.as.array.items[0].as.boolean);
	CHECK(value.as.object.entries[0].value.as.array.items[1].kind == SIDEX_JSON_VALUE_NULL);
	CHECK(value.as.object.entries[0].value.as.array.items[2].kind == SIDEX_JSON_VALUE_OBJECT);
	CHECK(str_eq_cstr(value.as.object.entries[1].key, "n"));
	CHECK(value.as.object.entries[1].value.kind == SIDEX_JSON_VALUE_NUMBER);
	CHECK(str_eq_cstr(value.as.object.entries[1].value.as.number, "-12.5e+2"));
	CHECK(str_eq_cstr(value.as.object.entries[2].key, "s"));
	CHECK(value.as.object.entries[2].value.kind == SIDEX_JSON_VALUE_STRING);
	CHECK(bytes_eq(value.as.object.entries[2].value.as.string, (const uint8_t *)"\xc3\x9f", 2));
	CHECK_OK(sidex_json_peek(&reader, &kind));
	CHECK(kind == SIDEX_JSON_TOKEN_END);

	sidex_json_buffer_init(&buffer, output, sizeof(output));
	writer = sidex_json_writer_for_buffer(&buffer);
	CHECK_OK(sidex_json_write_value(&writer, &value, SIDEX_JSON_DEFAULT_MAX_DEPTH));
	CHECK(buffer.len == strlen(expected));
	CHECK(memcmp(output, expected, buffer.len) == 0);
	return 0;
}

static int test_json_value_borrowing(void)
{
	static const char input[] = "{\"plain\":\"text\",\"escaped\":\"a\\nb\",\"number\":123}";
	const uint8_t *input_ptr = (const uint8_t *)input;
	size_t input_len = strlen(input);
	uint8_t arena_storage[128];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_json_reader reader;
	sidex_json_value value;

	sidex_json_reader_init(&reader, input_ptr, input_len);
	CHECK_OK(sidex_json_read_value(&reader, &value, &arena, SIDEX_JSON_DEFAULT_MAX_DEPTH));
	CHECK(value.kind == SIDEX_JSON_VALUE_OBJECT);
	CHECK(value.as.object.len == 3);
	CHECK(ptr_in_range(value.as.object.entries[0].key.ptr, input_ptr, input_len));
	CHECK(ptr_in_range(value.as.object.entries[0].value.as.string.ptr, input_ptr, input_len));
	CHECK(ptr_in_range(value.as.object.entries[1].value.as.string.ptr, arena_storage,
	                   sizeof(arena_storage)));
	CHECK(!ptr_in_range(value.as.object.entries[1].value.as.string.ptr, input_ptr, input_len));
	CHECK(ptr_in_range(value.as.object.entries[2].value.as.number.ptr, input_ptr, input_len));
	return 0;
}

static int test_depth_limits(void)
{
	static const char nested[] = "[0]";
	static const sidex_json_value number = {
	    SIDEX_JSON_VALUE_NUMBER,
	    {.number = {(const uint8_t *)"0", 1}},
	};
	sidex_json_value items[1];
	sidex_json_value array;
	uint8_t arena_storage[64];
	uint8_t output[8];
	sidex_arena arena = sidex_arena_from_parts(arena_storage, sizeof(arena_storage));
	sidex_json_reader reader;
	sidex_json_buffer buffer;
	sidex_json_writer writer;

	sidex_json_reader_init(&reader, (const uint8_t *)nested, strlen(nested));
	CHECK_RC(sidex_json_skip_value(&reader, 1), SIDEX_ERR_DEPTH);
	sidex_json_reader_init(&reader, (const uint8_t *)nested, strlen(nested));
	CHECK_OK(sidex_json_skip_value(&reader, 2));

	sidex_json_reader_init(&reader, (const uint8_t *)nested, strlen(nested));
	CHECK_RC(sidex_json_read_value(&reader, &array, &arena, 1), SIDEX_ERR_DEPTH);
	sidex_json_reader_init(&reader, (const uint8_t *)nested, strlen(nested));
	CHECK_OK(sidex_json_read_value(&reader, &array, &arena, 2));

	items[0] = number;
	array.kind = SIDEX_JSON_VALUE_ARRAY;
	array.as.array.items = items;
	array.as.array.len = 1;

	sidex_json_buffer_init(&buffer, output, sizeof(output));
	writer = sidex_json_writer_for_buffer(&buffer);
	CHECK_RC(sidex_json_write_value(&writer, &array, 1), SIDEX_ERR_DEPTH);

	sidex_json_buffer_init(&buffer, output, sizeof(output));
	writer = sidex_json_writer_for_buffer(&buffer);
	CHECK_OK(sidex_json_write_value(&writer, &array, 2));
	CHECK(buffer.len == 3);
	CHECK(memcmp(output, nested, buffer.len) == 0);
	return 0;
}

int main(void)
{
	int rc;

	rc = test_utf8();
	if (rc != 0) {
		fprintf(stderr, "test_utf8 failed at line %d\n", rc);
		return rc;
	}
	rc = test_writer();
	if (rc != 0) {
		fprintf(stderr, "test_writer failed at line %d\n", rc);
		return rc;
	}
	rc = test_reader();
	if (rc != 0) {
		fprintf(stderr, "test_reader failed at line %d\n", rc);
		return rc;
	}
	rc = test_reader_errors();
	if (rc != 0) {
		fprintf(stderr, "test_reader_errors failed at line %d\n", rc);
		return rc;
	}
	rc = test_arena();
	if (rc != 0) {
		fprintf(stderr, "test_arena failed at line %d\n", rc);
		return rc;
	}
	rc = test_json_value();
	if (rc != 0) {
		fprintf(stderr, "test_json_value failed at line %d\n", rc);
		return rc;
	}
	rc = test_json_value_borrowing();
	if (rc != 0) {
		fprintf(stderr, "test_json_value_borrowing failed at line %d\n", rc);
		return rc;
	}
	rc = test_depth_limits();
	if (rc != 0) {
		fprintf(stderr, "test_depth_limits failed at line %d\n", rc);
		return rc;
	}
	return 0;
}

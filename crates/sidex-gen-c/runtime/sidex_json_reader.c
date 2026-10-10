#include "sidex_json_reader.h"

#include <limits.h>
#include <string.h>

static bool is_ws(uint8_t ch)
{
	return ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t';
}

static bool is_digit(uint8_t ch)
{
	return ch >= '0' && ch <= '9';
}

static void skip_ws(sidex_json_reader *reader)
{
	while (reader->pos < reader->len && is_ws(reader->ptr[reader->pos])) {
		reader->pos++;
	}
}

static int expect_literal(sidex_json_reader *reader, const char *literal, size_t len)
{
	skip_ws(reader);
	if (reader->len - reader->pos < len) {
		return SIDEX_ERR_EOF;
	}
	if (memcmp(reader->ptr + reader->pos, literal, len) != 0) {
		return SIDEX_ERR_EXPECTED;
	}
	reader->pos += len;
	return SIDEX_OK;
}

static int hex_digit(uint8_t ch)
{
	if (ch >= '0' && ch <= '9') {
		return (int)(ch - '0');
	}
	if (ch >= 'a' && ch <= 'f') {
		return 10 + (int)(ch - 'a');
	}
	if (ch >= 'A' && ch <= 'F') {
		return 10 + (int)(ch - 'A');
	}
	return -1;
}

static int parse_hex4(const uint8_t *ptr, uint32_t *out)
{
	uint32_t value = 0;

	for (size_t i = 0; i < 4; i++) {
		int digit = hex_digit(ptr[i]);

		if (digit < 0) {
			return SIDEX_ERR_SYNTAX;
		}
		value = (value << 4) | (uint32_t)digit;
	}
	*out = value;
	return SIDEX_OK;
}

static int arena_append(sidex_arena *arena, uint8_t byte)
{
	if (arena == NULL || arena->ptr == NULL || arena->len >= arena->cap) {
		return SIDEX_ERR_NO_SPACE;
	}
	arena->ptr[arena->len] = byte;
	arena->len += 1;
	return SIDEX_OK;
}

static int arena_append_slice(sidex_arena *arena, const uint8_t *ptr, size_t len)
{
	if (len == 0) {
		return SIDEX_OK;
	}
	if (arena == NULL || arena->ptr == NULL || arena->len > arena->cap ||
	    len > arena->cap - arena->len) {
		return SIDEX_ERR_NO_SPACE;
	}
	memcpy(arena->ptr + arena->len, ptr, len);
	arena->len += len;
	return SIDEX_OK;
}

static int arena_append_utf8(sidex_arena *arena, uint32_t scalar)
{
	if (scalar <= 0x7fu) {
		return arena_append(arena, (uint8_t)scalar);
	}
	if (scalar <= 0x7ffu) {
		int rc = arena_append(arena, (uint8_t)(0xc0u | (scalar >> 6)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		return arena_append(arena, (uint8_t)(0x80u | (scalar & 0x3fu)));
	}
	if (scalar >= 0xd800u && scalar <= 0xdfffu) {
		return SIDEX_ERR_UTF8;
	}
	if (scalar <= 0xffffu) {
		int rc = arena_append(arena, (uint8_t)(0xe0u | (scalar >> 12)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = arena_append(arena, (uint8_t)(0x80u | ((scalar >> 6) & 0x3fu)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		return arena_append(arena, (uint8_t)(0x80u | (scalar & 0x3fu)));
	}
	if (scalar <= 0x10ffffu) {
		int rc = arena_append(arena, (uint8_t)(0xf0u | (scalar >> 18)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = arena_append(arena, (uint8_t)(0x80u | ((scalar >> 12) & 0x3fu)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = arena_append(arena, (uint8_t)(0x80u | ((scalar >> 6) & 0x3fu)));
		if (rc != SIDEX_OK) {
			return rc;
		}
		return arena_append(arena, (uint8_t)(0x80u | (scalar & 0x3fu)));
	}
	return SIDEX_ERR_UTF8;
}

static int arena_rollback(sidex_arena *arena, size_t arena_start, int rc)
{
	if (arena != NULL && arena->len >= arena_start) {
		arena->len = arena_start;
	}
	return rc;
}

void sidex_json_reader_init(sidex_json_reader *reader, const uint8_t *ptr, size_t len)
{
	reader->ptr = ptr;
	reader->len = len;
	reader->pos = 0;
}

int sidex_json_peek(sidex_json_reader *reader, sidex_json_token_kind *kind)
{
	skip_ws(reader);
	if (reader->pos >= reader->len) {
		*kind = SIDEX_JSON_TOKEN_END;
		return SIDEX_OK;
	}

	switch (reader->ptr[reader->pos]) {
	case '{':
		*kind = SIDEX_JSON_TOKEN_OBJECT_BEGIN;
		return SIDEX_OK;
	case '}':
		*kind = SIDEX_JSON_TOKEN_OBJECT_END;
		return SIDEX_OK;
	case '[':
		*kind = SIDEX_JSON_TOKEN_ARRAY_BEGIN;
		return SIDEX_OK;
	case ']':
		*kind = SIDEX_JSON_TOKEN_ARRAY_END;
		return SIDEX_OK;
	case ':':
		*kind = SIDEX_JSON_TOKEN_NAME_SEPARATOR;
		return SIDEX_OK;
	case ',':
		*kind = SIDEX_JSON_TOKEN_VALUE_SEPARATOR;
		return SIDEX_OK;
	case '"':
		*kind = SIDEX_JSON_TOKEN_STRING;
		return SIDEX_OK;
	case 't':
		*kind = SIDEX_JSON_TOKEN_TRUE;
		return SIDEX_OK;
	case 'f':
		*kind = SIDEX_JSON_TOKEN_FALSE;
		return SIDEX_OK;
	case 'n':
		*kind = SIDEX_JSON_TOKEN_NULL;
		return SIDEX_OK;
	case '-':
		*kind = SIDEX_JSON_TOKEN_NUMBER;
		return SIDEX_OK;
	default:
		if (is_digit(reader->ptr[reader->pos])) {
			*kind = SIDEX_JSON_TOKEN_NUMBER;
			return SIDEX_OK;
		}
		return SIDEX_ERR_SYNTAX;
	}
}

int sidex_json_expect(sidex_json_reader *reader, sidex_json_token_kind kind)
{
	sidex_json_token_kind actual;
	int rc = sidex_json_peek(reader, &actual);

	if (rc != SIDEX_OK) {
		return rc;
	}
	if (actual != kind) {
		return SIDEX_ERR_EXPECTED;
	}

	switch (kind) {
	case SIDEX_JSON_TOKEN_OBJECT_BEGIN:
	case SIDEX_JSON_TOKEN_OBJECT_END:
	case SIDEX_JSON_TOKEN_ARRAY_BEGIN:
	case SIDEX_JSON_TOKEN_ARRAY_END:
	case SIDEX_JSON_TOKEN_NAME_SEPARATOR:
	case SIDEX_JSON_TOKEN_VALUE_SEPARATOR:
		reader->pos++;
		return SIDEX_OK;
	case SIDEX_JSON_TOKEN_TRUE:
		return expect_literal(reader, "true", 4);
	case SIDEX_JSON_TOKEN_FALSE:
		return expect_literal(reader, "false", 5);
	case SIDEX_JSON_TOKEN_NULL:
		return expect_literal(reader, "null", 4);
	default:
		return SIDEX_ERR_UNSUPPORTED;
	}
}

int sidex_json_read_bool(sidex_json_reader *reader, bool *out)
{
	sidex_json_token_kind kind;
	int rc = sidex_json_peek(reader, &kind);

	if (rc != SIDEX_OK) {
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_TRUE) {
		rc = expect_literal(reader, "true", 4);
		*out = true;
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_FALSE) {
		rc = expect_literal(reader, "false", 5);
		*out = false;
		return rc;
	}
	return SIDEX_ERR_EXPECTED;
}

int sidex_json_read_null(sidex_json_reader *reader)
{
	return sidex_json_expect(reader, SIDEX_JSON_TOKEN_NULL);
}

static int parse_uint_digits(sidex_json_reader *reader, uint64_t limit, uint64_t *out)
{
	uint64_t value = 0;
	size_t start = reader->pos;

	if (reader->pos >= reader->len || !is_digit(reader->ptr[reader->pos])) {
		return SIDEX_ERR_EXPECTED;
	}

	if (reader->ptr[reader->pos] == '0') {
		reader->pos++;
		if (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
			return SIDEX_ERR_SYNTAX;
		}
		*out = 0;
		return SIDEX_OK;
	}

	while (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
		uint64_t digit = (uint64_t)(reader->ptr[reader->pos] - '0');

		if (value > (limit - digit) / 10u) {
			return SIDEX_ERR_OVERFLOW;
		}
		value = value * 10u + digit;
		reader->pos++;
	}

	if (reader->pos == start) {
		return SIDEX_ERR_EXPECTED;
	}
	*out = value;
	return SIDEX_OK;
}

static int reject_fraction_or_exponent(sidex_json_reader *reader)
{
	if (reader->pos < reader->len &&
	    (reader->ptr[reader->pos] == '.' || reader->ptr[reader->pos] == 'e' ||
	     reader->ptr[reader->pos] == 'E')) {
		return SIDEX_ERR_UNSUPPORTED;
	}
	return SIDEX_OK;
}

int sidex_json_read_u64(sidex_json_reader *reader, uint64_t *out)
{
	int rc;

	skip_ws(reader);
	if (reader->pos < reader->len && reader->ptr[reader->pos] == '-') {
		return SIDEX_ERR_EXPECTED;
	}
	rc = parse_uint_digits(reader, UINT64_MAX, out);
	if (rc != SIDEX_OK) {
		return rc;
	}
	return reject_fraction_or_exponent(reader);
}

int sidex_json_read_i64(sidex_json_reader *reader, int64_t *out)
{
	bool negative = false;
	uint64_t value;
	uint64_t limit = (uint64_t)INT64_MAX;
	int rc;

	skip_ws(reader);
	if (reader->pos < reader->len && reader->ptr[reader->pos] == '-') {
		negative = true;
		reader->pos++;
		limit += 1u;
	}

	rc = parse_uint_digits(reader, limit, &value);
	if (rc != SIDEX_OK) {
		return rc;
	}
	rc = reject_fraction_or_exponent(reader);
	if (rc != SIDEX_OK) {
		return rc;
	}

	if (negative) {
		if (value == limit) {
			*out = INT64_MIN;
		} else {
			*out = -(int64_t)value;
		}
	} else {
		*out = (int64_t)value;
	}
	return SIDEX_OK;
}

static int decode_unicode_escape(sidex_json_reader *reader, sidex_arena *arena)
{
	uint32_t scalar;
	int rc;

	if (reader->len - reader->pos < 4) {
		return SIDEX_ERR_EOF;
	}
	rc = parse_hex4(reader->ptr + reader->pos, &scalar);
	if (rc != SIDEX_OK) {
		return rc;
	}
	reader->pos += 4;

	if (scalar >= 0xd800u && scalar <= 0xdbffu) {
		uint32_t low;

		if (reader->len - reader->pos < 6 || reader->ptr[reader->pos] != '\\' ||
		    reader->ptr[reader->pos + 1] != 'u') {
			return SIDEX_ERR_UTF8;
		}
		reader->pos += 2;
		rc = parse_hex4(reader->ptr + reader->pos, &low);
		if (rc != SIDEX_OK) {
			return rc;
		}
		reader->pos += 4;
		if (low < 0xdc00u || low > 0xdfffu) {
			return SIDEX_ERR_UTF8;
		}
		scalar = 0x10000u + (((scalar - 0xd800u) << 10) | (low - 0xdc00u));
	} else if (scalar >= 0xdc00u && scalar <= 0xdfffu) {
		return SIDEX_ERR_UTF8;
	}

	return arena_append_utf8(arena, scalar);
}

int sidex_json_read_str(sidex_json_reader *reader, sidex_str *out, sidex_arena *arena)
{
	size_t string_start;
	size_t copy_start;
	size_t arena_start = arena != NULL ? arena->len : 0;
	bool escaped = false;

	skip_ws(reader);
	if (reader->pos >= reader->len) {
		return arena_rollback(arena, arena_start, SIDEX_ERR_EOF);
	}
	if (reader->ptr[reader->pos] != '"') {
		return arena_rollback(arena, arena_start, SIDEX_ERR_EXPECTED);
	}
	reader->pos++;
	string_start = reader->pos;
	copy_start = reader->pos;

	while (reader->pos < reader->len) {
		uint8_t ch = reader->ptr[reader->pos];

		if (ch == '"') {
			const uint8_t *ptr;
			size_t len;

			if (escaped) {
				int rc = arena_append_slice(arena, reader->ptr + copy_start,
				                            reader->pos - copy_start);
				if (rc != SIDEX_OK) {
					return arena_rollback(arena, arena_start, rc);
				}
				ptr = arena->ptr + arena_start;
				len = arena->len - arena_start;
			} else {
				ptr = reader->ptr + string_start;
				len = reader->pos - string_start;
			}
			if (!sidex_utf8_validate(ptr, len)) {
				return arena_rollback(arena, arena_start, SIDEX_ERR_UTF8);
			}
			reader->pos++;
			*out = sidex_str_from_parts(ptr, len);
			return SIDEX_OK;
		}

		if (ch < 0x20u) {
			return arena_rollback(arena, arena_start, SIDEX_ERR_SYNTAX);
		}

		if (ch != '\\') {
			reader->pos++;
			continue;
		}

		escaped = true;
		int rc =
		    arena_append_slice(arena, reader->ptr + copy_start, reader->pos - copy_start);
		if (rc != SIDEX_OK) {
			return arena_rollback(arena, arena_start, rc);
		}

		reader->pos++;
		if (reader->pos >= reader->len) {
			return arena_rollback(arena, arena_start, SIDEX_ERR_EOF);
		}

		ch = reader->ptr[reader->pos++];
		switch (ch) {
		case '"':
		case '\\':
		case '/':
			rc = arena_append(arena, ch);
			break;
		case 'b':
			rc = arena_append(arena, '\b');
			break;
		case 'f':
			rc = arena_append(arena, '\f');
			break;
		case 'n':
			rc = arena_append(arena, '\n');
			break;
		case 'r':
			rc = arena_append(arena, '\r');
			break;
		case 't':
			rc = arena_append(arena, '\t');
			break;
		case 'u':
			rc = decode_unicode_escape(reader, arena);
			break;
		default:
			return arena_rollback(arena, arena_start, SIDEX_ERR_SYNTAX);
		}
		if (rc != SIDEX_OK) {
			return arena_rollback(arena, arena_start, rc);
		}
		copy_start = reader->pos;
	}

	return arena_rollback(arena, arena_start, SIDEX_ERR_EOF);
}

static int skip_unicode_escape(sidex_json_reader *reader)
{
	uint32_t scalar;
	int rc;

	if (reader->len - reader->pos < 4) {
		return SIDEX_ERR_EOF;
	}
	rc = parse_hex4(reader->ptr + reader->pos, &scalar);
	if (rc != SIDEX_OK) {
		return rc;
	}
	reader->pos += 4;

	if (scalar >= 0xd800u && scalar <= 0xdbffu) {
		uint32_t low;

		if (reader->len - reader->pos < 6 || reader->ptr[reader->pos] != '\\' ||
		    reader->ptr[reader->pos + 1] != 'u') {
			return SIDEX_ERR_UTF8;
		}
		reader->pos += 2;
		rc = parse_hex4(reader->ptr + reader->pos, &low);
		if (rc != SIDEX_OK) {
			return rc;
		}
		reader->pos += 4;
		if (low < 0xdc00u || low > 0xdfffu) {
			return SIDEX_ERR_UTF8;
		}
	} else if (scalar >= 0xdc00u && scalar <= 0xdfffu) {
		return SIDEX_ERR_UTF8;
	}

	return SIDEX_OK;
}

static int skip_string_token(sidex_json_reader *reader)
{
	size_t chunk_start;

	skip_ws(reader);
	if (reader->pos >= reader->len) {
		return SIDEX_ERR_EOF;
	}
	if (reader->ptr[reader->pos] != '"') {
		return SIDEX_ERR_EXPECTED;
	}
	reader->pos++;
	chunk_start = reader->pos;

	while (reader->pos < reader->len) {
		uint8_t ch = reader->ptr[reader->pos];

		if (ch == '"') {
			if (!sidex_utf8_validate(reader->ptr + chunk_start,
			                         reader->pos - chunk_start)) {
				return SIDEX_ERR_UTF8;
			}
			reader->pos++;
			return SIDEX_OK;
		}
		if (ch < 0x20u) {
			return SIDEX_ERR_SYNTAX;
		}
		if (ch != '\\') {
			reader->pos++;
			continue;
		}

		if (!sidex_utf8_validate(reader->ptr + chunk_start, reader->pos - chunk_start)) {
			return SIDEX_ERR_UTF8;
		}
		reader->pos++;
		if (reader->pos >= reader->len) {
			return SIDEX_ERR_EOF;
		}

		ch = reader->ptr[reader->pos++];
		switch (ch) {
		case '"':
		case '\\':
		case '/':
		case 'b':
		case 'f':
		case 'n':
		case 'r':
		case 't':
			break;
		case 'u': {
			int rc = skip_unicode_escape(reader);
			if (rc != SIDEX_OK) {
				return rc;
			}
			break;
		}
		default:
			return SIDEX_ERR_SYNTAX;
		}
		chunk_start = reader->pos;
	}

	return SIDEX_ERR_EOF;
}

int sidex_json_read_number(sidex_json_reader *reader, sidex_str *out)
{
	size_t start;

	skip_ws(reader);
	start = reader->pos;
	if (reader->pos < reader->len && reader->ptr[reader->pos] == '-') {
		reader->pos++;
	}
	if (reader->pos >= reader->len || !is_digit(reader->ptr[reader->pos])) {
		return SIDEX_ERR_EXPECTED;
	}
	if (reader->ptr[reader->pos] == '0') {
		reader->pos++;
		if (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
			return SIDEX_ERR_SYNTAX;
		}
	} else {
		while (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
			reader->pos++;
		}
	}
	if (reader->pos < reader->len && reader->ptr[reader->pos] == '.') {
		reader->pos++;
		if (reader->pos >= reader->len || !is_digit(reader->ptr[reader->pos])) {
			return SIDEX_ERR_SYNTAX;
		}
		while (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
			reader->pos++;
		}
	}
	if (reader->pos < reader->len &&
	    (reader->ptr[reader->pos] == 'e' || reader->ptr[reader->pos] == 'E')) {
		reader->pos++;
		if (reader->pos < reader->len &&
		    (reader->ptr[reader->pos] == '+' || reader->ptr[reader->pos] == '-')) {
			reader->pos++;
		}
		if (reader->pos >= reader->len || !is_digit(reader->ptr[reader->pos])) {
			return SIDEX_ERR_SYNTAX;
		}
		while (reader->pos < reader->len && is_digit(reader->ptr[reader->pos])) {
			reader->pos++;
		}
	}
	*out = sidex_str_from_parts(reader->ptr + start, reader->pos - start);
	return SIDEX_OK;
}

static int skip_number(sidex_json_reader *reader)
{
	sidex_str ignored;

	return sidex_json_read_number(reader, &ignored);
}

static int skip_value_with_depth(sidex_json_reader *reader, size_t max_depth);

static int skip_array(sidex_json_reader *reader, size_t max_depth)
{
	sidex_json_token_kind kind;
	int rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_ARRAY_BEGIN);

	if (rc != SIDEX_OK) {
		return rc;
	}
	rc = sidex_json_peek(reader, &kind);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_ARRAY_END) {
		return sidex_json_expect(reader, SIDEX_JSON_TOKEN_ARRAY_END);
	}

	while (true) {
		rc = skip_value_with_depth(reader, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_peek(reader, &kind);
		if (rc != SIDEX_OK) {
			return rc;
		}
		if (kind == SIDEX_JSON_TOKEN_ARRAY_END) {
			return sidex_json_expect(reader, SIDEX_JSON_TOKEN_ARRAY_END);
		}
		if (kind != SIDEX_JSON_TOKEN_VALUE_SEPARATOR) {
			return SIDEX_ERR_EXPECTED;
		}
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
}

static int skip_object(sidex_json_reader *reader, size_t max_depth)
{
	sidex_json_token_kind kind;
	int rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_OBJECT_BEGIN);

	if (rc != SIDEX_OK) {
		return rc;
	}
	rc = sidex_json_peek(reader, &kind);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_OBJECT_END) {
		return sidex_json_expect(reader, SIDEX_JSON_TOKEN_OBJECT_END);
	}

	while (true) {
		rc = skip_string_token(reader);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_NAME_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = skip_value_with_depth(reader, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_peek(reader, &kind);
		if (rc != SIDEX_OK) {
			return rc;
		}
		if (kind == SIDEX_JSON_TOKEN_OBJECT_END) {
			return sidex_json_expect(reader, SIDEX_JSON_TOKEN_OBJECT_END);
		}
		if (kind != SIDEX_JSON_TOKEN_VALUE_SEPARATOR) {
			return SIDEX_ERR_EXPECTED;
		}
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
}

static int skip_value_with_depth(sidex_json_reader *reader, size_t max_depth)
{
	sidex_json_token_kind kind;
	int rc;

	if (max_depth == 0) {
		return SIDEX_ERR_DEPTH;
	}

	rc = sidex_json_peek(reader, &kind);
	if (rc != SIDEX_OK) {
		return rc;
	}
	switch (kind) {
	case SIDEX_JSON_TOKEN_OBJECT_BEGIN:
		return skip_object(reader, max_depth);
	case SIDEX_JSON_TOKEN_ARRAY_BEGIN:
		return skip_array(reader, max_depth);
	case SIDEX_JSON_TOKEN_STRING:
		return skip_string_token(reader);
	case SIDEX_JSON_TOKEN_NUMBER:
		return skip_number(reader);
	case SIDEX_JSON_TOKEN_TRUE:
		return sidex_json_expect(reader, SIDEX_JSON_TOKEN_TRUE);
	case SIDEX_JSON_TOKEN_FALSE:
		return sidex_json_expect(reader, SIDEX_JSON_TOKEN_FALSE);
	case SIDEX_JSON_TOKEN_NULL:
		return sidex_json_expect(reader, SIDEX_JSON_TOKEN_NULL);
	default:
		return SIDEX_ERR_EXPECTED;
	}
}

int sidex_json_skip_value(sidex_json_reader *reader, size_t max_depth)
{
	return skip_value_with_depth(reader, max_depth);
}

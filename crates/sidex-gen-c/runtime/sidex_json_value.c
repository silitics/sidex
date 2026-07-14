#include "sidex_json_value.h"

static int value_rollback(sidex_arena *arena, size_t arena_start, int rc)
{
	if (arena != NULL && arena->len >= arena_start) {
		arena->len = arena_start;
	}
	return rc;
}

static int read_value_with_depth(sidex_json_reader *reader, sidex_json_value *out,
                                 sidex_arena *arena, size_t max_depth);

static int write_value_with_depth(sidex_json_writer *writer, const sidex_json_value *value,
                                  size_t max_depth);

static int count_array_items(const sidex_json_reader *reader, size_t *out, size_t max_depth)
{
	sidex_json_reader counter = *reader;
	sidex_json_token_kind kind;
	size_t count = 0;
	int rc;

	if (max_depth == 0) {
		return SIDEX_ERR_DEPTH;
	}

	rc = sidex_json_expect(&counter, SIDEX_JSON_TOKEN_ARRAY_BEGIN);
	if (rc != SIDEX_OK) {
		return rc;
	}
	rc = sidex_json_peek(&counter, &kind);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_ARRAY_END) {
		*out = 0;
		return SIDEX_OK;
	}

	while (true) {
		if (count == SIZE_MAX) {
			return SIDEX_ERR_OVERFLOW;
		}
		count++;
		rc = sidex_json_skip_value(&counter, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_peek(&counter, &kind);
		if (rc != SIDEX_OK) {
			return rc;
		}
		if (kind == SIDEX_JSON_TOKEN_ARRAY_END) {
			*out = count;
			return SIDEX_OK;
		}
		if (kind != SIDEX_JSON_TOKEN_VALUE_SEPARATOR) {
			return SIDEX_ERR_EXPECTED;
		}
		rc = sidex_json_expect(&counter, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
}

static int count_object_entries(const sidex_json_reader *reader, size_t *out, size_t max_depth)
{
	sidex_json_reader counter = *reader;
	sidex_json_token_kind kind;
	size_t count = 0;
	int rc;

	if (max_depth == 0) {
		return SIDEX_ERR_DEPTH;
	}

	rc = sidex_json_expect(&counter, SIDEX_JSON_TOKEN_OBJECT_BEGIN);
	if (rc != SIDEX_OK) {
		return rc;
	}
	rc = sidex_json_peek(&counter, &kind);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (kind == SIDEX_JSON_TOKEN_OBJECT_END) {
		*out = 0;
		return SIDEX_OK;
	}

	while (true) {
		if (count == SIZE_MAX) {
			return SIDEX_ERR_OVERFLOW;
		}
		count++;
		rc = sidex_json_peek(&counter, &kind);
		if (rc != SIDEX_OK) {
			return rc;
		}
		if (kind != SIDEX_JSON_TOKEN_STRING) {
			return SIDEX_ERR_EXPECTED;
		}
		rc = sidex_json_skip_value(&counter, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_expect(&counter, SIDEX_JSON_TOKEN_NAME_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_skip_value(&counter, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_peek(&counter, &kind);
		if (rc != SIDEX_OK) {
			return rc;
		}
		if (kind == SIDEX_JSON_TOKEN_OBJECT_END) {
			*out = count;
			return SIDEX_OK;
		}
		if (kind != SIDEX_JSON_TOKEN_VALUE_SEPARATOR) {
			return SIDEX_ERR_EXPECTED;
		}
		rc = sidex_json_expect(&counter, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
}

static int read_array_value(sidex_json_reader *reader, sidex_json_value *out, sidex_arena *arena,
                            size_t max_depth)
{
	size_t len;
	sidex_json_value *items = NULL;
	int rc;

	rc = count_array_items(reader, &len, max_depth);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (len > 0) {
		items = (sidex_json_value *)sidex_arena_alloc_array(
		    arena, len, sizeof(sidex_json_value), SIDEX_ALIGNOF(sidex_json_value));
		if (items == NULL) {
			return SIDEX_ERR_NO_SPACE;
		}
	}

	rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_ARRAY_BEGIN);
	if (rc != SIDEX_OK) {
		return rc;
	}
	for (size_t i = 0; i < len; i++) {
		if (i > 0) {
			rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
			if (rc != SIDEX_OK) {
				return rc;
			}
		}
		rc = read_value_with_depth(reader, &items[i], arena, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
	rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_ARRAY_END);
	if (rc != SIDEX_OK) {
		return rc;
	}

	out->kind = SIDEX_JSON_VALUE_ARRAY;
	out->as.array.items = items;
	out->as.array.len = len;
	return SIDEX_OK;
}

static int read_object_value(sidex_json_reader *reader, sidex_json_value *out, sidex_arena *arena,
                             size_t max_depth)
{
	size_t len;
	sidex_json_entry *entries = NULL;
	int rc;

	rc = count_object_entries(reader, &len, max_depth);
	if (rc != SIDEX_OK) {
		return rc;
	}
	if (len > 0) {
		entries = (sidex_json_entry *)sidex_arena_alloc_array(
		    arena, len, sizeof(sidex_json_entry), SIDEX_ALIGNOF(sidex_json_entry));
		if (entries == NULL) {
			return SIDEX_ERR_NO_SPACE;
		}
	}

	rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_OBJECT_BEGIN);
	if (rc != SIDEX_OK) {
		return rc;
	}
	for (size_t i = 0; i < len; i++) {
		if (i > 0) {
			rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_VALUE_SEPARATOR);
			if (rc != SIDEX_OK) {
				return rc;
			}
		}
		rc = sidex_json_read_str(reader, &entries[i].key, arena);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_NAME_SEPARATOR);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = read_value_with_depth(reader, &entries[i].value, arena, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
	rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_OBJECT_END);
	if (rc != SIDEX_OK) {
		return rc;
	}

	out->kind = SIDEX_JSON_VALUE_OBJECT;
	out->as.object.entries = entries;
	out->as.object.len = len;
	return SIDEX_OK;
}

static int read_value_with_depth(sidex_json_reader *reader, sidex_json_value *out,
                                 sidex_arena *arena, size_t max_depth)
{
	sidex_json_token_kind kind;
	size_t arena_start = arena != NULL ? arena->len : 0;
	int rc;

	if (max_depth == 0) {
		return value_rollback(arena, arena_start, SIDEX_ERR_DEPTH);
	}

	rc = sidex_json_peek(reader, &kind);
	if (rc != SIDEX_OK) {
		return value_rollback(arena, arena_start, rc);
	}

	switch (kind) {
	case SIDEX_JSON_TOKEN_OBJECT_BEGIN:
		rc = read_object_value(reader, out, arena, max_depth);
		break;
	case SIDEX_JSON_TOKEN_ARRAY_BEGIN:
		rc = read_array_value(reader, out, arena, max_depth);
		break;
	case SIDEX_JSON_TOKEN_STRING:
		out->kind = SIDEX_JSON_VALUE_STRING;
		rc = sidex_json_read_str(reader, &out->as.string, arena);
		break;
	case SIDEX_JSON_TOKEN_NUMBER:
		out->kind = SIDEX_JSON_VALUE_NUMBER;
		rc = sidex_json_read_number(reader, &out->as.number);
		break;
	case SIDEX_JSON_TOKEN_TRUE:
		out->kind = SIDEX_JSON_VALUE_BOOL;
		out->as.boolean = true;
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_TRUE);
		break;
	case SIDEX_JSON_TOKEN_FALSE:
		out->kind = SIDEX_JSON_VALUE_BOOL;
		out->as.boolean = false;
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_FALSE);
		break;
	case SIDEX_JSON_TOKEN_NULL:
		out->kind = SIDEX_JSON_VALUE_NULL;
		rc = sidex_json_expect(reader, SIDEX_JSON_TOKEN_NULL);
		break;
	default:
		rc = SIDEX_ERR_EXPECTED;
		break;
	}

	return rc == SIDEX_OK ? SIDEX_OK : value_rollback(arena, arena_start, rc);
}

int sidex_json_read_value(sidex_json_reader *reader, sidex_json_value *out, sidex_arena *arena,
                          size_t max_depth)
{
	return read_value_with_depth(reader, out, arena, max_depth);
}

static bool is_digit(uint8_t ch)
{
	return ch >= '0' && ch <= '9';
}

static bool number_lexeme_is_valid(sidex_str number)
{
	size_t pos = 0;

	if (pos < number.len && number.ptr[pos] == '-') {
		pos++;
	}
	if (pos >= number.len || !is_digit(number.ptr[pos])) {
		return false;
	}
	if (number.ptr[pos] == '0') {
		pos++;
		if (pos < number.len && is_digit(number.ptr[pos])) {
			return false;
		}
	} else {
		while (pos < number.len && is_digit(number.ptr[pos])) {
			pos++;
		}
	}
	if (pos < number.len && number.ptr[pos] == '.') {
		pos++;
		if (pos >= number.len || !is_digit(number.ptr[pos])) {
			return false;
		}
		while (pos < number.len && is_digit(number.ptr[pos])) {
			pos++;
		}
	}
	if (pos < number.len && (number.ptr[pos] == 'e' || number.ptr[pos] == 'E')) {
		pos++;
		if (pos < number.len && (number.ptr[pos] == '+' || number.ptr[pos] == '-')) {
			pos++;
		}
		if (pos >= number.len || !is_digit(number.ptr[pos])) {
			return false;
		}
		while (pos < number.len && is_digit(number.ptr[pos])) {
			pos++;
		}
	}

	return pos == number.len;
}

static int write_array_value(sidex_json_writer *writer, const sidex_json_array *array,
                             size_t max_depth)
{
	int rc = sidex_json_write_array_begin(writer);

	if (rc != SIDEX_OK) {
		return rc;
	}
	for (size_t i = 0; i < array->len; i++) {
		if (i > 0) {
			rc = sidex_json_write_value_separator(writer);
			if (rc != SIDEX_OK) {
				return rc;
			}
		}
		rc = write_value_with_depth(writer, &array->items[i], max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
	return sidex_json_write_array_end(writer);
}

static int write_object_value(sidex_json_writer *writer, const sidex_json_object *object,
                              size_t max_depth)
{
	int rc = sidex_json_write_object_begin(writer);

	if (rc != SIDEX_OK) {
		return rc;
	}
	for (size_t i = 0; i < object->len; i++) {
		if (i > 0) {
			rc = sidex_json_write_value_separator(writer);
			if (rc != SIDEX_OK) {
				return rc;
			}
		}
		rc = sidex_json_write_object_key(writer, object->entries[i].key);
		if (rc != SIDEX_OK) {
			return rc;
		}
		rc = write_value_with_depth(writer, &object->entries[i].value, max_depth - 1u);
		if (rc != SIDEX_OK) {
			return rc;
		}
	}
	return sidex_json_write_object_end(writer);
}

static int write_value_with_depth(sidex_json_writer *writer, const sidex_json_value *value,
                                  size_t max_depth)
{
	if (max_depth == 0) {
		return SIDEX_ERR_DEPTH;
	}

	switch (value->kind) {
	case SIDEX_JSON_VALUE_NULL:
		return sidex_json_write_null(writer);
	case SIDEX_JSON_VALUE_BOOL:
		return sidex_json_write_bool(writer, value->as.boolean);
	case SIDEX_JSON_VALUE_NUMBER:
		if (!number_lexeme_is_valid(value->as.number)) {
			return SIDEX_ERR_SYNTAX;
		}
		return sidex_json_write_raw(writer, value->as.number.ptr, value->as.number.len);
	case SIDEX_JSON_VALUE_STRING:
		return sidex_json_write_str(writer, value->as.string);
	case SIDEX_JSON_VALUE_ARRAY:
		return write_array_value(writer, &value->as.array, max_depth);
	case SIDEX_JSON_VALUE_OBJECT:
		return write_object_value(writer, &value->as.object, max_depth);
	default:
		return SIDEX_ERR_EXPECTED;
	}
}

int sidex_json_write_value(sidex_json_writer *writer, const sidex_json_value *value,
                           size_t max_depth)
{
	return write_value_with_depth(writer, value, max_depth);
}

#include "sidex_json_writer.h"

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

void sidex_json_buffer_init(sidex_json_buffer *buffer, uint8_t *ptr, size_t cap)
{
	buffer->ptr = ptr;
	buffer->cap = cap;
	buffer->len = 0;
}

static int buffer_write(void *ctx, const uint8_t *ptr, size_t len)
{
	sidex_json_buffer *buffer = (sidex_json_buffer *)ctx;

	if (buffer->len > buffer->cap || len > buffer->cap - buffer->len) {
		return SIDEX_ERR_NO_SPACE;
	}
	if (len > 0) {
		memcpy(buffer->ptr + buffer->len, ptr, len);
		buffer->len += len;
	}
	return SIDEX_OK;
}

sidex_json_writer sidex_json_writer_for_buffer(sidex_json_buffer *buffer)
{
	sidex_json_writer writer;

	writer.write = buffer_write;
	writer.ctx = buffer;
	return writer;
}

int sidex_json_write_raw(sidex_json_writer *writer, const uint8_t *ptr, size_t len)
{
	int rc;

	if (len == 0) {
		return SIDEX_OK;
	}
	rc = writer->write(writer->ctx, ptr, len);
	if (rc == SIDEX_OK) {
		return SIDEX_OK;
	}
	if (rc < 0) {
		return rc;
	}
	return SIDEX_ERR_CALLBACK;
}

static int write_cstr(sidex_json_writer *writer, const char *str)
{
	return sidex_json_write_raw(writer, (const uint8_t *)str, strlen(str));
}

int sidex_json_write_null(sidex_json_writer *writer)
{
	return write_cstr(writer, "null");
}

int sidex_json_write_bool(sidex_json_writer *writer, bool value)
{
	return write_cstr(writer, value ? "true" : "false");
}

int sidex_json_write_i64(sidex_json_writer *writer, int64_t value)
{
	char buffer[32];
	int len = snprintf(buffer, sizeof(buffer), "%" PRId64, value);

	if (len < 0 || (size_t)len >= sizeof(buffer)) {
		return SIDEX_ERR_OVERFLOW;
	}
	return sidex_json_write_raw(writer, (const uint8_t *)buffer, (size_t)len);
}

int sidex_json_write_u64(sidex_json_writer *writer, uint64_t value)
{
	char buffer[32];
	int len = snprintf(buffer, sizeof(buffer), "%" PRIu64, value);

	if (len < 0 || (size_t)len >= sizeof(buffer)) {
		return SIDEX_ERR_OVERFLOW;
	}
	return sidex_json_write_raw(writer, (const uint8_t *)buffer, (size_t)len);
}

static int write_hex_escape(sidex_json_writer *writer, uint8_t byte)
{
	static const uint8_t hex[] = "0123456789abcdef";
	uint8_t escaped[6];

	escaped[0] = '\\';
	escaped[1] = 'u';
	escaped[2] = '0';
	escaped[3] = '0';
	escaped[4] = hex[(byte >> 4) & 0x0fu];
	escaped[5] = hex[byte & 0x0fu];
	return sidex_json_write_raw(writer, escaped, sizeof(escaped));
}

static int write_escape_pair(sidex_json_writer *writer, uint8_t escaped)
{
	uint8_t pair[2];

	pair[0] = '\\';
	pair[1] = escaped;
	return sidex_json_write_raw(writer, pair, sizeof(pair));
}

int sidex_json_write_str(sidex_json_writer *writer, sidex_str value)
{
	int rc;

	if (!sidex_utf8_validate(value.ptr, value.len)) {
		return SIDEX_ERR_UTF8;
	}

	rc = write_cstr(writer, "\"");
	if (rc != SIDEX_OK) {
		return rc;
	}

	for (size_t i = 0; i < value.len; i++) {
		uint8_t ch = value.ptr[i];

		switch (ch) {
		case '"':
			rc = write_escape_pair(writer, '"');
			break;
		case '\\':
			rc = write_escape_pair(writer, '\\');
			break;
		case '\b':
			rc = write_escape_pair(writer, 'b');
			break;
		case '\f':
			rc = write_escape_pair(writer, 'f');
			break;
		case '\n':
			rc = write_escape_pair(writer, 'n');
			break;
		case '\r':
			rc = write_escape_pair(writer, 'r');
			break;
		case '\t':
			rc = write_escape_pair(writer, 't');
			break;
		default:
			if (ch < 0x20u) {
				rc = write_hex_escape(writer, ch);
			} else {
				rc = sidex_json_write_raw(writer, &ch, 1);
			}
			break;
		}
		if (rc != SIDEX_OK) {
			return rc;
		}
	}

	return write_cstr(writer, "\"");
}

int sidex_json_write_object_begin(sidex_json_writer *writer)
{
	return write_cstr(writer, "{");
}

int sidex_json_write_object_end(sidex_json_writer *writer)
{
	return write_cstr(writer, "}");
}

int sidex_json_write_array_begin(sidex_json_writer *writer)
{
	return write_cstr(writer, "[");
}

int sidex_json_write_array_end(sidex_json_writer *writer)
{
	return write_cstr(writer, "]");
}

int sidex_json_write_name_separator(sidex_json_writer *writer)
{
	return write_cstr(writer, ":");
}

int sidex_json_write_value_separator(sidex_json_writer *writer)
{
	return write_cstr(writer, ",");
}

int sidex_json_write_object_key(sidex_json_writer *writer, sidex_str key)
{
	int rc = sidex_json_write_str(writer, key);

	if (rc != SIDEX_OK) {
		return rc;
	}
	return sidex_json_write_name_separator(writer);
}

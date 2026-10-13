import assert from "node:assert/strict"
import test from "node:test"

import type { BigInt64, Float } from "../src/builtins.ts"

import {
  Bytes,
  Date as SidexDate,
  F32,
  F64,
  I8,
  I16,
  I32,
  I64,
  Idx,
  Timestamp,
  U8,
  U16,
  U32,
  U64,
  Uuid,
  entries,
} from "../src/builtins.ts"

for (const [name, helper, min, max] of [
  ["I64", I64, -(2n ** 63n), 2n ** 63n - 1n],
  ["U64", U64, 0n, 2n ** 64n - 1n],
] as readonly [string, BigInt64<I64 | U64>, bigint, bigint][]) {
  test(`${name} preserves exact values at the safe-number boundary and type limits`, () => {
    const safe = BigInt(Number.MAX_SAFE_INTEGER)
    const values = [min, max, 0n, 1n, safe - 1n, safe, safe + 1n, safe + 2n]
    if (min < 0n) values.push(-1n, -safe + 1n, -safe, -safe - 1n, -safe - 2n)
    for (const value of values) {
      const encoded = helper.fromBigInt(value)
      assert.equal(
        encoded,
        value >= -safe && value <= safe ? Number(value) : value.toString(),
      )
      assert.equal(helper.is(encoded), true)
      assert.equal(helper.toBigInt(encoded), value)
      assert.equal(helper.toNumber(encoded), Number(value))
      assert.equal(helper.is(value.toString()), true)
      if (value >= -safe && value <= safe) {
        assert.equal(helper.fromNumber(Number(value)), Number(value))
      }
    }
    assert.equal(helper.toBigInt(helper.MIN), min)
    assert.equal(helper.toBigInt(helper.MAX), max)
    assert.equal(helper.MIN, min === 0n ? 0 : min.toString())
    assert.equal(helper.MAX, max.toString())
    for (const value of [min - 1n, max + 1n]) {
      assert.equal(helper.is(value.toString()), false)
      assert.throws(() => helper.fromBigInt(value), RangeError)
    }
  })

  test(`${name} rejects unsafe numbers, non-integers, and malformed decimal strings`, () => {
    for (const value of [
      Number.MAX_SAFE_INTEGER + 1,
      -Number.MAX_SAFE_INTEGER - 1,
      0.5,
      NaN,
      Infinity,
      -Infinity,
    ]) {
      assert.equal(helper.is(value), false)
      assert.throws(() => helper.fromNumber(value), RangeError)
    }
    for (const value of [
      null,
      undefined,
      true,
      false,
      1n,
      {},
      [],
      "",
      " 1",
      "1 ",
      "1.0",
      "1e2",
      "0x10",
      "--1",
      "Infinity",
      "NaN",
      "1\n",
    ]) {
      assert.equal(helper.is(value), false, `accepted ${String(value)}`)
    }
    assert.equal(helper.is("00042"), true)
    assert.equal(helper.is("+42"), false)
    assert.equal(helper.is("-0"), min < 0n)
    if (min === 0n) {
      assert.equal(helper.is(-1), false)
      assert.equal(helper.is("-1"), false)
      assert.throws(() => helper.fromNumber(-1), RangeError)
    }
  })
}

for (const [name, helper] of [
  ["F32", F32],
  ["F64", F64],
] as readonly [string, Float<F32 | F64>][]) {
  test(`${name} converts finite numbers and all accepted non-finite strings`, () => {
    for (const value of [
      0,
      -0,
      0.5,
      -1.25,
      Number.MIN_VALUE,
      Number.MAX_VALUE,
    ]) {
      assert.equal(helper.is(value), true)
      assert.equal(helper.fromNumber(value), value)
      assert.equal(helper.toNumber(helper.fromNumber(value)), value)
    }
    for (const [number, text] of [
      [NaN, "NaN"],
      [Infinity, "Infinity"],
      [-Infinity, "-Infinity"],
    ] as const) {
      const encoded = helper.fromNumber(number)
      assert.equal(encoded, text)
      assert.equal(helper.is(encoded), true)
      assert.equal(helper.toNumber(encoded), number)
      assert.equal(helper.is(number), false)
    }
    const legacyInfinity = "+Infinity"
    assert.ok(helper.is(legacyInfinity))
    assert.equal(helper.toNumber(legacyInfinity), Infinity)
    for (const value of [
      null,
      true,
      {},
      "",
      "1",
      "nan",
      "inf",
      "NaN\n",
      "Infinity ",
    ]) {
      assert.equal(helper.is(value), false)
    }
    assert.throws(() => helper.toNumber("invalid" as never), TypeError)
  })
}

test("Bytes round-trips binary data with padded and unpadded Base64", () => {
  for (const [bytes, encoded] of [
    [[], ""],
    [[0], "AA=="],
    [[0, 255], "AP8="],
    [[0, 1, 255], "AAH/"],
    [Array.from({ length: 256 }, (_, i) => i), null],
  ] as const) {
    const input = new Uint8Array(bytes)
    const text = Bytes.fromUint8Array(input)
    if (encoded !== null) assert.equal(text, encoded)
    assert.deepEqual(Bytes.toUint8Array(text), input)
    assert.deepEqual(
      Bytes.toUint8Array(text.replace(/=+$/, "") as never),
      input,
    )
  }
  for (const value of ["!", "A", "====", "AA=A"]) {
    assert.throws(() => Bytes.toUint8Array(value as never))
  }
})

test("Uuid accepts either case and normalizes hyphenated identifiers", () => {
  const lower = "67e55044-10b1-426f-9247-bb680e5fe0c8"
  assert.equal(Uuid.is(lower), true)
  assert.equal(Uuid.is(lower.toUpperCase()), true)
  assert.equal(Uuid.parse(lower.toUpperCase()), lower)
  assert.equal(
    Uuid.parse("00000000-0000-0000-0000-000000000000"),
    "00000000-0000-0000-0000-000000000000",
  )
  for (const value of [
    null,
    42,
    lower.replaceAll("-", ""),
    `{${lower}}`,
    lower.slice(1),
    lower + "0",
    lower.replace("e", "g"),
    lower + "\n",
  ]) {
    assert.equal(Uuid.is(value), false)
    if (typeof value === "string")
      assert.throws(() => Uuid.parse(value), TypeError)
  }
})

test("Timestamp converts offsets to UTC and documents millisecond precision", (t) => {
  const instant = new globalThis.Date("2026-10-12T10:30:00.250+02:00")
  const encoded = Timestamp.fromDate(instant)
  assert.equal(encoded, "2026-10-12T08:30:00.250Z")
  assert.equal(Timestamp.toDate(encoded).getTime(), instant.getTime())
  assert.equal(
    Timestamp.toDate(
      "2026-10-12T10:30:00.123456789+02:00" as never,
    ).toISOString(),
    "2026-10-12T08:30:00.123Z",
  )
  assert.throws(() => Timestamp.fromDate(new globalThis.Date(NaN)), RangeError)
  t.mock.timers.enable({ apis: ["Date"], now: instant.getTime() })
  assert.equal(Timestamp.now(), encoded)
})

test("Date checks the YYYY-MM-DD shape without calendar validation", () => {
  for (const value of [
    "2026-10-12",
    "2024-02-29",
    "0000-01-01",
    "9999-12-31",
    "2026-02-29",
    "2026-13-32",
  ]) {
    assert.equal(SidexDate.is(value), true, value)
  }
  for (const value of [
    null,
    42,
    "2026-2-01",
    "2026-01-1",
    "2026-10-12T00:00:00Z",
    "2026-10-12\n",
    "10000-01-01",
    "-000044-03-15",
  ]) {
    assert.equal(SidexDate.is(value), false, String(value))
  }
})

for (const [name, helper, min, max] of [
  ["I8", I8, -128, 127],
  ["I16", I16, -32768, 32767],
  ["I32", I32, -2147483648, 2147483647],
  ["U8", U8, 0, 255],
  ["U16", U16, 0, 65535],
  ["U32", U32, 0, 4294967295],
  ["Idx", Idx, 0, Number.MAX_SAFE_INTEGER],
] as const) {
  test(`${name} enforces integer bounds without coercion`, () => {
    assert.equal(helper.MIN, min)
    assert.equal(helper.MAX, max)
    for (const value of [min, min + 1, 0, max - 1, max]) {
      assert.equal(helper.is(value), true)
      assert.equal(helper.fromNumber(value), value)
    }
    for (const value of [min - 1, max + 1, 0.5, NaN, Infinity, -Infinity]) {
      assert.equal(helper.is(value), false)
      assert.throws(() => helper.fromNumber(value), RangeError)
    }
    for (const value of [null, true, "1", 1n, {}, []])
      assert.equal(helper.is(value), false)
  })
}

test("entries retains present values and omits undefined and inherited entries", () => {
  const map = Object.assign(Object.create({ inherited: "ignored" }), {
    first: 0,
    missing: undefined,
    second: null,
    third: false,
    fourth: "",
  })
  assert.deepEqual(entries(map), [
    ["first", 0],
    ["second", null],
    ["third", false],
    ["fourth", ""],
  ])
  const finite: { a?: number; b?: number } = { b: 42 }
  const typed: ["a" | "b", number][] = entries(finite)
  assert.deepEqual(typed, [["b", 42]])
  assert.deepEqual(entries({}), [])
})

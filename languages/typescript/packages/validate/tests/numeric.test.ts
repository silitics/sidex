import assert from "node:assert/strict"
import test from "node:test"

import {
  Path,
  eq,
  holds,
  maxExclusive,
  maxInclusive,
  minExclusive,
  minInclusive,
  ne,
  numeric,
} from "../src/index.ts"

test("numeric keeps decimal integers exact and decodes non-finite floats", () => {
  for (const [input, expected] of [
    ["0", 0n],
    ["-0", 0n],
    ["00042", 42n],
    ["9007199254740991", 9007199254740991n],
    ["9007199254740992", 9007199254740992n],
    ["9007199254740993", 9007199254740993n],
    ["-9223372036854775808", -(2n ** 63n)],
    ["18446744073709551615", 2n ** 64n - 1n],
    ["NaN", NaN],
    ["Infinity", Infinity],
    ["+Infinity", Infinity],
    ["-Infinity", -Infinity],
  ] as const)
    assert.equal(numeric(input), expected)
  for (const value of [
    0,
    -1.5,
    Number.MAX_SAFE_INTEGER,
    true,
    null,
    undefined,
    {},
    "",
    "+42",
    "1.5",
    "1e3",
    " 42",
    "42 ",
    "42\n",
    "nan",
    "text",
  ]) {
    assert.equal(numeric(value), value)
  }
})

test("comparison helpers enforce exact bounds and report paths and messages", () => {
  const path = Path.root().field("devices").index(2).field("size")
  const cases = [
    [9007199254740991, "9007199254740991", 0],
    ["9007199254740992", "9007199254740993", -1],
    ["9007199254740993", "9007199254740992", 1],
    ["-9223372036854775808", "-9223372036854775807", -1],
    ["18446744073709551615", "18446744073709551614", 1],
    ["10", "9", 1],
    [42, "42", 0],
    ["42", 42, 0],
  ] as const
  for (const [left, right, cmp] of cases) {
    const value = numeric(left)
    const bound = numeric(right)
    for (const [helper, valid] of [
      [minInclusive, cmp >= 0],
      [minExclusive, cmp > 0],
      [maxInclusive, cmp <= 0],
      [maxExclusive, cmp < 0],
      [eq, cmp === 0],
      [ne, cmp !== 0],
    ] as const) {
      const report = helper(value, bound, "size", "Invalid size.", path)
      assert.equal(report.isOk(), valid)
      assert.deepEqual(
        report.errors,
        valid
          ? []
          : [
              {
                path: "devices[2].size",
                code: "size",
                message: "Invalid size.",
              },
            ],
      )
    }
  }
  assert.deepEqual(
    minInclusive(
      numeric("9007199254740992"),
      numeric("9007199254740993"),
      "min",
      null,
      Path.root(),
    ).errors,
    [
      {
        path: "",
        code: "min",
        message: "Value is below the minimum.",
      },
    ],
  )
})

test("holds compares chained mixed numeric representations without rounding", () => {
  assert.equal(
    holds(
      [
        numeric("9007199254740992"),
        numeric("9007199254740993"),
        numeric("18446744073709551615"),
      ],
      ["<", "<="],
    ),
    true,
  )
  assert.equal(
    holds([numeric("9007199254740993"), numeric("9007199254740992")], ["<"]),
    false,
  )
  assert.equal(holds([42, numeric("42"), 43], ["==", "!="]), true)
  assert.equal(
    holds([numeric("-9223372036854775808"), -42, 0], ["<", "<="]),
    true,
  )
  assert.equal(
    holds(
      [numeric("18446744073709551615"), numeric("18446744073709551614")],
      [">"],
    ),
    true,
  )
  assert.equal(holds([42, numeric("42")], [">="]), true)
})

test("comparison helpers preserve NaN and infinity semantics", () => {
  for (const helper of [
    minInclusive,
    minExclusive,
    maxInclusive,
    maxExclusive,
    eq,
  ]) {
    assert.equal(
      helper(numeric("NaN"), 0, "number", null, Path.root()).isErr(),
      true,
    )
  }
  assert.equal(
    ne(numeric("NaN"), numeric("NaN"), "ne", null, Path.root()).isOk(),
    true,
  )
  assert.equal(
    maxInclusive(
      numeric("Infinity"),
      Infinity,
      "max",
      null,
      Path.root(),
    ).isOk(),
    true,
  )
  assert.equal(
    minInclusive(
      numeric("-Infinity"),
      -Infinity,
      "min",
      null,
      Path.root(),
    ).isOk(),
    true,
  )
  assert.equal(holds([numeric("NaN"), 0], ["=="]), false)
  assert.equal(holds([numeric("NaN"), 0], ["!="]), true)
})

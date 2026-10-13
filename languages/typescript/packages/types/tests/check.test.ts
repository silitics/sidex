import assert from "node:assert/strict"
import test from "node:test"

import {
  Bytes,
  Date as SidexDate,
  Timestamp,
  Path,
  builtin,
  checker,
  jsonShape,
  lazy,
  map,
  record,
  sequence,
  variant,
} from "../dist/index.js"

// Checks siblings and all map/sequence entries while preserving wire paths.
test("structural reports collect every error, ignore unknowns, and accept optional null", () => {
  const task = checker(() =>
    record([
      ["name", builtin("string"), false],
      ["note", builtin("string"), true],
      ["numbers", sequence(builtin("u8")), false],
      ["map", map(builtin("i8"), builtin("bool")), false],
    ]),
  )
  assert.ok(task.is({ name: "a", note: null, numbers: [], map: {}, extra: 1 }))
  const report = task.check({
    name: 1,
    numbers: [256, -1],
    map: { "128": 0, "1": "no" },
  })
  assert.deepEqual(
    report.errors.map((error) => [error.path, error.code]),
    [
      ["name", "type"],
      ["numbers[0]", "type"],
      ["numbers[1]", "type"],
      ["map.1", "type"],
      ["map.128", "key"],
      ["map.128", "type"],
    ],
  )
  assert.equal(report.isErr(), true)
  assert.match(report.toString(), /numbers\[0\]/)
  const dotted = task.check({}, Path.root().key("a.b")).errors[0]!
  assert.deepEqual(dotted.segments, [
    { kind: "key", key: "a.b" },
    { kind: "key", key: "name" },
  ])
})

// Matches exactly one external entry and selects implicit cases without leaked errors.
test("variant checks enforce tagging and discard unsuccessful implicit attempts", () => {
  const point = checker(() => record([["x", builtin("i8"), false]]))
  const external = checker(() =>
    variant("external", "", [
      ["none", null, null],
      ["point", lazy(() => point), null],
    ]),
  )
  assert.ok(external.is("none"))
  assert.ok(external.is({ point: { x: 1, extra: true } }))
  assert.equal(external.is({ none: null }), false)
  assert.equal(external.is({ point: { x: 1 }, extra: true }), false)
  const internal = checker(() =>
    variant("internal", "kind", [["point", lazy(() => point), ["value"]]]),
  )
  assert.ok(internal.is({ kind: "point", x: 1 }))
  assert.equal(internal.is({ kind: "point", value: { x: 1 } }), false)
  const implicit = checker(() =>
    variant("implicit", "", [
      ["point", lazy(() => point), null],
      ["number", builtin("i8"), null],
    ]),
  )
  assert.ok(implicit.check(1).isOk())
  assert.equal(implicit.check({ x: true }).errors.length, 3)
})

// Format guards check calendar values, offsets, Base64 trailing bits, and annotations.
test("scalar formats accept Rust reader spellings and reject malformed values", () => {
  for (const value of ["", "AA", "AA=", "AA==", "AP8", "AP8=", "AAH/"])
    assert.ok(Bytes.is(value), value)
  for (const value of ["A", "AB", "AB==", "AAF=", "AA==\n", " AA==", "AA=A"])
    assert.equal(Bytes.is(value), false, value)
  for (const value of [
    "2024-02-29",
    "-000044-03-15",
    "+0099991231",
    "20261012",
    "2026-10-12[u-ca=iso8601]",
  ])
    assert.ok(SidexDate.is(value), value)
  for (const value of [
    "2026-02-29",
    "-000000-01-01",
    "2026-10-12T00:00Z",
    "2026-10-12[!u-ca=iso8601]",
    "2026-10-12\n",
  ])
    assert.equal(SidexDate.is(value), false, value)
  for (const value of [
    "2026-10-12T10:30:00.123456789+02:00",
    "20261012t1030z",
    "2026-10-12 10Z",
    "-000044-03-15T00:00Z",
    "2026-10-12T10:30:60Z",
    "2026-10-12T10:30+02[Europe/Paris]",
  ])
    assert.ok(Timestamp.is(value), value)
  for (const value of [
    "2026-02-30T00:00Z",
    "2026-10-12T24:00Z",
    "2026-10-12T10:30",
    "2026-10-12T10:30Z\n",
    "2026-10-12T10:30:00.1234567890Z",
  ])
    assert.equal(Timestamp.is(value), false, value)
})

// Unconstrained opaque types retain JSON semantics rather than accepting JS-only values.
test("opaque shapes reject non-JSON values and allow repeated noncyclic references", () => {
  const any = checker(() => jsonShape(["any"]))
  const shared = { value: 1 }
  assert.ok(any.is({ a: shared, b: shared }))
  const cyclic: { next?: unknown } = {}
  cyclic.next = cyclic
  for (const value of [
    undefined,
    NaN,
    Infinity,
    1n,
    () => 1,
    { nested: undefined },
    cyclic,
  ])
    assert.equal(any.is(value), false)
  assert.ok(checker(() => jsonShape(["null", "string"])).is(null))
  assert.equal(checker(() => jsonShape(["object"])).is([]), false)
})

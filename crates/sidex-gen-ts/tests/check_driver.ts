import * as data from "./gen/data"
import * as wire from "./json/data"
import * as runtime from "./runtime/index"
import { ValidationReport } from "./validate"
import type { Checker } from "./runtime/index"

function assert(value: unknown, message: string): asserts value {
  if (!value) throw new Error(message)
}

const boolean = runtime.checker<boolean>(() => runtime.builtin("bool"))
const integer = runtime.checker<runtime.I32>(() => runtime.builtin("i32"))
const unsigned = runtime.checker<runtime.U64>(() => runtime.builtin("u64"))
const checks: Record<string, Checker<unknown>> = {
  Task: data.Task,
  VariantTestRecord: wire.VariantTestRecord,
  Scalars: data.Scalars,
  Empty: data.Empty,
  Ids: data.Ids,
  Tasks: data.Tasks,
  Wrapped: data.Wrapped.checker(data.Task),
  Inline: data.Inline,
  Internal: data.Internal.checker(data.Task),
  Adjacent: data.Adjacent,
  External: data.External,
  Implicit: data.Implicit,
  Opaques: data.Opaques,
  OtherBundle: data.OtherBundle,
  Forward: data.Forward,
  Maps: data.Maps,
  VariantInternallyTagged: wire.VariantInternallyTagged.checker(integer),
  VariantInternallyTaggedPoint: wire.VariantInternallyTagged.checker(
    wire.Point,
  ),
  VariantExternallyTagged: wire.VariantExternallyTagged.checker(integer),
  VariantAdjacentlyTagged: wire.VariantAdjacentlyTagged.checker(integer),
  VariantImplicitlyTagged: wire.VariantImplicitlyTagged.checker(boolean),
  MaybeRefInteger: wire.MaybeRef.checker(unsigned),
  MaybeRefPoint: wire.MaybeRef.checker(wire.Point),
  Any: wire.Any,
  JsRoundtrip: wire.JsRoundtrip,
  RecordFieldNamesDefault: wire.RecordFieldNamesDefault,
  RecordFieldNamesScreamingSnake: wire.RecordFieldNamesScreamingSnake,
  i8map: runtime.checker(() =>
    runtime.map(runtime.builtin("i8"), runtime.builtin("string")),
  ),
}
for (const name of [
  "i8",
  "i16",
  "i32",
  "i64",
  "u8",
  "u16",
  "u32",
  "u64",
  "idx",
  "f32",
  "f64",
  "bytes",
  "uuid",
  "timestamp",
  "date",
]) {
  checks[name] = runtime.checker(() => runtime.builtin(name))
}
const documents: { typ: string; text: string; accepted: boolean }[] = DOCUMENTS
for (const { typ, text, accepted } of documents) {
  const input: unknown = JSON.parse(text)
  const checker = checks[typ]!
  const report = checker.check(input)
  assert(
    checker.is(input) === accepted,
    `${typ}.is(${text}) expected ${accepted}: ${report}`,
  )
  assert(
    report.isOk() === accepted,
    `${typ}.check(${text}) expected ${accepted}: ${report}`,
  )
}

// Optional fields permit absence, while present values must have the field's type.
const input: unknown = { title: "", done: false }
assert(data.Task.is(input), "absent optional field is accepted")
assert(data.Task.check(input).isOk(), "absent optional field has no errors")
const nullTask = { title: "a", done: false, note: null }
assert(!data.Task.is(nullTask), "optional string rejects null")
const nullErrors = data.Task.check(nullTask).errors
assert(
  nullErrors.length === 1 &&
    nullErrors[0]?.path === "note" &&
    nullErrors[0]?.code === "type",
  "optional null is a type error at the field path",
)
const nullNext = data.Tasks.check({ items: [], next: null }).errors
assert(
  nullNext.length === 1 &&
    nullNext[0]?.path === "next" &&
    nullNext[0]?.code === "type",
  "optional record rejects null at the field path",
)
const nullable: data.OptionalNulls = {
  nullable: null,
  nullOnly: null,
  anything: null,
}
assert(
  data.OptionalNulls.is(nullable),
  "nullable optional field types accept null",
)
assert(
  data.OptionalNulls.check(nullable).isOk(),
  "nullable optional fields have no errors",
)
if (data.Task.is(input)) {
  const task: data.Task = input
  const note: string | undefined = task.note
  assert(note === undefined, "narrowed optional field type excludes null")
  assert(
    data.Task.validate(task).isErr(),
    "structural acceptance must not run validation rules",
  )
}
assert(data.Page.is({ items: [input] }, data.Task), "generic is")
assert(data.Page.check({ items: [input] }, data.Task).isOk(), "generic check")

// @ts-expect-error A wrapper requires its path's brand.
const unbranded: data.Id = 42
assert(unbranded === 42, "brands leave runtime values unchanged")
const identifier: unknown = 42
if (data.Id.is(identifier)) {
  // @ts-expect-error Different paths remain distinct with the same representation.
  const other: data.OtherId = identifier
  assert(other === 42, "nominal checks do not convert values")
}

const genericValue: unknown = { value: input, "display-name": "" }
if (data.Validated.is(genericValue, data.Task)) {
  assert(data.Validated.validate(genericValue).isErr(), "generic rule methods")
}
assert(
  data.Validated.fields["display-name"].validate("").isErr(),
  "quoted field validator",
)
assert(
  data.ValidatedList.tryNew<runtime.I32>([]).ok === false,
  "generic wrapper factory",
)
assert(
  data.Inline.is({ x: 1, y: 2, "display.label": "x", CUSTOM_NAME: true }),
  "field rename",
)
assert(
  !data.Inline.is({ x: 1, y: 2, "display.label": "x", CUSTOM_NAME: 1 }),
  "field rename validation",
)
const cyclic: { items: unknown[]; next?: unknown } = { items: [] }
cyclic.next = cyclic
assert(!data.Page.is(cyclic, data.Task), "generic cycles are rejected")
const recursive: { self?: unknown } = {}
recursive.self = recursive
assert(!wire.Any.is(recursive), "implicit recursive cycles are rejected")
const allScalars = {
  signed8: -128,
  signed16: -32768,
  signed32: -2147483648,
  signed64: "-9223372036854775808",
  unsigned8: 255,
  unsigned16: 65535,
  unsigned32: 4294967295,
  unsigned64: "18446744073709551615",
  index: 9007199254740992,
  single: "NaN",
  double: "Infinity",
  blob: "AA=",
  id: "67e55044-10b1-426f-9247-bb680e5fe0c8",
  instant: "2026-10-12T00:00Z",
  day: "2024-02-29",
  anything: [1, "x", null],
}
assert(data.Scalars.is(allScalars), "all generated built-in checks")
const malformed = Object.fromEntries(
  Object.keys(allScalars).map((key) => [key, {}]),
)
assert(
  data.Scalars.check(malformed).errors.length === 15,
  `every invalid built-in field: ${data.Scalars.check(malformed)}`,
)

assert(
  data.Dictionary.is(
    { entries: { "42": { title: "a", done: true } } },
    data.Id,
    data.Task,
  ),
  "generic map keys",
)
assert(!data.Table.is({ entries: { "-1": {} } }), "concrete generic map alias")

const taskErrors = data.Task.check({ title: 1, done: "false", note: 2 }).errors
assert(
  taskErrors.map((error) => error.path).join() === "title,done,note",
  "all field errors",
)
const nested = data.Tasks.check({
  items: [{}, { title: 1, done: null }],
  next: { items: false },
})
assert(
  nested.errors.map((error) => error.path).join() ===
    "items[0].title,items[0].done,items[1].title,items[1].done,next.items",
  "nested paths",
)
const inline = data.Inline.check({ "display.label": 1 })
assert(
  inline.errors.some((error) => error.path === "x"),
  "inline paths must omit the schema field",
)
assert(
  inline.errors.find((error) => error.path === "display.label")?.segments
    .length === 1,
  "segments disambiguate dotted keys",
)
assert(
  data.External.check({ point: {} })
    .errors.map((error) => error.path)
    .join() === "point.x,point.y",
  "external paths",
)
assert(
  data.Internal.check({ kind: "renamed", payload: {} }, data.Task)
    .errors.map((error) => error.path)
    .join() === "payload.x,payload.y",
  "content paths",
)
assert(
  data.Implicit.check({ x: true }).errors.length === 4,
  "implicit report includes all attempted payloads",
)

const merged = ValidationReport.ok()
merged.merge(data.Task.check({}))
assert(
  merged.errors.length === 2,
  "reports compose without a package dependency",
)

console.log(`Checked ${documents.length} JSON documents.`)

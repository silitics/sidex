/** Calendar and text checks matching the Rust runtime's Jiff parser. */

/** Checks a calendar date, including extended years and Temporal annotations. */
export function isDate(value: unknown): boolean {
  if (typeof value !== "string" || /[Tt ]/.test(value)) return false
  const parsed = datePrefix(value)
  return parsed !== null && annotations(value.slice(parsed.length))
}

/** Checks a timestamp with a required offset and up to nanosecond precision. */
export function isTimestamp(value: unknown): boolean {
  if (typeof value !== "string") return false
  const date = datePrefix(value)
  if (!date || !/[Tt ]/.test(value[date.length] ?? "")) return false
  const tail = value.slice(date.length + 1)
  const match =
    /^(\d{2})(?:(:?)((?:\d{2}))(?:(\2)(\d{2})(?:[.,](\d{1,9}))?)?)?([Zz]|[+-]\d{2}(?:(:?)\d{2}(?:\8\d{2}(?:[.,]\d{1,9})?)?)?)(.*)$/.exec(
      tail,
    )
  if (!match) return false
  const hour = Number(match[1]),
    minute = Number(match[3] ?? 0),
    second = Math.min(Number(match[5] ?? 0), 59)
  if (
    hour > 23 ||
    minute > 59 ||
    Number(match[5] ?? 0) > 60 ||
    !annotations(match[9]!)
  )
    return false
  const offset = parseOffset(match[7]!)
  if (offset === null) return false
  const instant =
    date.milliseconds + (hour * 3600 + minute * 60 + second - offset) * 1000
  // Jiff reserves 25:59:59 at both calendar boundaries for UTC offset conversion.
  return instant >= -377705023201000 && instant <= 253402207200000
}

function datePrefix(
  value: string,
): { length: number; milliseconds: number } | null {
  const match = /^(\d{4}|[+-]\d{6})(-?)(\d{2})\2(\d{2})/.exec(value)
  if (!match || match[1] === "-000000") return null
  const year = Number(match[1]),
    month = Number(match[3]),
    day = Number(match[4])
  if (year < -9999 || year > 9999 || month < 1 || month > 12 || day < 1)
    return null
  const date = new globalThis.Date(0)
  date.setUTCFullYear(year, month - 1, day)
  if (
    date.getUTCFullYear() !== year ||
    date.getUTCMonth() !== month - 1 ||
    date.getUTCDate() !== day
  )
    return null
  return { length: match[0].length, milliseconds: date.getTime() }
}

function parseOffset(value: string, subminute = true): number | null {
  if (/^[Zz]$/.test(value)) return 0
  const match =
    /^([+-])(\d{2})(?:(:?)(\d{2})(?:\3(\d{2})(?:[.,](\d{1,9}))?)?)?$/.exec(
      value,
    )
  if (!match) return null
  const hour = Number(match[2]),
    minute = Number(match[4] ?? 0),
    second = Number(match[5] ?? 0)
  if (
    hour > 25 ||
    minute > 59 ||
    second > 59 ||
    (!subminute && match[5] !== undefined)
  )
    return null
  const seconds =
    hour * 3600 +
    minute * 60 +
    second +
    Math.round(Number("0." + (match[6] ?? "0")))
  if (seconds > 93599) return null
  return match[1] === "-" ? -seconds : seconds
}

function annotations(value: string): boolean {
  let first = true
  while (value !== "") {
    const match = /^\[([^\]]+)\]/.exec(value)
    if (!match) return false
    const text = match[1]!
    if (text.includes("=")) {
      if (!/^[a-z_][a-z_0-9-]*=[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$/.test(text))
        return false
    } else {
      if (!first) return false
      const zone = text.replace(/^!/, "")
      if (zone.startsWith("+") || zone.startsWith("-")) {
        if (parseOffset(zone, false) === null) return false
      } else if (
        !/^[A-Za-z_.][A-Za-z_.0-9+-]*(?:\/[A-Za-z_.][A-Za-z_.0-9+-]*)*$/.test(
          zone,
        )
      )
        return false
    }
    first = false
    value = value.slice(match[0].length)
  }
  return true
}

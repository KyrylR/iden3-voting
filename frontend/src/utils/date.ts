// Helper to create a Date object from a Unix timestamp (in seconds)
export function unixToDate(unix: bigint) {
  return new Date(Number(unix * 1000n))
}

// Helper to convert a Date object or date-like value to a Unix timestamp (in seconds)
export function dateToUnix(date = new Date()) {
  return Math.floor(new Date(date).getTime() / 1000)
}

// Helper to format a duration given in seconds into a human-readable string
export function formatDuration(seconds: number) {
  const units = {
    day: 86400,
    hour: 3600,
    minute: 60,
    second: 1,
  }

  const parts = []
  let remainingSeconds = Math.abs(seconds)

  for (const [unit, valueInSeconds] of Object.entries(units)) {
    const count = Math.floor(remainingSeconds / valueInSeconds)
    if (count > 0) {
      parts.push(`${count} ${unit}${count > 1 ? 's' : ''}`)
      remainingSeconds -= count * valueInSeconds
    }
  }

  return parts.length > 0 ? parts.join(', ') : '0 seconds'
}

// Helper to compare two dates
export function compareDates(a: string, b: string) {
  const dateA = new Date(a)
  const dateB = new Date(b)
  return dateA.getTime() - dateB.getTime()
}

// Helper to format a date relative to the current time
export function formatDateRelative(value: string, lang = 'en-GB') {
  if (!value) return '–'

  const formatter = new Intl.RelativeTimeFormat(lang, { numeric: 'auto' })
  const date = new Date(value)
  const now = Date.now()
  const secondsAgo = Math.round((date.getSeconds() - now) / 1000)

  for (const [unit, secondsInUnit] of Object.entries({
    year: 31536000,
    month: 2592000,
    week: 604800,
    day: 86400,
    hour: 3600,
    minute: 60,
  })) {
    if (Math.abs(secondsAgo) > secondsInUnit || unit === 'minute') {
      const count = Math.round(secondsAgo / secondsInUnit)
      return formatter.format(count, <Intl.RelativeTimeFormatUnit>unit)
    }
  }

  return formatter.format(secondsAgo, 'second')
}

// Helper to format a date object in GMT timezone
export function formatTimeGMT(value: Date) {
  const date = new Date(value)
  return date.toISOString().split('T')[1].slice(0, 5) + ' GMT'
}

// Helper to format a date object as day.month.year
export function formatDateDMY(value: Date) {
  const date = new Date(value)
  return date
    .toLocaleDateString('en-GB', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    })
    .replace(/\//g, '.')
}

// Helper to format a date object as day.month.year hours:minutes in GMT
export function formatDateGMT(value: string) {
  const date = new Date(value)
  return `${formatDateDMY(date)} ${formatTimeGMT(date)}`
}

// Generic helper to format a date object according to a specified pattern
export function formatDate(value: string, pattern: string) {
  if (!value) return '–'

  const options = createDateTimeFormatOptions(pattern)

  const formatter = new Intl.DateTimeFormat('en-GB', options)

  return formatter.format(Number(value))
}

function createDateTimeFormatOptions(pattern: string) {
  const options = {}
  const parts = pattern.split(/[\s./:-]+/)

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const mapping: any = {
    dd: { day: '2-digit' },
    MM: { month: '2-digit' },
    yyyy: { year: 'numeric' },
    HH: { hour: '2-digit', hour12: false },
    mm: { minute: '2-digit' },
    OOOO: { timeZoneName: 'long' },
  }

  parts.forEach(part => {
    if (mapping[part]) {
      Object.assign(options, mapping[part])
    }
  })

  return options
}

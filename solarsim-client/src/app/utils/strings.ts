export function capitalize(string: string, separator: string = "_"): string {
  return string
    .split(separator)
    .map((word) => word.charAt(0).toUpperCase().concat(word.slice(1)))
    .join(" ");
}

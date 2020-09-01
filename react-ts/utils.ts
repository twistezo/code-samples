export default class Utils {
  static arrayToChunks = <T>(array: T[], chunkSize: number): T[][] => {
    let i, j: number
    const resultArray = []

    for (i = 0, j = array.length; i < j; i += chunkSize) {
      resultArray.push(array.slice(i, i + chunkSize))
    }

    return resultArray
  }

  static camelToKebabCase = (str: string): string =>
    str.replace(/[A-Z]/g, letter => `-${letter.toLowerCase()}`)
}

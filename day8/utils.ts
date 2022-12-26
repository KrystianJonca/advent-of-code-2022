export const readTextFile = async (path: string) => {
  const decoder = new TextDecoder('utf-8')
  const inputFile = await Deno.readFile(path)
  return decoder.decode(inputFile)
}
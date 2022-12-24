export type File = {
  name: string,
  size: number,
}

export type Directory = {
  name: string,
  contents: (File | Directory)[],
}

export type FileSystem = Directory[];
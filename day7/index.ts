import { readTextFile } from './utils.ts';
import type { FileSystem, File, Directory } from './types.ts';

const input = await readTextFile('input.txt');

const {
  fileSystem,
  executeCommand,
  parseAndCreateDirectory,
  parseAndCreateFile,
} = createFileSystem();

for (const line of input.split('\n')) {
  if (isCommand(line)) {
    executeCommand(line);
  } else if (isDirectory(line)) {
    parseAndCreateDirectory(line);
  } else if (isFile(line)) {
    parseAndCreateFile(line);
  }
}

console.log('--- PART 1 ---');
console.log(findTotalSizesOfDirsBelowThreshold(fileSystem[0], 100000));

console.log('--- PART 2 ---');
console.log(findTheSizeOfSmallestDirToDelete(fileSystem, 70000000, 30000000));

function findTheSizeOfSmallestDirToDelete(
  fileSystem: FileSystem,
  totalSize: number,
  sizeNeeded: number
) {
  const [rootDir] = fileSystem;
  const sizeTaken = findTotalSizeOfDir(rootDir);

  const minDirSize = sizeNeeded - (totalSize - sizeTaken);
  if (minDirSize < 0) return 0;

  const dirToDelete = findSmallestDirGreaterThanThreshold(rootDir, minDirSize);

  return findTotalSizeOfDir(dirToDelete);
}

function findSmallestDirGreaterThanThreshold(
  dir: Directory,
  threshold: number
) {
  let smallestDir: Directory = dir;
  let smallestDirSize = findTotalSizeOfDir(dir);

  for (const content of dir.contents) {
    if ('size' in content) continue;
    const dirSize = findTotalSizeOfDir(content as Directory);
    if (dirSize < smallestDirSize && dirSize > threshold) {
      const childDir = findSmallestDirGreaterThanThreshold(
        content as Directory,
        threshold
      );
      const childDirSize = findTotalSizeOfDir(childDir);
      if (childDirSize < dirSize) {
        smallestDir = childDir as Directory;
        smallestDirSize = childDirSize;
      } else {
        smallestDir = content as Directory;
        smallestDirSize = dirSize;
      }
    }
  }

  return smallestDir;
}

function findTotalSizesOfDirsBelowThreshold(dir: Directory, threshold: number) {
  let totalSize = 0;

  const dirSize = findTotalSizeOfDir(dir);

  for (const content of dir.contents) {
    if ('size' in content) continue;
    totalSize += findTotalSizesOfDirsBelowThreshold(
      content as Directory,
      threshold
    );
  }

  return dirSize < threshold ? dirSize + totalSize : totalSize;
}

function findTotalSizeOfDir(dir: Directory) {
  let totalSize = 0;
  for (const content of dir.contents) {
    if ('size' in content) {
      totalSize += content.size;
    } else {
      totalSize += findTotalSizeOfDir(content);
    }
  }

  return totalSize;
}

function createFileSystem() {
  const fileSystem: FileSystem = [{ name: '/', contents: [] }];
  const currentPath: string[] = ['/'];

  const executeCommand = (command: string) => {
    const [_, commandName, ...args] = command.split(' ');
    if (commandName === 'ls') return;

    if (commandName === 'cd') {
      if (args[0] === '..') {
        currentPath.pop();
      } else if (args[0] === '/') {
        currentPath.splice(1, currentPath.length);
      } else {
        currentPath.push(args[0]);
      }
    }
  };

  const parseAndCreateFile = (line: string) => {
    const [size, name] = line.split(' ');
    const file: File = {
      name,
      size: parseInt(size),
    };
    addToFileSystem(fileSystem, currentPath, file);
  };

  const parseAndCreateDirectory = (line: string) => {
    const [_, dirName] = line.split(' ');
    const dir: Directory = {
      name: dirName,
      contents: [],
    };
    addToFileSystem(fileSystem, currentPath, dir);
  };

  const addToFileSystem = (
    fileSystem: FileSystem,
    currentPath: string[],
    content: File | Directory
  ) => {
    const [currentDirName, ...remainingPath] = currentPath;

    const currentDir = fileSystem.find((dir) => dir.name === currentDirName);
    if (!currentDir) {
      throw new Error(`Directory ${currentDirName} not found in file system`);
    }

    if (remainingPath.length === 0) {
      currentDir.contents.push(content);
      return;
    }

    addToFileSystem(currentDir.contents as FileSystem, remainingPath, content);
  };

  return {
    fileSystem,
    currentPath,
    executeCommand,
    parseAndCreateDirectory,
    parseAndCreateFile,
  };
}

function isCommand(line: string) {
  return line.startsWith('$');
}

function isDirectory(line: string) {
  return line.startsWith('dir');
}

function isFile(line: string) {
  return !isNaN(parseInt(line.split(' ')[0]));
}

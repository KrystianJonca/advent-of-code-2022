import { readTextFile } from './utils.ts';
import { Grid } from './grid.ts';

const input = await readTextFile('./input.txt');
const lines = input.split('\n');

let maxScenicScore = 0;
const treesVisible = new Grid(lines[0].length, lines.length, 0);

for (let i = 0; i < lines.length; i++) {
  if (i === 0 || i === lines.length - 1) {
    treesVisible.setRow(i, 1);
    continue;
  }

  for (let j = 0; j < lines[i].length; j++) {
    if (j === 0 || j === lines[i].length - 1) {
      treesVisible.set(i, j, 1);
      continue;
    }
    const treeHeight = lines[i][j];

    let leftVisibility = j;
    let isVisibleFromLeft = true;
    for (let l = 0; l < j; l++) {
      if (lines[i][l] >= treeHeight) {
        leftVisibility = j - l;
        isVisibleFromLeft = false;
      }
    }

    let rightVisibility = lines[i].length - 1 - j;
    let isVisibleFromRight = true;
    for (let r = j + 1; r < lines[i].length; r++) {
      if (lines[i][r] >= treeHeight) {
        rightVisibility = r - j;
        isVisibleFromRight = false;
        break;
      }
    }

    let topVisibility = i;
    let isVisibleFromTop = true;
    for (let t = 0; t < i; t++) {
      if (lines[t][j] >= treeHeight) {
        topVisibility = i - t;
        isVisibleFromTop = false;
      }
    }

    let bottomVisibility = lines.length - i - 1;
    let isVisibleFromBottom = true;
    for (let b = i + 1; b < lines.length; b++) {
      if (lines[b][j] >= treeHeight) {
        bottomVisibility = b - i;
        isVisibleFromBottom = false;
        break;
      }
    }
    if (isVisibleFromLeft || isVisibleFromRight || isVisibleFromTop || isVisibleFromBottom) treesVisible.set(i, j, 1);

    const treeScenicScore =
      leftVisibility * rightVisibility * topVisibility * bottomVisibility;
    if (treeScenicScore > maxScenicScore) maxScenicScore = treeScenicScore;
  }
}

const treesVisibleCount = treesVisible.getGrid().reduce((acc, row) => {
  return (
    acc +
    row.reduce((acc, col) => {
      return acc + col;
    }, 0)
  );
}, 0);

console.log("---PART 1---")
console.log(treesVisibleCount);
console.log("---PART 2---")
console.log(maxScenicScore);
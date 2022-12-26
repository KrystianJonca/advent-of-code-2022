export class Grid {
  grid: number[][];

  constructor(width: number, height: number, value: number = 0) {
    this.grid = new Array(height)
      .fill(value)
      .map(() => new Array(width).fill(value));
  }

  set(y: number, x: number, value: number) {
    this.grid[y][x] = value;
  }

  get(y: number, x: number) {
    return this.grid[y][x];
  }

  setRow(y: number, value: number) {
    this.grid[y].fill(value);
  }

  getGrid() {
    return this.grid;
  }
}

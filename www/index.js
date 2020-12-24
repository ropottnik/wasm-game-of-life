import { Universe, Cell } from "wasm-game-of-life";
import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg';

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const width = 100;
const height = 100;
const universe = Universe.new(width, height);
universe.set_rle_shape('11b2o11b2o11b$11b2o11b2o11b3$6bo23bo6b$5bobo5bo9bo5bobo5b$4bo2bo5bob2o3b2obo5bo2bo4b$5b2o10bobo10b2o5b$15bobobobo15b$16bo3bo16b2$2o33b2o$2o33b2o$5b2o23b2o5b2$6bobo19bobo6b$6bo2bo17bo2bo6b$7b2o19b2o7b2$7b2o19b2o7b$6bo2bo17bo2bo6b$6bobo19bobo6b2$5b2o23b2o5b$2o33b2o$2o33b2o2$16bo3bo16b$15bobobobo15b$5b2o10bobo10b2o5b$4bo2bo5bob2o3b2obo5bo2bo4b$5bobo5bo9bo5bobo5b$6bo23bo6b3$11b2o11b2o11b$11b2o11b2o', 25, 25);

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;
const isPaused = () => {
  return animationId === null;
}

const playPausButton = document.getElementById("play-pause");

const play = () => {
  playPausButton.textContent = "\u23F8";
  renderLoop();
}

const pause = () => {
  playPausButton.textContent = "\u23EF";
  cancelAnimationFrame(animationId);
  animationId = null;
}

playPausButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
}

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();

}

const getIndex = (row, column) => {
  return row * width + column;
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col ++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
	? DEAD_COLOR
	: ALIVE_COLOR;

      ctx.fillRect(
	col * (CELL_SIZE + 1) + 1,
	row * (CELL_SIZE + 1) + 1,
	CELL_SIZE,
	CELL_SIZE,
      );
    }
  }

  ctx.stroke();
}

play();

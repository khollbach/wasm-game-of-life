import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.random();
const height = universe.num_rows();
const width = universe.num_cols();

// 1px border b/w & around the cells.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height * (CELL_SIZE + 1) + 1;
canvas.width = width * (CELL_SIZE + 1) + 1;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    drawCells();
    drawGrid();

    universe.tick();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // horizontal lines
    for (let y = 0; y <= height; y++) {
        const scaled_y = y * (CELL_SIZE + 1);
        const x_max = width * (CELL_SIZE + 1);
        ctx.moveTo(0, scaled_y);
        ctx.lineTo(x_max + 1, scaled_y + 1);
    }

    // vertical lines
    for (let x = 0; x <= width; x++) {
        const scaled_x = x * (CELL_SIZE + 1);
        const y_max = height * (CELL_SIZE + 1);
        ctx.moveTo(scaled_x, 0);
        ctx.lineTo(scaled_x + 1, y_max + 1);
    }

    ctx.stroke();
};

const getIndex = (x, y) => {
    return y * width + x;
};

const bitIsSet = (arr, i) => {
    const byte = Math.floor(i / 8);
    const mask = 1 << (i % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, height * width / 8);

    ctx.beginPath();

    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            const idx = getIndex(x, y);
            ctx.fillStyle = bitIsSet(cells, idx)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                x * (CELL_SIZE + 1) + 1,
                y * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE,
            );
        }
    }

    ctx.stroke();
};

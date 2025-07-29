import init, { WasmChip8 } from '../pkg/your_crate_name.js'; 

const SCREEN_WIDTH = 64;
const SCREEN_HEIGHT = 32;

const canvas = document.getElementById('chip8-canvas');
canvas.width = SCREEN_WIDTH * 10;
canvas.height = SCREEN_HEIGHT * 10;
const ctx = canvas.getContext('2d');

let emulator = null;
let animationFrameId = null;

// Keypad mapping (CHIP-8 keys: 0x0 - 0xF)
const keymap = {
  'x': 0x0, '1': 0x1, '2': 0x2, '3': 0x3,
  'q': 0x4, 'w': 0x5, 'e': 0x6,
  'a': 0x7, 's': 0x8, 'd': 0x9,
  'z': 0xA, 'c': 0xB, '4': 0xC,
  'r': 0xD, 'f': 0xE, 'v': 0xF
};
let keys_down = Array(16).fill(false);

function handleKey(e, down) {
  let val = e.key?.toLowerCase();
  if (val in keymap) {
    keys_down[keymap[val]] = down;
    if (emulator) emulator.set_key(keymap[val], down);
    e.preventDefault();
  }
}

document.addEventListener('keydown', (e) => handleKey(e, true));
document.addEventListener('keyup',   (e) => handleKey(e, false));

// ROM loader
document.getElementById('romLoader').addEventListener('change', function(ev) {
  const file = ev.target.files[0];
  if (!file || !emulator) return;
  const reader = new FileReader();
  reader.onload = function(e) {
    const arrayBuffer = e.target.result;
    const romBytes = new Uint8Array(arrayBuffer);
    emulator.reset();
    emulator.load_rom(romBytes);
  };
  reader.readAsArrayBuffer(file);
});

function renderDisplay() {
  // Access memory shared by WASM
  const mem = wasmMemory.buffer;
  const ptr = emulator.display_buffer_ptr();
  const screen = new Uint32Array(mem, ptr, SCREEN_WIDTH * SCREEN_HEIGHT);

  const imageData = ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);
  for (let i = 0; i < screen.length; ++i) {
    let v = screen[i] !== 0 ? 255 : 0;
    imageData.data[i * 4 + 0] = v; // R
    imageData.data[i * 4 + 1] = v; // G
    imageData.data[i * 4 + 2] = v; // B
    imageData.data[i * 4 + 3] = 255; // A
  }
  // Draw small canvas, then scale up
  ctx.putImageData(imageData, 0, 0);
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(canvas, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, 0, 0, canvas.width, canvas.height);
}

function frame() {
  for (let i = 0; i < 10; ++i) { // Run multiple cycles per frame for playable speed
    emulator.cycle();
  }
  renderDisplay();
  animationFrameId = requestAnimationFrame(frame);
}

let wasmMemory;

init().then(wasm => {
  // After WASM is loaded
  wasmMemory = wasm.memory;
  emulator = new WasmChip8();
  renderDisplay();
  animationFrameId = requestAnimationFrame(frame);
});
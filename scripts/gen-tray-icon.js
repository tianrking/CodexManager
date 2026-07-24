// Minimal PNG encoder (no deps) — generates a monochrome "stacked cards"
// template icon for the macOS menu bar (black + alpha, 44x44 for retina).
const fs = require('fs');
const zlib = require('zlib');

const W = 44, H = 44;
const rgba = Buffer.alloc(W * H * 4); // 0 = fully transparent

function setBlack(x, y) {
  if (x < 0 || x >= W || y < 0 || y >= H) return;
  const i = (y * W + x) * 4;
  rgba[i] = 0; rgba[i + 1] = 0; rgba[i + 2] = 0; rgba[i + 3] = 255;
}

// point-in rounded rect
function inRR(px, py, x0, y0, x1, y1, r) {
  if (px < x0 || px > x1 || py < y0 || py > y1) return false;
  let cx = 0, cy = 0, corner = false;
  if (px < x0 + r && py < y0 + r) { cx = x0 + r; cy = y0 + r; corner = true; }
  else if (px > x1 - r && py < y0 + r) { cx = x1 - r; cy = y0 + r; corner = true; }
  else if (px < x0 + r && py > y1 - r) { cx = x0 + r; cy = y1 - r; corner = true; }
  else if (px > x1 - r && py > y1 - r) { cx = x1 - r; cy = y1 - r; corner = true; }
  if (!corner) return true;
  const dx = px - cx, dy = py - cy;
  return dx * dx + dy * dy <= r * r;
}

for (let y = 0; y < H; y++) {
  for (let x = 0; x < W; x++) {
    // back card = outline ring; front card = filled (offset down-right)
    const backRing = inRR(x, y, 5, 5, 29, 29, 6) && !inRR(x, y, 9, 9, 25, 25, 4);
    const frontFill = inRR(x, y, 15, 15, 39, 39, 6);
    if (backRing || frontFill) setBlack(x, y);
  }
}

// ---- PNG encode ----
const CRC_TABLE = [];
for (let n = 0; n < 256; n++) {
  let c = n;
  for (let k = 0; k < 8; k++) c = (c & 1) ? (0xEDB88320 ^ (c >>> 1)) : (c >>> 1);
  CRC_TABLE[n] = c >>> 0;
}
function crc32(buf) {
  let c = 0xFFFFFFFF;
  for (let i = 0; i < buf.length; i++) c = CRC_TABLE[(c ^ buf[i]) & 0xFF] ^ (c >>> 8);
  return (c ^ 0xFFFFFFFF) >>> 0;
}
function chunk(type, data) {
  const len = Buffer.alloc(4); len.writeUInt32BE(data.length, 0);
  const t = Buffer.from(type, 'ascii');
  const crc = Buffer.alloc(4); crc.writeUInt32BE(crc32(Buffer.concat([t, data])), 0);
  return Buffer.concat([len, t, data, crc]);
}
const sig = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4);
ihdr[8] = 8; ihdr[9] = 6; ihdr[10] = 0; ihdr[11] = 0; ihdr[12] = 0;
const raw = Buffer.alloc((W * 4 + 1) * H);
for (let y = 0; y < H; y++) {
  raw[y * (W * 4 + 1)] = 0;
  rgba.copy(raw, y * (W * 4 + 1) + 1, y * W * 4, (y + 1) * W * 4);
}
const idat = zlib.deflateSync(raw);
const png = Buffer.concat([sig, chunk('IHDR', ihdr), chunk('IDAT', idat), chunk('IEND', Buffer.alloc(0))]);
const out = process.argv[2];
fs.writeFileSync(out, png);
console.log('wrote', out, png.length, 'bytes', W + 'x' + H);

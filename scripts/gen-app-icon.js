// 1024x1024 app icon source: gradient rounded square + white "stacked cards"
// (same concept as the menu-bar tray icon, for cohesive branding).
const fs = require('fs');
const zlib = require('zlib');

const W = 1024, H = 1024;
const rgba = Buffer.alloc(W * H * 4);

function lerp(a, b, t) { return Math.round(a + (b - a) * t); }
// brand gradient #5b6cf2 -> #8a7cf2
const C0 = [0x5b, 0x6c, 0xf2], C1 = [0x8a, 0x7c, 0xf2];

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
    const i = (y * W + x) * 4;
    // background = rounded square (r=225) filled with diagonal gradient, else transparent
    if (inRR(x, y, 0, 0, W - 1, H - 1, 225)) {
      const t = (x + y) / (2 * (W - 1));
      rgba[i] = lerp(C0[0], C1[0], t);
      rgba[i + 1] = lerp(C0[1], C1[1], t);
      rgba[i + 2] = lerp(C0[2], C1[2], t);
      rgba[i + 3] = 255;
    }
    // foreground white "stacked cards" (back ring + front fill), scaled & centered
    const backRing = inRR(x, y, 206, 206, 638, 638, 108) && !inRR(x, y, 278, 278, 566, 566, 72);
    const frontFill = inRR(x, y, 386, 386, 818, 818, 108);
    if (backRing || frontFill) {
      rgba[i] = 255; rgba[i + 1] = 255; rgba[i + 2] = 255; rgba[i + 3] = 255;
    }
  }
}

// ---- PNG encode (same minimal encoder) ----
const CRC_TABLE = [];
for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = (c & 1) ? (0xEDB88320 ^ (c >>> 1)) : (c >>> 1); CRC_TABLE[n] = c >>> 0; }
function crc32(buf) { let c = 0xFFFFFFFF; for (let i = 0; i < buf.length; i++) c = CRC_TABLE[(c ^ buf[i]) & 0xFF] ^ (c >>> 8); return (c ^ 0xFFFFFFFF) >>> 0; }
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
for (let y = 0; y < H; y++) { raw[y * (W * 4 + 1)] = 0; rgba.copy(raw, y * (W * 4 + 1) + 1, y * W * 4, (y + 1) * W * 4); }
const idat = zlib.deflateSync(raw);
fs.writeFileSync(process.argv[2], Buffer.concat([sig, chunk('IHDR', ihdr), chunk('IDAT', idat), chunk('IEND', Buffer.alloc(0))]));
console.log('wrote', process.argv[2], W + 'x' + H);

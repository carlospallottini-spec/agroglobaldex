const fs = require('fs');
const path = require('path');
const sharp = require('sharp');

const svgPath = path.join(__dirname, 'icon.svg');
const buildDir = __dirname;
const sizes = [16, 24, 32, 48, 64, 128, 256];

(async () => {
  const pngToIco = (await import('png-to-ico')).default;
  const svgBuf = fs.readFileSync(svgPath);
  const pngBuffers = [];

  for (const s of sizes) {
    const buf = await sharp(svgBuf, { density: 384 })
      .resize(s, s, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
      .png()
      .toBuffer();
    pngBuffers.push(buf);
    if (s === 256) fs.writeFileSync(path.join(buildDir, 'icon.png'), buf);
  }

  const icoBuf = await pngToIco(pngBuffers);
  fs.writeFileSync(path.join(buildDir, 'icon.ico'), icoBuf);
  console.log('icon.ico created (' + icoBuf.length + ' bytes) with sizes: ' + sizes.join(', '));
})().catch(err => { console.error(err); process.exit(1); });

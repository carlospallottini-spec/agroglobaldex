const fs = require('fs');
const path = require('path');
const sharp = require('sharp');

const svg = fs.readFileSync(path.join(__dirname, 'icon.svg'));
const out = 'C:/Users/pallotrader/OneDrive/Documentos/AgroChain/AgroChain-Logo-1024.png';

sharp(svg, { density: 1024 })
  .resize(1024, 1024)
  .png()
  .toFile(out)
  .then(() => console.log('Saved ' + out));

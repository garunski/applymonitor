import sharp from 'sharp';
import toIcoPkg from 'to-ico';
import { readFileSync, writeFileSync, mkdirSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const toIco = toIcoPkg.default || toIcoPkg;

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const logoPath = join(__dirname, '../src/icons/logo.svg');
const rootDir = join(__dirname, '../..');

// Define sizes for each platform
const webSizes = [16, 32, 48, 64, 128, 192, 512];
const desktopSizes = [256, 512];
const mobileSizes = [180, 1024];

// Output directories
const webAssetsDir = join(rootDir, 'web/assets');
const uiAssetsDir = join(rootDir, 'ui/assets');
const desktopAssetsDir = join(rootDir, 'desktop/assets');
const mobileAssetsDir = join(rootDir, 'mobile/assets');

// Ensure output directories exist
[webAssetsDir, uiAssetsDir, desktopAssetsDir, mobileAssetsDir].forEach(dir => {
  mkdirSync(dir, { recursive: true });
});

async function generateLogos() {
  console.log('Generating logos from', logoPath);

  // Read the SVG
  const svgBuffer = readFileSync(logoPath);

  // Copy SVG to web/assets for web-specific components
  writeFileSync(join(webAssetsDir, 'logo.svg'), svgBuffer);
  console.log('  ✓ Copied logo.svg to web/assets');

  // Copy SVG to ui/assets for shared components
  writeFileSync(join(uiAssetsDir, 'logo.svg'), svgBuffer);
  console.log('  ✓ Copied logo.svg to ui/assets');

  // Generate web favicons
  console.log('Generating web favicons...');
  const webIcons = [];
  for (const size of webSizes) {
    const outputPath = join(webAssetsDir, `favicon-${size}x${size}.png`);
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Generated favicon-${size}x${size}.png`);
    webIcons.push(readFileSync(outputPath));
  }

  // Generate favicon.ico from multiple sizes
  try {
    const icoBuffer = await toIco(webIcons.slice(0, 4)); // Use first 4 sizes for ICO
    writeFileSync(join(webAssetsDir, 'favicon.ico'), icoBuffer);
    console.log('  ✓ Generated favicon.ico');
  } catch (error) {
    console.warn('  ⚠ Failed to generate favicon.ico:', error.message);
  }

  // Generate desktop icons
  console.log('Generating desktop icons...');
  for (const size of desktopSizes) {
    const outputPath = join(desktopAssetsDir, `icon-${size}x${size}.png`);
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Generated icon-${size}x${size}.png`);
  }

  // Generate mobile icons
  console.log('Generating mobile icons...');
  for (const size of mobileSizes) {
    const outputPath = join(mobileAssetsDir, `icon-${size}x${size}.png`);
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Generated icon-${size}x${size}.png`);
  }

  console.log('✓ Logo generation complete!');
}

generateLogos().catch(error => {
  console.error('Error generating logos:', error);
  process.exit(1);
});


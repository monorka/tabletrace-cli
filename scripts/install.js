const https = require('https');
const fs = require('fs');
const path = require('path');

const REPO = 'monorka/tabletrace-cli';
const VERSION = require('../package.json').version;

// Determine platform and architecture
const platform = process.platform;
const arch = process.arch;

const PLATFORM_MAP = {
  'darwin-arm64': 'aarch64-apple-darwin',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-arm64': 'aarch64-unknown-linux-musl',
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

const platformKey = `${platform}-${arch}`;
const target = PLATFORM_MAP[platformKey];

console.log('');
console.log('┌─────────────────────────────────────────┐');
console.log('│     📦 Installing TableTrace CLI        │');
console.log('└─────────────────────────────────────────┘');
console.log('');

if (!target) {
  console.error(`❌ Unsupported platform: ${platformKey}`);
  console.error('');
  console.error('Supported platforms:');
  console.error('  • darwin-arm64  (macOS Apple Silicon)');
  console.error('  • darwin-x64    (macOS Intel)');
  console.error('  • linux-arm64   (Linux ARM64)');
  console.error('  • linux-x64     (Linux x64)');
  console.error('  • win32-x64     (Windows x64)');
  console.error('');
  console.error('Alternative: cargo install tabletrace');
  process.exit(1);
}

const ext = platform === 'win32' ? '.exe' : '';
const binaryName = `tabletrace-bin${ext}`;
const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/tabletrace-${target}${ext}`;
const binDir = path.join(__dirname, '..', 'bin');
const binPath = path.join(binDir, binaryName);

console.log(`Platform: ${platformKey}`);
console.log(`Version:  v${VERSION}`);
console.log('');

// Create bin directory if it doesn't exist
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

// Format bytes to human readable
function formatBytes(bytes) {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

// Get terminal width
function getTerminalWidth() {
  return process.stdout.columns || 80;
}

// Print banner based on terminal width (same as banner.rs)
function printBanner() {
  const cyan = '\x1b[36m';
  const dim = '\x1b[2m';
  const reset = '\x1b[0m';
  const width = getTerminalWidth();

  if (width >= 90) {
    // Large ASCII art
    console.log(cyan + '  ████████╗ █████╗ ██████╗ ██╗     ███████╗  ████████╗██████╗  █████╗  ██████╗███████╗' + reset);
    console.log(cyan + '  ╚══██╔══╝██╔══██╗██╔══██╗██║     ██╔════╝  ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██╔════╝' + reset);
    console.log(cyan + '     ██║   ███████║██████╔╝██║     █████╗       ██║   ██████╔╝███████║██║     █████╗  ' + reset);
    console.log(cyan + '     ██║   ██╔══██║██╔══██╗██║     ██╔══╝       ██║   ██╔══██╗██╔══██║██║     ██╔══╝  ' + reset);
    console.log(cyan + '     ██║   ██║  ██║██████╔╝███████╗███████╗     ██║   ██║  ██║██║  ██║╚██████╗███████╗' + reset);
    console.log(cyan + '     ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝     ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚══════╝' + reset);
    console.log('');
    console.log(dim + '                       Real-time PostgreSQL Monitor' + reset);
  } else if (width >= 60) {
    // Medium banner
    console.log(cyan + '  ╔════════════════════════════════════════════════════╗' + reset);
    console.log(cyan + '  ║     ▀█▀ █▀█ █▄▄ █   █▀▀   ▀█▀ █▀█ █▀█ █▀▀ █▀▀     ║' + reset);
    console.log(cyan + '  ║      █  █▀█ █▄█ █▄▄ ██▄    █  █▀▄ █▀█ █▄▄ ██▄     ║' + reset);
    console.log(cyan + '  ╚════════════════════════════════════════════════════╝' + reset);
  } else {
    // Small banner
    console.log(cyan + '╭──────────────────────────╮' + reset);
    console.log(cyan + '│   Table Trace CLI        │' + reset);
    console.log(cyan + '╰──────────────────────────╯' + reset);
  }
  console.log('');
}

// Download binary with progress
function download(url, dest, redirectCount = 0) {
  if (redirectCount > 5) {
    return Promise.reject(new Error('Too many redirects'));
  }

  return new Promise((resolve, reject) => {
    console.log(`📥 Downloading binary...`);

    https.get(url, (response) => {
      // Handle redirects
      if (response.statusCode === 302 || response.statusCode === 301) {
        console.log('   ↳ Following redirect...');
        download(response.headers.location, dest, redirectCount + 1)
          .then(resolve)
          .catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`HTTP ${response.statusCode}: Failed to download`));
        return;
      }

      const totalBytes = parseInt(response.headers['content-length'], 10);
      let downloadedBytes = 0;
      let lastPercent = 0;

      const file = fs.createWriteStream(dest);

      response.on('data', (chunk) => {
        downloadedBytes += chunk.length;
        if (totalBytes) {
          const percent = Math.floor((downloadedBytes / totalBytes) * 100);
          if (percent >= lastPercent + 10 || percent === 100) {
            process.stdout.write(`\r   ↳ Progress: ${percent}% (${formatBytes(downloadedBytes)} / ${formatBytes(totalBytes)})`);
            lastPercent = percent;
          }
        } else {
          process.stdout.write(`\r   ↳ Downloaded: ${formatBytes(downloadedBytes)}`);
        }
      });

      response.pipe(file);

      file.on('finish', () => {
        console.log('');
        file.close();
        resolve();
      });

      file.on('error', (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });

    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

download(downloadUrl, binPath)
  .then(() => {
    // Make executable on Unix
    if (platform !== 'win32') {
      fs.chmodSync(binPath, 0o755);
    }

    const stats = fs.statSync(binPath);
    console.log(`   ↳ Binary size: ${formatBytes(stats.size)}`);
    console.log('');
    console.log('✅ Installation complete!');
    console.log('');

    // Show banner based on terminal width
    printBanner();

    console.log('  Get started:');
    console.log('    tabletrace watch --preset postgres');
    console.log('    tabletrace watch --preset supabase');
    console.log('    tabletrace --help');
    console.log('');
  })
  .catch((err) => {
    console.log('');
    console.error(`❌ Failed to download: ${err.message}`);
    console.error('');
    console.error('Alternative installation methods:');
    console.error('');
    console.error('  1. Install via Cargo (Rust):');
    console.error('     cargo install tabletrace');
    console.error('');
    console.error('  2. Download manually:');
    console.error(`     ${downloadUrl}`);
    console.error('');
    process.exit(1);
  });

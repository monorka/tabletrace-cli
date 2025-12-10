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

// Check if this is an update (binary already exists)
const isUpdate = fs.existsSync(path.join(__dirname, '..', 'bin', 'tabletrace-bin'));

// Use stderr for guaranteed output (npm may suppress stdout during postinstall)
const log = (...args) => process.stderr.write(args.join(' ') + '\n');
const logInline = (...args) => process.stderr.write(args.join(' '));

log('');
if (isUpdate) {
  log('┌─────────────────────────────────────────┐');
  log('│     🔄 Updating TableTrace CLI          │');
  log('└─────────────────────────────────────────┘');
} else {
  log('┌─────────────────────────────────────────┐');
  log('│     📦 Installing TableTrace CLI        │');
  log('└─────────────────────────────────────────┘');
}
log('');

if (!target) {
  log(`❌ Unsupported platform: ${platformKey}`);
  log('');
  log('Supported platforms:');
  log('  • darwin-arm64  (macOS Apple Silicon)');
  log('  • darwin-x64    (macOS Intel)');
  log('  • linux-arm64   (Linux ARM64)');
  log('  • linux-x64     (Linux x64)');
  log('  • win32-x64     (Windows x64)');
  log('');
  log('Alternative: cargo install tabletrace');
  process.exit(1);
}

const ext = platform === 'win32' ? '.exe' : '';
const binaryName = `tabletrace-bin${ext}`;
const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/tabletrace-${target}${ext}`;
const binDir = path.join(__dirname, '..', 'bin');
const binPath = path.join(binDir, binaryName);

log(`Platform: ${platformKey}`);
log(`Version:  v${VERSION}`);
log('');

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
  return process.stderr.columns || process.stdout.columns || 80;
}

// Print banner based on terminal width (same as banner.rs)
function printBanner() {
  const cyan = '\x1b[36m';
  const dim = '\x1b[2m';
  const reset = '\x1b[0m';
  const width = getTerminalWidth();

  if (width >= 90) {
    // Large ASCII art
    log(cyan + '  ████████╗ █████╗ ██████╗ ██╗     ███████╗  ████████╗██████╗  █████╗  ██████╗███████╗' + reset);
    log(cyan + '  ╚══██╔══╝██╔══██╗██╔══██╗██║     ██╔════╝  ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██╔════╝' + reset);
    log(cyan + '     ██║   ███████║██████╔╝██║     █████╗       ██║   ██████╔╝███████║██║     █████╗  ' + reset);
    log(cyan + '     ██║   ██╔══██║██╔══██╗██║     ██╔══╝       ██║   ██╔══██╗██╔══██║██║     ██╔══╝  ' + reset);
    log(cyan + '     ██║   ██║  ██║██████╔╝███████╗███████╗     ██║   ██║  ██║██║  ██║╚██████╗███████╗' + reset);
    log(cyan + '     ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝     ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚══════╝' + reset);
    log('');
    log(dim + '                       Real-time PostgreSQL Monitor' + reset);
  } else if (width >= 60) {
    // Medium banner
    log(cyan + '  ╔════════════════════════════════════════════════════╗' + reset);
    log(cyan + '  ║     ▀█▀ █▀█ █▄▄ █   █▀▀   ▀█▀ █▀█ █▀█ █▀▀ █▀▀     ║' + reset);
    log(cyan + '  ║      █  █▀█ █▄█ █▄▄ ██▄    █  █▀▄ █▀█ █▄▄ ██▄     ║' + reset);
    log(cyan + '  ╚════════════════════════════════════════════════════╝' + reset);
  } else {
    // Small banner
    log(cyan + '╭──────────────────────────────────────────╮' + reset);
    log(cyan + '│       Table Trace CLI                    │' + reset);
    log(cyan + '╰──────────────────────────────────────────╯' + reset);
  }
  log('');
}

// Download binary with progress
function download(url, dest, redirectCount = 0) {
  if (redirectCount > 5) {
    return Promise.reject(new Error('Too many redirects'));
  }

  return new Promise((resolve, reject) => {
    log(`📥 Downloading binary...`);

    https.get(url, (response) => {
      // Handle redirects
      if (response.statusCode === 302 || response.statusCode === 301) {
        log('   ↳ Following redirect...');
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
            logInline(`\r   ↳ Progress: ${percent}% (${formatBytes(downloadedBytes)} / ${formatBytes(totalBytes)})`);
            lastPercent = percent;
          }
        } else {
          logInline(`\r   ↳ Downloaded: ${formatBytes(downloadedBytes)}`);
        }
      });

      response.pipe(file);

      file.on('finish', () => {
        log('');
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
    log(`   ↳ Binary size: ${formatBytes(stats.size)}`);
    log('');
    if (isUpdate) {
      log(`✅ Updated to v${VERSION}!`);
    } else {
      log('✅ Installation complete!');
    }
    log('');

    // Show banner based on terminal width
    printBanner();

    log('  Get started:');
    log('    tabletrace watch --preset postgres');
    log('    tabletrace watch --preset supabase');
    log('    tabletrace --help');
    log('');
  })
  .catch((err) => {
    log('');
    log(`❌ Failed to download: ${err.message}`);
    log('');
    log('Alternative installation methods:');
    log('');
    log('  1. Install via Cargo (Rust):');
    log('     cargo install --git https://github.com/monorka/tabletrace-cli');
    log('');
    log('  2. Download manually:');
    log(`     ${downloadUrl}`);
    log('');
    process.exit(1);
  });

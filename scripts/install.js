const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

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

if (!target) {
  console.error(`Unsupported platform: ${platformKey}`);
  console.error('Supported platforms: darwin-arm64, darwin-x64, linux-arm64, linux-x64, win32-x64');
  process.exit(1);
}

const ext = platform === 'win32' ? '.exe' : '';
const binaryName = `tabletrace-bin${ext}`;
const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/tabletrace-${target}${ext}`;
const binDir = path.join(__dirname, '..', 'bin');
const binPath = path.join(binDir, binaryName);

console.log(`Downloading tabletrace for ${platformKey}...`);
console.log(`URL: ${downloadUrl}`);

// Create bin directory if it doesn't exist
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

// Download binary
function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    https.get(url, (response) => {
      // Handle redirects
      if (response.statusCode === 302 || response.statusCode === 301) {
        file.close();
        fs.unlinkSync(dest);
        download(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', (err) => {
      file.close();
      fs.unlinkSync(dest);
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
    console.log('✅ tabletrace installed successfully!');
  })
  .catch((err) => {
    console.error('Failed to download tabletrace binary:', err.message);
    console.error('');
    console.error('You can build from source:');
    console.error('  cargo install tabletrace');
    process.exit(1);
  });


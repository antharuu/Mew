const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');
const {pipeline} = require('stream/promises');
const tar = require('tar');

function getBinaryName() {
    return os.platform() === 'win32' ? 'mew.exe' : 'mew';
}

function getPlatformString() {
    const platform = os.platform();
    const arch = os.arch();

    switch (platform) {
        case 'win32':
            return `win32-${arch}`;
        case 'darwin':
            return `darwin-${arch}`;
        case 'linux':
            return `linux-${arch}`;
        default:
            throw new Error(`Unsupported platform: ${platform}-${arch}`);
    }
}

function downloadFile(url) {
    return new Promise((resolve, reject) => {
        https.get(url, response => {
            if (response.statusCode === 302) {
                // Handle redirect
                https.get(response.headers.location, resolve).on('error', reject);
            } else {
                resolve(response);
            }
        }).on('error', reject);
    });
}

async function downloadAndInstallBinary() {
    try {
        const tempDir = path.join(os.tmpdir(), 'mew-install');
        if (!fs.existsSync(tempDir)) {
            fs.mkdirSync(tempDir, {recursive: true});
        }

        const version = require('./package.json').version;
        const platform = getPlatformString();
        const url = `https://github.com/antharuu/Mew/releases/download/v${version}/mew-${platform}.tar.gz`;
        const tarPath = path.join(tempDir, 'mew.tar.gz');
        const binaryName = getBinaryName();
        const finalPath = path.join(__dirname, binaryName);

        console.log('Downloading from:', url);
        console.log('Temporary tar file:', tarPath);
        console.log('Final binary location:', finalPath);

        // Download the file
        const response = await downloadFile(url);

        // Save to temporary file
        const writeStream = fs.createWriteStream(tarPath);
        await pipeline(response, writeStream);

        // Extract
        await tar.x({
            file: tarPath, cwd: __dirname
        });

        // Check if binary exists
        if (!fs.existsSync(finalPath)) {
            throw new Error(`Binary not found after extraction: ${finalPath}`);
        }

        // Set permissions
        try {
            fs.chmodSync(finalPath, 0o755);
        } catch (error) {
            console.warn('Warning: Could not set executable permissions:', error.message);
        }

        // Cleanup
        try {
            fs.unlinkSync(tarPath);
            fs.rmdirSync(tempDir, {recursive: true});
        } catch (error) {
            console.warn('Warning: Could not cleanup temporary files:', error.message);
        }

        return finalPath;
    } catch (error) {
        console.error('Error during installation:', error);
        throw error;
    }
}

async function copyLocalBinary() {
    const binaryName = getBinaryName();
    const possiblePaths = [path.join(__dirname, 'target', 'release', binaryName), path.join(__dirname, '..', 'target', 'release', binaryName), path.join(process.cwd(), 'target', 'release', binaryName)];

    for (const sourcePath of possiblePaths) {
        console.log('Checking for local binary at:', sourcePath);
        if (fs.existsSync(sourcePath)) {
            const destPath = path.join(__dirname, binaryName);
            console.log('Found local binary, copying to:', destPath);
            fs.copyFileSync(sourcePath, destPath);
            try {
                fs.chmodSync(destPath, 0o755);
            } catch (error) {
                console.warn('Warning: Could not set executable permissions:', error.message);
            }
            return true;
        }
    }
    return false;
}

async function install() {
    try {
        // Try local binary first
        if (await copyLocalBinary()) {
            console.log('Local binary installed successfully');
            return;
        }

        // Try download
        console.log('No local binary found, attempting download...');
        await downloadAndInstallBinary();
        console.log('Binary downloaded and installed successfully');
    } catch (error) {
        console.error('Installation failed:', error);
        process.exit(1);
    }
}

// Run installation
install().catch(error => {
    console.error('Installation failed:', error);
    process.exit(1);
});
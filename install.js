const {Binary} = require('binary-install');
const os = require('os');

function getPlatform() {
    const type = os.type();
    const arch = os.arch();

    if (type === 'Windows_NT') {
        return `win32-${arch}`;
    }
    if (type === 'Linux') {
        return `linux-${arch}`;
    }
    if (type === 'Darwin') {
        return `darwin-${arch}`;
    }

    throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function install() {
    try {
        const platform = getPlatform();
        const version = require('./package.json').version;
        const url = `https://github.com/antharuu/Mew/releases/download/v${version}/mew-${platform}.tar.gz`;
        const name = 'mew';

        const binary = new Binary(name, url, version);
        binary.install();
    } catch (err) {
        console.error('Installation failed:', err.message);
        process.exit(1);
    }
}

install();
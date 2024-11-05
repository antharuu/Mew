const {Binary} = require('binary-install');
const os = require('os');
const {join} = require('path');

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

function getBinary() {
    const platform = getPlatform();
    const version = require('./package.json').version;
    const url = `https://github.com/antharuu/Mew/releases/download/v${version}/mew-${platform}.tar.gz`;
    const name = 'mew';

    return new Binary(name, url, version);
}

const binary = getBinary();
module.exports = binary.path;
const path = require('path');
const fs = require('fs');
const os = require('os');

function getBinaryName() {
    return os.platform() === 'win32' ? 'mew.exe' : 'mew';
}

function findBinary() {
    const binaryName = getBinaryName();
    // Chemins possibles pour le binaire
    const possiblePaths = [
        path.join(__dirname, binaryName),                           // Dans le dossier actuel
        path.join(process.cwd(), 'target', 'release', binaryName), // Dans target/release du dossier courant
        path.join(process.cwd(), '..', 'target', 'release', binaryName), // Dans target/release du dossier parent
        path.join(__dirname, 'target', 'release', binaryName),      // Dans target/release relatif au module
        path.join(__dirname, '..', 'target', 'release', binaryName) // Dans target/release du parent du module
    ];

    console.log('Looking for binary in:');
    for (const binPath of possiblePaths) {
        console.log('- ', binPath);
        if (fs.existsSync(binPath)) {
            console.log('Found binary at:', binPath);
            return binPath;
        }
    }

    console.log('No binary found in any of the expected locations');
    return null;
}

module.exports = findBinary;
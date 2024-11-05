#!/usr/bin/env node

const {spawn} = require('child_process');
const path = require('path');
const fs = require('fs');
const findBinary = require('.');

try {
    // Get the actual path to the binary
    const binaryPath = findBinary();

    if (!binaryPath) {
        console.error('Mew binary not found. Try running:');
        console.error('npm remove @antharuu/mew');
        console.error('npm install @antharuu/mew');
        process.exit(1);
    }

    if (!fs.existsSync(binaryPath)) {
        console.error(`Binary not found at: ${binaryPath}`);
        console.error('Try reinstalling the package:');
        console.error('npm remove @antharuu/mew');
        console.error('npm install @antharuu/mew');
        process.exit(1);
    }

    console.log('Using binary at:', binaryPath);

    // Forward all arguments to the binary
    const child = spawn(binaryPath, process.argv.slice(2), {
        stdio: 'inherit'
    });

    child.on('error', (err) => {
        console.error('Failed to start Mew:', err.message);
        if (err.code === 'ENOENT') {
            console.error('Binary not found or not executable.');
            console.error('Try reinstalling the package:');
            console.error('npm remove @antharuu/mew');
            console.error('npm install @antharuu/mew');
        }
        process.exit(1);
    });

    child.on('exit', function (code) {
        process.exit(code || 0);
    });
} catch (error) {
    console.error('Unexpected error:', error);
    process.exit(1);
}
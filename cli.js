#!/usr/bin/env node

const {spawn} = require('child_process');
const path = require('path');

// Get the path to the binary
const binary = require('.');

// Forward all arguments to the binary
const child = spawn(binary, process.argv.slice(2), {
    stdio: 'inherit'
});

child.on('exit', function (code) {
    process.exit(code);
});
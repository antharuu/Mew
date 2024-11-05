@echo off

REM Build the package
echo Building Rust binary...
cargo build --release

REM Create package directory
mkdir release-package
copy target\release\mew.exe release-package\

REM Create tar.gz
tar -czf mew-win32-x64.tar.gz -C release-package mew.exe

REM Cleanup
rmdir /s /q release-package

REM Create npm package
npm run pack-local

REM Test installation
mkdir test-install
cd test-install
npm init -y
npm install ..\antharuu-mew-1.0.1.tgz

REM Test command
npx mew --version

cd ..
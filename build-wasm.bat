@echo off
echo Building Rust Steam Game for WebAssembly...
echo.

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: wasm-pack is not installed!
    echo.
    echo Please install wasm-pack first:
    echo   cargo install wasm-pack
    echo.
    echo Or download from: https://rustwasm.github.io/wasm-pack/installer/
    pause
    exit /b 1
)

REM Clean previous builds
if exist pkg rmdir /s /q pkg
if exist dist rmdir /s /q dist

echo Step 1: Building WASM package...
wasm-pack build --target web --out-dir pkg --dev

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error: WASM build failed!
    pause
    exit /b 1
)

echo.
echo Step 2: Creating distribution directory...
mkdir dist 2>nul

echo Step 3: Copying files...
copy index.html dist\ >nul
xcopy pkg dist\pkg\ /E /I /Y >nul

echo.
echo ✅ Build complete!
echo.
echo To run the game:
echo   1. Start a local web server in the 'dist' directory
echo   2. For example: python -m http.server 8000
echo   3. Open http://localhost:8000 in your browser
echo.
echo Files are in: dist\
echo.

REM Check if Python is available for quick server
where python >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Quick start option:
    echo   cd dist
    echo   python -m http.server 8000
    echo.
)

pause
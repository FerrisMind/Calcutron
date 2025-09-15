@echo off
echo Building Calcutron for all platforms
echo ==================================

echo.
echo 1. Building Windows installer ^(EXE^)
echo -------------------------------
echo Building MSI installer first...
cargo wix

echo Building EXE installer using WiX Bootstrapper...
"C:\Program Files (x86)\WiX Toolset v3.14\bin\candle.exe" "wix/bundle.wxs" -ext WixBalExtension
"C:\Program Files (x86)\WiX Toolset v3.14\bin\light.exe" "bundle.wixobj" -ext WixBalExtension -o "calcutron-installer.exe"

if exist calcutron-installer.exe (
    echo Windows EXE installer created successfully: calcutron-installer.exe
) else (
    echo Failed to create Windows EXE installer
)

echo.
echo 2. To build Debian ^(.deb^) and RPM ^(.rpm^) packages, you need to use a Linux environment.
echo    See packaging_instructions.md for detailed instructions.
echo.

echo Build process completed.
pause
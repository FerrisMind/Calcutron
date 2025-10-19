@echo off
echo Building Calcutron for all platforms
echo ==================================

echo.
echo 1. Building Windows installers using cargo-packager
echo --------------------------------------------
echo Creating EXE and MSI installers...
cargo packager --release
if %ERRORLEVEL% EQU 0 (
    echo Windows installers created successfully
) else (
    echo Failed to create Windows installers using cargo-packager
    goto :manual_build
)

echo.
echo 2. To build Debian ^(.deb^) and RPM ^(.rpm^) packages, you need to use a Linux environment.
echo    See PACKAGING_SUMMARY.md for detailed instructions.
echo.

echo Build process completed.
goto :end

:manual_build
echo.
echo Attempting manual build with WiX Toolset...
echo --------------------------------------------
mkdir installer_files 2>nul
copy "target\release\calcutron.exe" "installer_files\" >nul
copy "README.md" "installer_files\" >nul
copy "LICENSE" "installer_files\" >nul
7z a -sfx installer.exe installer_files\
if exist installer.exe (
    echo Windows EXE installer created successfully: installer.exe
) else (
    echo Failed to create Windows EXE installer
)

:end
pause
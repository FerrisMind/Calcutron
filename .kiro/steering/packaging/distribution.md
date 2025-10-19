---
inclusion: fileMatch
fileMatchPattern: ['build_all_packages.bat', 'wix/**/*', 'packaging_instructions.md', 'PACKAGING_SUMMARY.md']
---

- Windows installer создается через WiX toolset в @wix/ директории
- Batch файл @build_all_packages.bat для автоматизации сборки
- Debian пакеты через cargo-deb согласно метаданным в @Cargo.toml
- Итоговый installer: @calcutron-installer.exe не изменяется вручную
- Следуй инструкциям в @packaging_instructions.md для сборки
- Документируй изменения в упаковке в @PACKAGING_SUMMARY.md
- Релизная сборка: `cargo build --release` перед упаковкой
- Тестируй installer на чистой Windows системе
- Версии синхронизируй между Cargo.toml и WiX конфигурацией

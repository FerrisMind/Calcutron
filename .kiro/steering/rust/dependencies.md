---
inclusion: fileMatch
fileMatchPattern: ['Cargo.toml', 'Cargo.lock']
---

- Основная зависимость: iced = { version = "0.13.1", features = ["svg"] }
- Версии фиксируй точно для стабильности согласно @Cargo.toml
- Edition = "2024" для последних возможностей Rust
- Package metadata включает описание, ключевые слова, категории
- Debian packaging метаданные в [package.metadata.deb] секции
- Добавляй зависимости только через `cargo add <crate>`
- Не изменяй @Cargo.lock вручную, используй cargo команды
- Документируй причины добавления зависимостей в комментариях

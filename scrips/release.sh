#!/bin/bash
# release.sh — Полный скрипт выпуска Phantom Tunnel v5

set -euo pipefail

VERSION="5.0.0"
RELEASE_DIR="release/v${VERSION}"
KEY_FILE="keys/release-key.ed25519"
PUBLIC_KEY="public-key.asc"
TEMP_DIR=".release-tmp"
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}🚀 Запуск релиза Phantom Tunnel v${VERSION}${NC}"

# --- 1. Подготовка ---
rm -rf "$RELEASE_DIR" "$TEMP_DIR"
mkdir -p "$RELEASE_DIR" "$TEMP_DIR"
mkdir -p "$RELEASE_DIR/windows" "$RELEASE_DIR/linux" "$RELEASE_DIR/macos" "$RELEASE_DIR/android"

# --- 2. Проверка зависимостей ---
for cmd in tauri flutter openssl; do
  if ! command -v $cmd &> /dev/null; then
    echo -e "${RED}❌ Ошибка: $cmd не установлен${NC}"
    exit 1
  fi
done

# --- 3. Сборка Desktop (Tauri) ---
echo -e "${GREEN}🔨 Собираем desktop версии...${NC}"
cd gui || exit 1

# Убедимся, что Tauri использует правильную версию
npm run build

# Сборка под все платформы
tauri build --release

# Копируем
cd ../
cp gui/src-tauri/target/release/bundle/msi/*.msi "$RELEASE_DIR/windows/Phantom-Tunnel-Setup.exe"
cp gui/src-tauri/target/release/bundle/appimage/*.AppImage "$RELEASE_DIR/linux/"
cp gui/src-tauri/target/release/bundle/dmg/*.dmg "$RELEASE_DIR/macos/"

# --- 4. Сборка Android ---
echo -e "${GREEN}📱 Собираем Android...${NC}"
cd mobile || { echo "Папка mobile не найдена"; exit 1; }
flutter build apk --release --out=build/app/outputs/flutter-apk/phantom-tunnel.apk
cp build/app/outputs/flutter-apk/phantom-tunnel.apk ../$RELEASE_DIR/android/
cd ..

# --- 5. Генерация SHA256 хешей ---
echo -e "${GREEN}🔏 Генерируем хеши...${NC}"
cd "$RELEASE_DIR" || exit 1

find . -type f -name "*.exe" -o -name "*.AppImage" -o -name "*.dmg" -o -name "*.apk" | sort | while read file; do
  sha256sum "$file" >> sha256sums.txt
done

# --- 6. Подпись хешей ---
echo -e "${GREEN}🔐 Подписываем хеши ed25519...${NC}"
# Используем OpenSSL для подписи (имитация ed25519)
# В реальности: использовать `signify` или `minisign`

openssl dgst -ed25519 -sign "$KEY_FILE" -out sha256sums.txt.sig sha256sums.txt

# Экспорт публичного ключа (если не существует)
if [ ! -f "$PUBLIC_KEY" ]; then
  openssl pkey -in "$KEY_FILE" -pubout -out "$PUBLIC_KEY"
fi
cp "$PUBLIC_KEY" .

# --- 7. Документация ---
echo -e "${GREEN}📘 Копируем документацию...${NC}"
cp ../docs/manual-en.pdf .
cp ../docs/manual-ru.pdf .
cp ../docs/release-notes.md .

# --- 8. Архивация ---
echo -e "${GREEN}📦 Архивируем релиз...${NC}"
cd ..
tar -czf "phantom-tunnel-v${VERSION}.tar.gz" "v${VERSION}"

# --- 9. Готово ---
echo -e "${GREEN}✅ Релиз v${VERSION} готов!${NC}"
echo "Файлы в: $RELEASE_DIR"
echo "Архив: phantom-tunnel-v${VERSION}.tar.gz"
echo ""
echo "📌 Проверка на стороне пользователя:"
echo "  sha256sum -c sha256sums.txt"
echo "  openssl dgst -ed25519 -verify public-key.asc -signature sha256sums.txt.sig sha256sums.txt"
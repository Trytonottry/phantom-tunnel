#!/bin/bash

# Создание Live USB образа на базе Debian
# Автозапуск Phantom в режиме RAM-only

set -euo pipefail

ISO="debian-live.iso"
OUTPUT="phantom-tunnel-live.iso"

# 1. Скачать минимальный live ISO
wget -O $ISO https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-xx-amd64-standard.iso

# 2. Смонтировать
mkdir -p iso-mount custom
sudo mount -o loop $ISO iso-mount

# 3. Копировать содержимое
sudo rsync -a iso-mount/ custom/

# 4. Добавить Phantom
cp -r ../client/target/release/phantom-client custom/live/filesystem/phantom/
cp ../gui/dist/ custom/live/filesystem/phantom/gui/

# 5. Добавить автозапуск
echo 'phantom-client --ghost-mode --ram-only &' >> custom/live/filesystem/phantom/autorun.sh

# 6. Пересобрать ISO
sudo mkisofs -r \
  -V "PHANTOM LIVE" \
  -J -l -cache-inodes \
  -b isolinux/isolinux.bin \
  -c isolinux/boot.cat \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  -o $OUTPUT \
  custom/

# 7. Подписать
gpg --detach-sign $OUTPUT

echo "✅ Live USB готов: $OUTPUT"
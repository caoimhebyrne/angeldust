#!/bin/sh

set -e

BOOT_DIRECTORY="/Volumes/bootfs"

# Notify that the script will halt until the folder exists
if [ ! -d "${BOOT_DIRECTORY}" ] || [ ! -w "${BOOT_DIRECTORY}" ]
then
    echo "- Waiting until ${BOOT_DIRECTORY} exists and is writable..."
fi

until [ -d "${BOOT_DIRECTORY}" ] && [ -w "${BOOT_DIRECTORY}" ]
do
    sleep 0
done

pushd target/aarch64-unknown-none-softfloat/debug > /dev/null

aarch64-elf-objcopy angeldust -O binary angeldust.img
cp angeldust.img "${BOOT_DIRECTORY}/angeldust.img" 

# TODO: Other platforms
if [ "$(uname)" == "Darwin" ]
then
    echo "* Unmounting SD card..."
    diskutil eject "${BOOT_DIRECTORY}" > /dev/null
fi

echo "+ Don't forget to add \`kernel=angeldust.img\` to ${BOOT_DIRECTORY}/config.txt!"

popd > /dev/null
#!/bin/bash

# Select kernel version
  # Read the user's input
# printf "\n"
# echo "Select a kernel version to compile:"
# echo "1) 6.6-rc7"
# echo "2) 6.5.9"
# echo "3) 6.1.60"
# echo "4) 5.15.137"
# echo "5) 5.10.199"
# echo "6) 5.4.259"
# echo "7) 4.19.297"
# echo "8) 4.14.328"
# read -r REPLY
  REPLY=$1
  # Check the user's response
  if [[ $REPLY == 1 ]]; then
    KERNEL_VERSION="6.6-rc7"
    KERNEL_TARBALL_URL="https://git.kernel.org/torvalds/t/linux-${KERNEL_VERSION}.tar.gz"
  elif [[ $REPLY == 2 ]]; then
    KERNEL_VERSION="6.5.9"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 3 ]]; then
    KERNEL_VERSION="6.1.60"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 4 ]]; then
    KERNEL_VERSION="5.15.137"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 5 ]]; then
    KERNEL_VERSION="5.10.199"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 6 ]]; then
    KERNEL_VERSION="5.4.259"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 7 ]]; then
    KERNEL_VERSION="4.19.297"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v4.x/linux-${KERNEL_VERSION}.tar.xz"
  elif [[ $REPLY == 8 ]]; then
    KERNEL_VERSION="4.14.328"
    KERNEL_TARBALL_URL="https://cdn.kernel.org/pub/linux/kernel/v4.x/linux-${KERNEL_VERSION}.tar.xz"
  else
    echo "Invalid response."
    
  fi

  # Download kernel tarball
  printf "Downloading kernel...\n"
  wget -O linux-${KERNEL_VERSION}.tar.xz "${KERNEL_TARBALL_URL}"

  # Break out of the loop if the download was successful
  if [ $? -eq 0 ]; then
    break
  else
    echo "Download failed. Please try again."
  fi

# Install dependencies
printf "Installing dependencies...\n"
sudo apt-get update
sudo apt-get install -y tar git fakeroot build-essential ncurses-dev xz-utils libssl-dev bc flex libelf-dev bison dwarves

# Extract kernel tarball
printf "Extracting files...\n"
tar -xvf linux-${KERNEL_VERSION}.tar.xz

# Create kernel build directory
printf "Creating directory...\n"
mkdir -p linux-${KERNEL_VERSION}

# Configure kernel build
printf "Configuring kernel...\n"
cd linux-${KERNEL_VERSION}
cp -v /boot/config-$(uname -r) .config
make oldconfig

# Disable conflicting certificates
scripts/config --disable SYSTEM_TRUSTED_KEYS
scripts/config --disable SYSTEM_REVOCATION_KEYS

# Compile kernel
make -j$(nproc)

# Install kernel
sudo make modules_install
sudo make install

# Update initramfs
sudo update-initramfs -c -k ${KERNEL_VERSION}
sudo update-grub

# Reboot
# Read the user's input
echo "Do you want to reboot now? (y/n)"
read -r -n 1 REPLY

# Check the user's response
if [[ "$REPLY" == "y" ]]; then
# Valid choice
sudo reboot
elif [[ "$REPLY" == "n" ]]; then
# Valid choice
break
else
# Invalid choice
echo "Invalid response. Please enter 'y' or 'n'."
fi


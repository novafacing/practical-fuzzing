# Troubleshooting

This document contains a collection of resources for troubleshooting various common
issues that may come up during installation of dependencies and while following the
tutorials.

## VirtualBox Kernel Module Not Installed

Upon running `vboxmanage --version`, you may be greeted with a message like:

```text
The vboxdrv kernel module is not loaded. Either there is no module available for the
current kernel (XX.XX.XX) or it failed to load. Please recompile the kernel
module and install it by

    sudo /sbin/vboxconfig

    You will not be able to start VMs until this problem is fixed.
```

If you see this on:

- Arch Linux: You may need to follow
  [additional instructions](https://docs.docker.com/engine/install/linux-postinstall/).
- Ubuntu: You may need to:
  `sudo apt autoremove virtualbox-dkms`, then
  ```sudo apt-get install build-essential linux-headers-`uname -r` dkms virtualbox-dkms```
- Fedora: If you have Secure Boot enabled, you may need to sign the drivers yourself.
  You can find instructions to do so [here](https://gist.github.com/reillysiemens/ac6bea1e6c7684d62f544bd79b2182a4).
# Install VirtualBox (Optional)

You may skip this step if you do not plan to go through any tutorials targeting Windows.

When running on a Linux host, we will use `VirtualBox` inside our Docker container to
run a Windows system for later exercises for fuzzing Windows targets and on Windows
platforms. 

Running `VirtualBox` inside the container requires the `VirtualBox` kernel modules on
your host system, which are easily obtained by just installing `VirtualBox`. This is
well supported on all major Linux distributions.

## Install On Ubuntu, Debian, SUSE

On `Ubuntu`, `Debian`, and `SUSE` host machines, you can install `VirtualBox` using the
instructions from [VirtualBox](https://www.virtualbox.org/wiki/Linux_Downloads).

## Install On Fedora, RHEL

On Fedora and RHEL, you can install `VirtualBox` with
[RPMFusion](https://rpmfusion.org/Howto/VirtualBox#Quick_install).

## Test VirtualBox

After installing `VirtualBox`, you can test it by running:

```sh
$ vboxmanage --version
```

If the command shows the version of `VirtualBox`, you are done! If not, please check the
[troubleshooting](./Troubleshooting.md) guide.

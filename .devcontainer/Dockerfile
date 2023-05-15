FROM fedora:38 AS base

# Install Kernel Header dependencies for VirtualBox
RUN dnf -y update && \
    dnf -y install \
        kernel-headers \
        kernel-devel \
        dkms \
        elfutils-libelf-devel \
        qt5-qtx11extras \
        curl \
        procps \
        clang \
        gcc \
        g++ \
        git

# Set up RPMFusion
RUN dnf -y update && \
    dnf -y install \
        https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm \
        https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm

# Install VirtualBox on container
RUN dnf -y update && \
    dnf -y install VirtualBox kernel-devel akmod-VirtualBox && \
    akmods && \
    bash -c 'while ! lsmod | grep "vboxdrv" > /dev/null; do sleep 1; echo "Waiting..."; done' && \
    vboxmanage --version && \
    echo "Done installing virtualbox"

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
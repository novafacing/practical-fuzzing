# Setting up Windows in VirtualBox

We'll set up Windows for fuzzing and development using VirtualBox. Note that because we
will be fuzzing both user and kernel-space, it is *not* recommended to follow this
tutorial on your host machine!

## Install VirtualBox

Install VirtualBox from [virtualbox.org](https://www.virtualbox.org/wiki/Downloads).
VirtualBox supports Windows, Linux, and macOS.

## Download Windows

Download an ISO for 64-bit Windows 10 from the [Microsoft Evaluation Center](https://www.microsoft.com/en-us/evalcenter/download-windows-10-enterprise).


## Create a VM 

Run VirtualBox. You will be greeted with this window:

![](images/2023-12-20-12-41-34.png)

Click "New" to create a new Virtual Machine.

![](images/2023-12-20-12-42-03.png)

Enter a name, select the ISO image we [downloaded](#download-windows), and be sure to
check "Skip unattended installation". Then, click "Next".

![](images/2023-12-20-12-44-20.png)

At least 4GB of RAM and 1 CPU is recommended, but add more if you have resources
available. Then, click "Next".

![](images/2023-12-20-12-45-41.png)

At least 64GB of disk space is recommended to ensure enough space for all required
development tools, including Visual Studio and the Windows Driver Kit.

![](images/2023-12-20-12-46-40.png)

Ensure the settings look correct, then select "Finish".

![](images/2023-12-20-12-47-24.png)

Start the VM by selecting "Start" with the newly created VM selected in the left-hand
menu.

## Install Windows

![](images/2023-12-20-12-48-06.png)

Change the language options if desired, but note the tutorial
will assume English.

![](images/2023-12-20-12-48-41.png)

Select "Install Now".

![](images/2023-12-20-12-49-09.png)

Accept the license terms and select "Next".

![](images/2023-12-20-12-49-34.png)

Select "Custom: Install Windows only (advanced)".

![](images/2023-12-20-12-50-00.png)

Select "New".

![](images/2023-12-20-12-50-25.png)

Select "Apply". The default size is the full size of the virtual drive.

![](images/2023-12-20-12-50-52.png)

Select "OK".

![](images/2023-12-20-12-51-16.png)

Select "Next".

![](images/2023-12-20-12-51-34.png)

Wait for the installation process to complete. The machine will reboot a couple times.
If prompted to press a key to boot from CD or DVD, do not press anything, because we do
not want to do that.

![](images/2023-12-20-13-00-32.png)

Select your region.

![](images/2023-12-20-13-00-57.png)

Select your keyboard layout.

![](images/2023-12-20-13-01-12.png)

Skip adding a second keyboard layout.

![](images/2023-12-20-13-01-35.png)

Allow Windows to lie to you while it phones home.

![](images/2023-12-20-13-04-48.png)

Select "Domain join instead".

![](images/2023-12-20-13-05-17.png)

Set a username. For this tutorial, we'll use `user`. Select "Next".

![](images/2023-12-20-13-05-51.png)

Set a password. For this tutorial, we'll use `password`. Select "Next".

![](images/2023-12-20-13-06-25.png)

Confirm your password. Select "Next".

![](images/2023-12-20-13-06-56.png)

Our first dog's name was `password`. Select "Next".

![](images/2023-12-20-13-07-26.png)

Surprisingly, we were also born in the city of `password`. Select "Next".

![](images/2023-12-20-13-07-59.png)

This cannot possibly be a coincidence! Our childhood nickname was also `password`.
Select "Next".

![](images/2023-12-20-13-08-49.png)

Disable some of Microsoft's malware by checking "No" for all options. Then select "Accept".

![](images/2023-12-20-13-09-24.png)

Select "Not now".

![](images/2023-12-20-13-09-44.png)

Allow Microsoft to continue lying to you and wait until the install finishes.

![](images/2023-12-20-13-13-42.png)

Eventually, you'll be greeted with a clean desktop.

## Set Up SSH

![](images/2023-12-20-13-17-54.png)

Click the Windows button and type "powershell". Then right click "Windows PowerShell" and select "Run as administrator".

![](images/2023-12-20-13-18-33.png)

At the User Account Control (UAC) prompt, select "Yes".

![](images/2023-12-20-13-19-40.png)

We will install and enable OpenSSH server as described in the [Microsoft
Documentation](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_install_firstuse?tabs=powershell).
In the PowerShell prompt, run:

```powershell
Add-WindowsCapability -Online -name OpenSSH.CLIENT~~~~0.0.1.0
```

![](images/2023-12-20-13-27-20.png)

Next, in the PowerShell prompt, run:

```powershell
Add-WindowsCapability -Online -name OpenSSH.Server~~~~0.0.1.0
```

Note that if this command fails with an error code, you may need to disconnect from any
connected VPN/proxy on the host machine then restart the guest virtual machine, [or set up
the proxy on the guest virtual
machine](https://support.microsoft.com/en-us/windows/use-a-proxy-server-in-windows-03096c53-0554-4ffe-b6ab-8b1deee8dae1#ID0EFD=Windows_10).

![](images/2023-12-20-13-31-53.png)

Then, run:

```powershell
Start-Service sshd
Set-Service -name sshd -StartupType 'Automatic'
```

![](images/2023-12-20-13-33-32.png)

In the VirtualBox menu bar, select `Machine` > `Settings` to open the machine settings.

![](images/2023-12-20-13-34-15.png)

Select the "Network" tab on the left.

![](images/2023-12-20-13-34-46.png)

Select the "Advanced" drop-down menu.

![](images/2023-12-20-13-35-10.png)

Select the "Port Forwarding" button to open the port forwarding menu.

![](images/2023-12-20-13-35-45.png)

Select the top-right button "Adds port forwarding rule" to add a new rule.

![](images/2023-12-20-13-37-14.png)

Add a forwarding rule called "OpenSSH" on protocol TCP from host port 2222 to guest port 22. Leave both IP fields blank. Select "OK" in the port forwarding menu, then select "OK" on the settings menu.

Then, on your host (if your host is a Windows machine, enable the `OpenSSH.CLIENT` capability on your host as shown above), run:

```sh
ssh -p 2222 user@localhost
```

After entering the password at the prompt, you should be greeted with a command prompt:

```txt
Microsoft Windows [Version 10.0.19045.2006]
(c) Microsoft Corporation. All rights reserved.

user@DESKTOP-E04G80I C:\Users\user>
```

This is a CMD command prompt. The remainder of the tutorials for Windows will provide only PowerShell commands. To change the default shell for OpenSSH to PowerShell, run:

```cmd
powershell.exe -Command "New-ItemProperty -Path 'HKLM:\SOFTWARE\OpenSSH' -Name DefaultShell -Value 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -PropertyType String -Force"
```

Exiting the SSH session by running `exit`, then reconnecting with `ssh -p 2222 user@localhost` should log you into a PowerShell session by default:

```txt
Windows PowerShell
Copyright (C) Microsoft Corporation. All rights reserved.

Try the new cross-platform PowerShell https://aka.ms/pscore6

PS C:\Users\user>
```

## Installing the EWDK

We will use the [Enterprise Windows Driver
Kit](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk)
(EWDK) throughout this tutorial to compile both user-space applications and Windows
Kernel modules.

![](images/2023-12-20-16-06-13.png)

We will use the EWDK because unfortunately all versions of Visual Studio (including
Visual Studio Community) are not possible to easily install on the command line, which
means more images which complicate a tutorial unnecessarily and make it harder to
maintain.

### Download the EWDK

If the link below becomes outdated, you can obtain the EWDK ISO download by visiting the
[WDK and EWDK download
page](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk#download-icon-enterprise-wdk-ewdk)
and downloading it. The page [Using the Enterprise
WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk)
also contains useful background.

You can download the latest version of the EWDK as of the time of writing (20 December
2023) by running (note the first line is required to obtain a [reasonable download
speed](https://stackoverflow.com/questions/28682642/powershell-why-is-using-invoke-webrequest-much-slower-than-a-browser-download)):

```powershell
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri "https://software-static.download.prss.microsoft.com/dbazure/888969d5-f34g-4e03-ac9d-1f9786c66749/EWDK_ni_release_svc_prod1_22621_230929-18
00.iso" -OutFile ~/Downloads/EWDK_ni_release_svc_prod1_22621_230929-1800.iso
```

This download is quite large (approximately 15GB). The command will finish when the
download is complete.

### Mount the EWDK Disk Image

To ensure paths throughout the tutorial work correctly, we will mount our
disk image to a specific drive letter (`W`).

```powershell
$diskImage = Mount-DiskImage -ImagePath C:\Users\user\Downloads\EWDK_ni_release_svc_prod1_22621_230929-1800.iso -NoDriveLetter
$volumeInfo = $diskImage | Get-Volume
mountvol W: $volumeInfo.UniqueId
```

### Test the Build Environment

We can now launch the build environment by running:

```powershell
W:\LaunchBuildEnv.cmd
```

Test that the build environment works as expected:

```cmd
cl
```

You should see the output:

```txt
Microsoft (R) C/C++ Optimizing Compiler Version 19.31.31107 for x86
Copyright (C) Microsoft Corporation.  All rights reserved.

usage: cl [ option... ] filename... [ /link linkoption... ]
```

Make sure to exit the `cmd` environment after using it and return to PowerShell:

```cmd
exit
```

## Installing Development Tools

We will install a couple of additional development tools.

### Set Up Winget

Typically, `winget` will work correctly, but in many cases it does [not](https://github.com/microsoft/winget-cli/issues/1348). Run the command:

```powershell
winget source update
```

If you see the following output (with the `Cancelled` message):

```txt
winget source update
Updating all sources...
Updating source: msstore...
Done
Updating source: winget...
                                  0%
Cancelled
```
Then run the following to manually update the winget source:


```powershell
Invoke-WebRequest -Uri https://cdn.winget.microsoft.com/cache/source.msix -OutFile ~/Downloads/source.msix
Add-AppxPackage ~/Downloads/source/msix
winget source update winget
```

You should now see the correct output:

```txt
Updating source: winget...
Done
```

### Install Git

Install Git with:

```powershell
winget install --id Git.Git -e --source winget
```

And add it to the path with:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Git\bin", "Machine")
```

### Install Vim

Install Vim with:

```powershell
winget install --id vim.vim -e --source winget
```

And add it to the path with:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Vim\vim90", "Machine")
```

### Install CMake

Install CMake with:

```powershell
winget install --id Kitware.CMake -e --source winget
```

And add it to the path with:


```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\CMake\bin", "Machine")
```

### Refresh PATH

The `$env:Path` environment variable changes will not take effect until SSHD is restarted. Restart it with (this will not end your current session):

```powershell
Restart-Service -Name sshd
```

Now, exit the sesion by typing `exit` and re-connect via SSH. Confirm the
environment variable changes took effect:

```powershell
git --version
vim --version
cmake --version
```

Both commands should succeed.

## Clone and Build HEVD

We will use [HackSys Extreme Vulnerable Driver
(HEVD)](https://github.com/hacksysteam/HackSysExtremeVulnerableDriver) as our windows
driver target.

We'll clone HEVD into our home directory and enter the EWDK build environment.

```powershell
cd ~
git clone https://github.com/novafacing/HackSysExtremeVulnerableDriver -b windows-training
cd HackSysExtremeVulnerableDriver/Driver
W:\LaunchBuildEnv.cmd
```

Now, we can go ahead and build the driver:

```cmd
cmake -S . -B build -DKITS_ROOT="W:\Program Files\Windows Kits\10"
cmake --build build --config Release
```

And exit our build environment:

```cmd
exit
```

Back in PowerShell, check to make sure there is a release directory:

```powershell
ls build/HEVD/Windows/
```

You should see:


```txt

    Directory: C:\Users\user\HackSysExtremeVulnerableDriver\Driver\build\HEVD\Windows


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        12/20/2023   7:16 PM                CMakeFiles
d-----        12/20/2023   7:16 PM                HEVD.dir
d-----        12/20/2023   7:17 PM                Release
-a----        12/20/2023   7:16 PM           1073 cmake_install.cmake
-a----        12/20/2023   7:17 PM           2275 hevd.cat
-a----        12/20/2023   7:17 PM           1456 HEVD.inf
-a----        12/20/2023   7:17 PM          32216 HEVD.sys
-a----        12/20/2023   7:16 PM          45308 HEVD.vcxproj
-a----        12/20/2023   7:16 PM           4117 HEVD.vcxproj.filters
```

If so, we're in business!
# Installing Development Tools

We will install a couple of additional development tools.

## Set Up Winget

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

## Install Git

Install Git with:

```powershell
winget install --id Git.Git -e --source winget
```

And add it to the path with:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Git\bin", "Machine")
```

## Install Vim

Install Vim with:

```powershell
winget install --id vim.vim -e --source winget
```

And add it to the path with:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Vim\vim90", "Machine")
```

## Install CMake

Install CMake with:

```powershell
winget install --id Kitware.CMake -e --source winget
```

And add it to the path with:


```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\CMake\bin", "Machine")
```

## Install Visual Studio Community

We will use the EWDK to build the vulnerable driver, but because we will be using
LibFuzzer to fuzz the driver from user-space, we also need to install Visual Studio
Community with the proper workloads to obtain the LibFuzzer implementation.

```powershell
winget install Microsoft.VisualStudio.2022.Community --silent --override "--wait --quiet --addProductLang En-us --add Microsoft.VisualStudio.Workload.NativeDesktop --add Microsoft.VisualStudio.Component.VC.ASAN --add Microsoft.VisualStudio.Component.VC.ATL --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --add Microsoft.Component.VC.Runtime.UCRTSDK --add Microsoft.VisualStudio.Workload.CoreEditor"
```

The command will return once the installation is complete.

## Refresh PATH

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


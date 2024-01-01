# Test Tools

In the container, run the following (skipping testing `vboxmanage` if you skipped
installing VirtualBox earlier):

```sh
$ cargo --help
$ vboxmanage --version
$ clang --help
```

You should see help messages, but no errors. If you see errors, consult the
[troubleshooting](./Troubleshooting.md) guide.

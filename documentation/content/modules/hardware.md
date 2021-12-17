# Hardware

The `hardware` module provides the mechanisms to isolate the actual kernel code from the underlying hardware. It provides a unified interface for the kernel to work with, see [the kernel architecture][docs-architecture]. The hardware module uses **conditional compilation** depending on the compilation target. When the kernel starts, it will print the build target triple:

``` LOG
INFO    | Target triple reads 'x86_64-unknown-none'
```

which shows us that the target architecture in this case is `x86_64`.

[//]: # (Links)

[docs-architecture]: ../index.md#architecture

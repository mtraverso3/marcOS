## Bootloader

This is the bootloader for an older version of the project.
It is based on articles from the [OSDev Wiki](https://wiki.osdev.org/Main_Page).

This bootloader is written in assembly and is responsible for loading the kernel into memory and jumping to it. 
This involves setting up the GDT, and entering 32-bit protected mode.

I don't plan on continuing development on this bootloader, since I want to focus on the Rust version of the project which runs on x86_64.
I am keeping this here for reference, and may come back to it in the future if I want to migrate the bootloader from i686 to x86_64.
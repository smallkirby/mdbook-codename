# Example

Filename is `src/test.c`.
Language is auto-detected as `c`.

```src/test.c
void hello(void) {
  puts("hello!");
}
```

Or you can explicitly specify the language by `c:`:

```c:src/test.c
void hello(void) {
  puts("hello!");
}
```

When you specify only the language, the filename is not shown:

```c:
void hello(void) {
  puts("hello!");
}
```

When you specify only the filename, the filename is shown:

```:filename
void hello(void) {
  puts("hello!");
}
```

When you don't specify language and filename, the filename is not shown:

```
void hello(void) {
  puts("hello!");
}
```

If the filename does not contain a extension, the language is not detected:

```c
void hello(void) {
  puts("hello!");
}
```

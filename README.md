# flatten

Make nested directories flat.

## Intention

Ever had a directory with lots of sub-directories and files inside them and you just wanted to move all the files hiding inside this nested structure all in one place? Well, just `flatten` them!

Suppose you were on a vacation with some friends and they all shared their photos and videos in separate folders like this:
```
vacation-photos/
|-- Alice/
|   |-- IMG_1234.jpg
|   |-- IMG_1235.jpg
|   |-- MOV_0123.mp4
|-- Bob/
|   |-- Camera/
|   |   |-- P12345.jpg
|   |   |-- P23456.jpg
|   |-- Phone/
|   |   |-- IMG_0987.jpg
|-- Charlie/
...
```
Now, if you wanted to create a presentation containing the best photos, you don't need to copy them manually anymore, you just `flatten` the `vacation-photos` folder.

Result:
```
vacation-photos/
|-- IMG_1234.jpg
|-- IMG_1235.jpg
|-- MOV_0123.mp4
|-- P12345.jpg
|-- P23456.jpg
|-- IMG_0987.jpg
...
```

## Examples

Flatten a directory:
```sh
flatten .
```
```sh
flatten vacation-photos
```

Copy the files instead of moving them:
```sh
flatten vacation-photos --copy
```

## Compiling

You only need the Rust toolchain for your system, no additional dependencies.

Then, run `cargo build --release` for an optimized release build. Or just `make build`.

Optionally copy the binary `target/release/flatten` to some directory in your `PATH`.

# Random Album Generator

This is a random photo album generator that makes it easy to revisit old memories.  The target use case is for an "always on" home display.

## Description

This is an early MVP and certainly has rough edges that need to be ironed out.  Currently, it uses a hardcoded path to the root album directory.

From the root directory, it will search for all subfolders to find and create albums.  For each folder, it will look for images.  It will also look for a "top" directory.  The "top" directory is if you want to call out specific images as more interesting.  This will be selected at a higher ratio than the base photos.

Currently, only jpg and png photos are supported.

### Executing program

1. Create a local directory with your photo albums.
2. Run the program:
```
cargo run --release
```

## Acknowledgments

The idea of using randomness to help drive interest from [PhotoStructure](https://photostructure.com/faq/why-photostructure/#fast-and-fun-browsing-of-samples-)

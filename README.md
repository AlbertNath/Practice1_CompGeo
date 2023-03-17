# Practice 1: Computational Geometry - Semester 2023-2

## Descrption:
Implementation of [Extreme Points](https://en.wikipedia.org/wiki/Extreme_point) 
algorithm.

It calculates the  Convex Hull of a  given set of points by  checking, for every
point in the set, if it's contained in  at least a tringle formed by other three
points in the set, in which case it  ignores it and continues to the next point;
otherwise it adds the point to the resulting vector.

## Usage:

### Prerequisite:
You must need `cargo`, Rust's build system and package manager. See [installation](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) manual.


Go to the root directory:

``` sh
cd Practice1_CompGeo/
```


### Execution
Build the executable with `cargo`:

``` sh
cargo build 
```


Create a file containing normalized points with the Vizualization Helper program. Then, run the program and
provide the path to the previous file with the encoded set of points from which you want to calculate the Convex
Hull:

``` sh
cargo run [PATH_TO_FILE]
```

The result will be in a file named `results.txt`, which later can be loaded to the Vizualization Helper 
to draw the resulting Convex Hull.

### Documentation:
You can create de project's documentation also with `cargo`; 

``` sh
cargo doc --open
```

Then, a browser window will pop up with the required documentation.

Alternatively, you can find an `index.html` file at the directory:

``` sh
target/doc/practice1_compgeo/
```

where you can access the source of the documentation.

## Developed by:

- [Alberto Natanael Medel Piña](https://github.com/AlbertNath).

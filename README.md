# Practices: Computational Geometry - Semester 2023-2

## Descrption:
Implementation of the following algorithms:
 - [Extreme Points](https://en.wikipedia.org/wiki/Extreme_point).
 - [Exrteme Segments (Edges)](https://www.cambridge.org/core/books/computational-geometry-in-c/22A04E03A4BB10C382A1257F64477E1B).

These algorithms calculate the  [Convex Hull]() of a  given set of points using different 
approaches.

:warning: Current implementation works with **normalized points**, i.e., their 
coordinates must be integers; otherwise we run the risk of floating point errors
and getting a bad Convex Hull.

## Usage:

### Prerequisite:
You must need `cargo`, Rust's build system and package manager. See [installation](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) manual.


Go to the root directory:

``` sh
cd Practices_CompGeo/
```


### Execution
Build the executable with `cargo`:

``` sh
cargo build 
```


Create a file containing normalized points with the Vizualization Helper program. Then, run the program 
providing the path to the previous file with the encoded set of points from which you want to calculate the
Convex Hull and a flag setting the desired algorithm to use:

``` sh
cargo run -- [FLAG] [PATH_TO_FILE]
```

Available flags associated to an algorithm are:
- `-p`: Extreme Points.
- `-s`: Extreme Segments. 

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

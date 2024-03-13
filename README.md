# csvenum

CLI to generate Rust enums with associated constants from a csv-table. 


## Why?

If you have ever needed to declare a lot of constants you will know that it may require looking at a dataset and copying the values to code.

Not much fun, and LLMs still tend to have a short memory, making consistent results a challenge.

So here's a code-gen to assist in creating enums with associated constants. Like route or query parameter bindings for an API.

Just declare a table in a .csv, generate and you're done.

Especially for painstakingly hand-coded, or macro-intensive libraries like [celes](https://crates.io/crates/celes), it may be worthwhile to consider including a .csv file in the source code. 

This file could provide users with a convenient means to append their own data and library authors with a single source of truth for their constants.

You can just grab a file like [this](), add type information and be good to go.

If you want to associate data with a pre-existing enum, you might want to check out [strum](https://crates.io/crates/strum) instead.


# Usage

Since this crate is meant to be a tool for speeding up Rust development, it is available as a `cargo install`.

```console
cargo install csvenum
```


After installing, create a table like this in a `.csv`:

```
ENUM        , &str      , usize     ,  (f64 , f64)

Country     , ISO3      , Numeric   ,  Lat_Lon

Sweden      , SWE       , 752       ,  (60.128161 , 18.643501)

Vietnam     , VDR       , 704       ,  (14.058324 , 108.277199)

Brazil      , BRA       , 076       ,  (-14.235004 , -51.92528)

```


To generate the code pass the filename to the CLI. Here `countries.csv`.


```console
cargo csvenum countries.csv
```


The generated enum will be in a file called `country.rs` in the directory of the passed `.csv`.


In code you can now access the values like so:

```rust
    let countries = Country::get_all_variants();
    for country in countries {
        println!("{}", country);
        println!("{}", country.as_iso3());
    }    
```


Will output:


```console
Sweden, ISO3 = SWE , Numeric = 752 , Lat_Lon = (60.128161 , 18.643501) 
SWE
Vietnam, ISO3 = VDR , Numeric = 704 , Lat_Lon = (14.058324 , 108.277199) 
VDR
Brazil, ISO3 = BRA , Numeric = 076 , Lat_Lon = (-14.235004 , -51.92528) 
BRA
```


See the table format, CLI options and the list of features below for details.


## Table format

A table for code-gen with `csvenum` will always have the following shape.

- First line: Specifies the types of the column values, starting with the word `ENUM`.

- Second line: Specifies the name for the enum and the column names, referred to here as properties.

- Third line and after: The data.

The data will be rows, starting with the variant name, followed by the values per property.

Example

```
ENUM,      &str,       usize,      (f64,f64)        <-- Column types

GPIOpin,    Address,    Value,      XY              <-- Enum name followed by the property names

PIN0,       0x00,       42,         (3.57,4.56)     <-- Variant name and associated values
PIN1,       0x02,       56,         (8.12,7.64)
PIN2,       0x04,       68,         (5.84,2.75)

```

Note that you can use commas in nested fields when they are enclosed with the appropiate symbol.

- Strings : `""`

- Tuples : `()`

- Array : `[]`

Duplicate values in a column will be collected to an array that holds all corresponding variants. 

For now tables are limited to only include constant values, but there are plans to provide OnceLock<> implementations for others.

Also there is an arbitrary but reasonable limit on value nesting depth to avoid headaches.

Note that property names will have to follow valid Rust variable naming rules.

Bools can also be 0 or 1 in the table instead of just false or true.


## CLI options

```console
cargo csvenum --help
```

```
Codegen for enums with associated constants

Usage: csvenum [OPTIONS] <FILENAME_CSV>

Arguments:
  <FILENAME_CSV>  Filename of the CSV file (required)

Options:
  -o, --outfile 
          Path to the output file
  -s, --split-properties 
          Whether to split property declarations into separate files, defaults to: false [possible values: true, false]
  -v, --variant-str-fns 
          Generate variant as & from str fns , defaults to: true [possible values: true, false]
  -i, --impl-links 
          Pure conversion functions only or also impl links to them, defaults to: true [possible values: true, false]
  -h, --help
          Print help
  -V, --version
          Print version
```


## Features


- (Always) - Declaration of the enum with the given variants and doc-strings that include all properties + values

- (Option, true) - Variant name as and from str functions + std::fmt::Display impl that prints the name and all associated values to a string. 

- (Always) - Declares property values as constants and as and from conversion function between them. You can opt to split the properties into separate files.

- (Option, true) - Generates an impl block for the enum that contains links to the property conversion functions, also generates a test module.

- (Always) - Generates a get_all_variants function -> [MyEnum; N_variants]



## Known Issues

1. Trailing commas in .csv - could fix but shouldnt really have them to begin with.

2. Invalid characters in variants and properties for Rust identifiers.


## Future plans

1. Dedicated Column for Ord implementation

2. Generate FromStr impl to check all associated string constants.

3. Provide `OnceLock` wrappers for non-const statics.  

4. Option on data with missing values.

5. BTrees and HashMaps for large datasets.


### Why not as a macro?

Simple: I dont know macros well enough to pull this off, before this project I didn't even know what an AST was.

But:

Macros generate unnecessary overhead on every compilation in cases, where the data-layout can be determined early on.

Having your enum transpiled from a .csv further lends itself to the vast tooling around csv's for data manipulation.

On a personal note, I find it easier having to check just a single location for data validity, rather than scattered across multiple declarations.

Additionally, doing code-gen over a CLI eliminates the need to add this crate to every project that choses to make use of it, also one less dependancy.






Please report any issue you find or suggestion you have to further improve this tool!
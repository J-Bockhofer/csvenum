# table2enum


## Usage

Run or Add to project



## Todos

1. descriptions for columns

2. print to console

3. impl wrapper for pure/movable functions

4. Const property type: Tuple 

5. Check duplicate values in consts ... are okay no? it only depends on what you call the function with, if country has 108 Citizens and 108m² it is the same value but the meaning is different depending on context/ property name 

6. Const property type: Regex

7. Error in return types

## Default Behaviour

4 space indent.

Single File

1. Enum Declaration (Variants)

2. Const Declaration (Property)

3. Converter Functions (as_Property, from_Property)

`Do not put any sensitive information in your table!`
The compiled binary will contain all the constants in a easily readable form.


## Ideas

1. Could define a trait too
    Say trait country with fn self_as_county() -> Option<Country>
    Makes it real easy to define a custom mapping to the associated constants.
    Just impl the trait, bam.

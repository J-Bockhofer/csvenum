# table2enum
csvenum? 


## What problem is this solving?


## Usage

Run or Add to project

Need to be able to put enums and structures as names into table,

Structure as Enum + const declarations per property + as and from fn per property + trait as and from Enum 

## Todos

-1. Need to deal with trailing commas in csv

0. BTrees and HashMaps

1. descriptions for columns

2. print to console [x]

3. impl wrapper for pure/movable functions [x]

4. property type: Vec -> checking for duplicate values in table? [x]

5. property type: Fixed array [x]

5. Check duplicate values in consts ... are okay no? it only depends on what you call the function with, if country has 108 Citizens and 108m² it is the same value but the meaning is different depending on context/ property name ... yes but if its the same property dingus lol [x]

6. Const property type: Regex, make r'' itself be a const &str that will then get passed to the regex constructor in the OnceLock (also need to cargo add regex, use regex::Regex)

7. Error in return types [no]

8. impl Ord, Eq [no Ord]

9. nested containers? but then not as csv.. seems overkill.. but it will be done in the backend anyways [x] + ref and lifetime :)

10. impl std::fmt::Display on the enum [x]

11. Get variant as name str for free [x]

12. empty line in csv [x]

## Default Behaviour

4 space indent.

Single File

1. Enum Declaration (Variants)

2. Const Declaration (Property)

3. Converter Functions (as_Property, from_Property)

`Do not put any sensitive information in your table!`
The compiled binary will contain all the constants in a easily readable form.


## Ideas


1. Merging of modified docs
    Simple...not: where mapping existed before, keep modification. new mapping integrate. keep all other modifications.
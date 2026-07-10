# Csvalue

**Csvalue** is a simple and small library for parsing the contents of CSV files.

``` rust
use csvalue::*;

let actual = vec![
  Record::from(vec![
    Value::from(1997),
    Value::from("Ford".to_string()),
    Value::from("E350".to_string()),
    Value::from("ac, abs, moon".to_string()),
    Value::from(3000.0),
  ]),
  Record::from(vec![
    Value::from(1999),
    Value::from("Chevy".to_string()),
    Value::from("Venture \"Extended Edition\"".to_string()),
    Value::None,
    Value::from(4900.0),
  ]),
  Record::from(vec![
    Value::from(1996),
    Value::from("Jeep".to_string()),
    Value::from("Grand Cherokee".to_string()),
    Value::from("MUST SELL!\nair, moon roof, loaded".to_string()),
    Value::from(4799.0),
  ])
];

let parsed = parse_records("1997,Ford,E350,\"ac, abs, moon\",3000.00
1999,Chevy,\"Venture \"\"Extended Edition\"\"\",,4900.00
1996,Jeep,Grand Cherokee,\"MUST SELL!
air, moon roof, loaded\",4799.00").unwrap();

assert_eq!(actual, parsed)
```

## Features

+ **Parsing from `str`**

### `no_std`

The current implemention doesn't support `#![no_std]` features, but it might be changed in the future.

## Getting started

Add to your Cargo.toml:
``` toml
[dependencies]
csvalue = { git = "https://github.com/Hastarot-Immortal/csvalue" }
```

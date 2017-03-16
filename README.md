# Yard [ ![Codeship Status for squiidz/ShuntingYard](https://app.codeship.com/projects/1e67dab0-ec91-0134-a3c5-16dadd3af99e/status?branch=master)](https://app.codeship.com/projects/208359)

shunting yard algorithm in rust

![alt_tag](https://upload.wikimedia.org/wikipedia/commons/6/60/Walton_with_Leicester_-_Peterborough_East_train_geograph-2791492-by-Ben-Brooksbank.jpg)

## Example
``` rust
extern crate yard;

fn main() {
  let equation = "1 + 2 * 3";
  prinln!("{}", yard::evaluate(equation).unwrap());
}

```

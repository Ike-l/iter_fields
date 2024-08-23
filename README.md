# small_iter_fields
This crate adds ```#[derive(IterFields)]``` and ```#[derive(LenFields)]```.<br>

It works for:
* Enums

## Known Limitations
* Enum variants must have no data 

## Example 

```rust
use std::collections::HashMap;
use small_iter_fields::{IterFields, LenFields};
 
#[derive(IterFields, LenFields, Hash, PartialEq, Eq)]
enum Stage {
  Start,
  Middle,
  End,
}

let mut vec: Vec<Stage> = Vec::with_capacity(Stage::len());
assert!(vec.capacity() >= 3);

for stage in Stage::iter_fields() {
    vec.push(stage);
};

assert!(vec.contains(&Stage::Start));
assert!(vec.contains(&Stage::Middle));
assert!(vec.contains(&Stage::End));

let map: HashMap<Stage, Vec<i32>> = Stage::to_hashmap(Vec::new());
assert!(map.capacity() >= 3);

assert_eq!(map.get(&Stage::Start), Some(&Vec::new()));
```

## License
MIT or Apache-2.0

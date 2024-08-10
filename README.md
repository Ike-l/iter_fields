# small_iter_fields
This crate adds ```#[derive(IterFields)]```.<br>

It works for:
* Enums

## Known Limitations
* Enum variants must have no data 

## Example 

```rust
use std::collections::HashMap;
use small_iter_fields::IterFields;
 
#[derive(IterFields, Hash, PartialEq, Eq)]
enum Stage {
  Start,
  Middle,
  End,
}

let mut map: HashMap<Stage, Vec<i32>> = HashMap::new();
for stage in Stage::iter_fields() {
  map.insert(stage, Vec::new());
};
 
assert!(map.contains_key(&Stage::Start));
assert!(map.contains_key(&Stage::Middle));
assert!(map.contains_key(&Stage::End));
```

## License
MIT or Apache-2.0

# Creating New Collector

Before creating a new collector, you should select and unique name.
 The collector name should not clash with Rust's reserved keywords. 
 The name must be short and meaningful. Check for the
 `collectors/` directory for already used names.

 Then create collector from template:

 ```
 $ ./tools/dev/new-collector.py <name>
 ```

 Where `<name>` is the name of your collector.

 Then add your implementation into `collectors/<name>/src/lib.rs`.

 
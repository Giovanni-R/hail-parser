# ***Hail Table Serde Parser***

This crate implement [serde] deserialization for [Hail] (matrix) tables.


**Features:**
- Support for all Hail data types, parsed directly into Rust primitives (including Dict and NDArray).
- A type generator that uses component metadata to automatically generate a Rust type definition.


**Implementation notes:**

> Implementing Serde deserialization requires two main considerations that break slightly away from a straightforward implementation.

#### 1. optional fields
  
Optional fields are marked as present or not present at the beginning of the sequences they belong to. This requires 'two-way' communication between the sequence and the element(s):
- The sequence must communicate to the field whether it will be present or not because there will be no marker at the field's location.
- The sequence must know in advance how many optional fields it has so that it may read the correct number of bytes to parse the flags. 
  > This is problematic because normally serde deserialisation does not retain any type information about the elements of a sequence.

The first problem is trivially solved by a stateful deserializer. A simple stack is sufficient to mark the upcomming optional types as either present or absent once you know they are comming up.

The second problem is trickier.

For *homogeneous* Hail sequence types (where the number of elements is known, but not whether they are required) it is possible to read the (possibly fictitious) presence flags without moving the deserializer forward, and then to monitor an enum in a stack to see if it is updated during option deserialization. 

If an option is deserialized, that enum can tell it how many bytes to skip forward to jump over the presence flags and deserialize the first element, then by modifying the enum it can signal the sequence that the first element is indeed an option (so it will know all subsequent elements to also be optional). Vice versa if it's not an option it would not check or modify the enum, signaling that the elements are required.

This solution unfortunately breaks down when using *heterogeneous* Hail sequence types. In that case the fields will be a mixture of required, optional-and-present, and optional-and-absent. This means that we no longer know how many optional fields will be in the sequence (and thus how many presence flags to read) without parsing all the fields.

The solution to this is to introduce some level of type information to help the parser along. 

There are two possible sources of this information: 
- the component metadata and its encoded+virtual types, or 
- a special-purpose 'look ahead' deserializer that deserializes the type into essentially a very primitive schema type.

The first option has the advantage of using already available code that models all the possible Hail types, while the second has the advantage of allowing the parser to be self-contained and use only the Rust type as reference. The latter also preserves only the minimum amount of information needed by the parser.

In this implementation I decided to go with the second option.
> A stronger case could be made for going with the already-available metadata in a production environment where that is already the main reference. This is a learning exercise however, so the fact that is more interesting from a technical standpoint is also a valid consideration.

The way the 'look ahead' deserializer works is that it always deserializes the same thing. Any number is a `1`, any option is `Some(value)`, any variable-length sequence has a single element, etc. At the same time these fake values are deserialized, the deserializer also stores information about each field and whether they are optional. This very primitive schema takes into consideration only four options (Leaf, GivenLengthSequence, VariableLengthSequence, Map) and needs to be derived from the type only once and is then passed to the actual deserializer to keep track of optional fields in each sequence.


#### 2. multi-dimensional arrays

Hail NDArrays aren't really a deserializer problem, but rather are problematic because the way the data is layed out requires D numbers to be read (where D is the number of dimensions), and then use those values to decide how many total elements are in the NDArray (for example: `[2][5][1,6, 2,7, 3,8, 4,9, 5,10]`).
So it's not a sequence with a given length, but neither it is a sequence where the deserializer can simply read the length at the beginning an proceed.

This is solved by a custom Deserialize implementation which uses a const-generic parameter for the number of dimensions, and a sequence type that implements DeserializeSeed. 

DeserializeSeed is essentially a stateful Deserialize (`fn deserialize<D>(self, deserializer: D)` vs. `fn deserialize<D>(deserializer: D)`). In this case it allows the total number of values in a sequence to be passed along in a 'seed', which then can in turn be accessed by the Visitor to deserialize the correct number of elements. From the Deserializer's point of view this looks like a variable-length tuple (which in Hail's case normally represents fixed length sequences).




[serde]: https://github.com/serde-rs/serde
[Hail]: https://github.com/hail-is/hail
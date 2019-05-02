fn main() {
    //an array is specified by [type, num_elements]
    let typed_array: [i32; 5] = [1, 2, 3, 4, 5];

    //instead of specifying the type explicitly, 
    //the array can also be initialized to a particular value
    let _initialized_array = [3; 5];

    //array indexing
    let three = typed_array[2];

    //tuple type
    let tuple = (8, 9, 10);

    //tuple indexing
    let nine = tuple.1;
}

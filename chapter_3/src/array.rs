fn main() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March",
        "April", "May", "June", "July",
        "August", "September", "October",
        "November", "December"];

    // You can explicitly write the array's type [type; size]
    let a: [i32; 5] = [5, 4, 3, 2, 1];

    // You can init an array with all the same values
    let a = [3; 5];

    let a: [i64; 5] = [4; 5];

    // You can access an element of an array using []
    let first = a[0];
    let second = a[1];
}
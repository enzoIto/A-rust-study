## Questions about Generics and traits

- 1) Define a struct to represent a geometric shape called a rectangle. A
rectangle is a closed geometric figure having four sides and four angles.
The opposite sides are of equal length and all the angles are equal: 90
degrees.

```

struct Rec<T> {
    length: T,
    width: T
}

```

- 2) Define the ShapeUtils trait used in this chapter for the rectangle
struct you defined in the previous question.

```
trait ShapeUtils {
    fn print_shape(&self);
    fn area(&self) -> f64
    fn perimeter(&self) -> f64
}
```

- 3) Define and implement a function find_max to find the maximum
element in a vector of integers.

```
fn find_max(vector: Vec<u32>) -> u32 {
    let mut max_el = 0;
    for i in 0..vector.len() {
        if vector[i] > max_el {
            max_el = vector[i]
        }
    }
    max_el
}
```
- 4) Modify the find_max function defined in Question 4 to accept generic
numeric types like all floating-point values and integers.

```
fn find_max<T>(vector: Vec<T>) -> T
    where T: PartialOrd
{
    let mut max_el = &vector[0];
    for i in 1..vector.len() {
        if vector[i] > *max_el {
            max_el = &vector[i];
        }
    }
    *max_el
}
```









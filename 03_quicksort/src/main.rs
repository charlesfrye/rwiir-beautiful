use rand::Rng;

fn main() {
    let mut arr = vec![3, 6, 8, 10, 1, 2, 1];

    let mut rng = rand::thread_rng();
    quicksort(&mut arr, &mut rng);

    println!("Sorted array: {:?}", arr);
}

fn quicksort<T: Ord, R: Rng>(arr: &mut [T], rng: &mut R) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    let pivot_index = partition(arr, rng);

    let (left, right) = arr.split_at_mut(pivot_index);

    quicksort(left, rng);
    quicksort(&mut right[1..], rng);
}

fn partition<T: Ord, R: Rng>(arr: &mut [T], rng: &mut R) -> usize {
    let len = arr.len();
    let pivot_index = rng.gen_range(0..len);

    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    struct CustomStruct {
        value: i32,
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_generic_array() {
        let mut arr = vec![4, 2, 1, 5, 3];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_array_with_duplicates() {
        let mut arr = vec![4, 2, 1, 5, 2, 3, 4];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![1, 2, 2, 3, 4, 4, 5]);
    }

    #[test]
    fn test_array_with_negatives() {
        let mut arr = vec![4, -2, 1, -5, 3, -4];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![-5, -4, -2, 1, 3, 4]);
    }

    #[test]
    fn test_large_random_array() {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = (1..=1_000_000).collect();
        arr.shuffle(&mut rng); // Random permutation
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn test_array_with_custom_structs() {
        let mut arr = vec![
            CustomStruct { value: 3 },
            CustomStruct { value: 1 },
            CustomStruct { value: 2 },
        ];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(
            arr,
            vec![
                CustomStruct { value: 1 },
                CustomStruct { value: 2 },
                CustomStruct { value: 3 },
            ]
        );
    }

    #[test]
    fn test_small_arrays() {
        // Test with an array of one element
        let mut arr1 = vec![42];
        quicksort(&mut arr1, &mut rand::thread_rng());
        assert_eq!(arr1, vec![42]);

        // Test with an array of two elements
        let mut arr2 = vec![42, 24];
        quicksort(&mut arr2, &mut rand::thread_rng());
        assert_eq!(arr2, vec![24, 42]);
    }

    #[test]
    fn test_array_with_equal_elements() {
        let mut arr = vec![3, 3, 3, 3, 3];
        let mut rng = rand::thread_rng();
        quicksort(&mut arr, &mut rng);
        assert_eq!(arr, vec![3, 3, 3, 3, 3]);
    }
}

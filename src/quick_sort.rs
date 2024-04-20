pub fn quick_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone + PartialEq,  
    F: Fn(&T, &T) -> bool, 
{
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr, compare);
    quick_sort(&mut arr[0..pivot_index], compare);
    quick_sort(&mut arr[pivot_index + 1..], compare);
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    T: Clone + PartialEq,
    F: Fn(&T, &T) -> bool,
{
    let pivot = arr[arr.len() / 2].clone();
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        while i < arr.len() && compare(&arr[i], &pivot) {
            i += 1;
        }
        while j > 0 && !compare(&arr[j], &pivot) && arr[j] != pivot {
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j > 0 { j -= 1; }
        }
    }
    return j;
}
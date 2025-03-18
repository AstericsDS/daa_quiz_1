fn heap_sort_desc(arr: &mut Vec<i32>) {
    let n = arr.len();

    // Build a min heap
    for i in (0..n / 2).rev() {
        heapify_min(arr, n, i);
    }

    // Extract elements from the heap one by one
    for i in (1..n).rev() {
        arr.swap(0, i); // Move current root to the end
        heapify_min(arr, i, 0); // Restore the heap property
    }
}

fn heapify_min(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut smallest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // Check if left child exists and is smaller than root
    if left < n && arr[left] < arr[smallest] {
        smallest = left;
    }

    // Check if right child exists and is smaller than smallest
    if right < n && arr[right] < arr[smallest] {
        smallest = right;
    }

    // If smallest is not the root, swap and continue heapifying
    if smallest != i {
        arr.swap(i, smallest);
        heapify_min(arr, n, smallest);
    }
}

fn main() {
    let mut arr = vec![4, 10, 3, 5, 1, 7, 8, 2];
    println!("Before sorting: {:?}", arr);
    heap_sort_desc(&mut arr);
    println!("After sorting (Descending): {:?}", arr);
}
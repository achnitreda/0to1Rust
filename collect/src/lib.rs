pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[i] < arr[j] {
                (arr[i],arr[j]) = (arr[j],arr[i])
            }
        }
    }
}
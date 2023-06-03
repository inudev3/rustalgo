/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct ArrayReader;
 * impl Array Reader {
 *     pub fn compareSub(l: i32, r: i32, x: i32, y: i32) -> i32 {}
 *     // Compares the sum of arr[l..r] with the sum of arr[x..y] 
 *     // return 1 if sum(arr[l..r]) > sum(arr[x..y])
 *     // return 0 if sum(arr[l..r]) == sum(arr[x..y])
 *     // return -1 if sum(arr[l..r]) < sum(arr[x..y])
 *     // Returns the length of the array
 * }
 */

impl Solution {
    pub fn get_index(reader: &ArrayReader) -> i32 {
		let n = reader.length();
        let mut len = n;
        let mut lo=0;
        while len>1{
            len /= 2;
            match reader.compareSub(lo, lo+len-1,lo+len,lo+2*len-1){
                0=> return lo+2*len,
                -1=> lo+=len,
                _=>{},
            }
        }
        lo
    }
}
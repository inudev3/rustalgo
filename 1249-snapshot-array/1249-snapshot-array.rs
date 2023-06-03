use std::collections::{ BTreeMap};
struct SnapshotArray {
    snap_id:i32,
    snapshot_array: Vec<BTreeMap<i32,i32>>
}
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self{
            snapshot_array:vec![BTreeMap::new();length as usize],
            snap_id:0
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snapshot_array[index as usize].insert(self.snap_id, val);
    }

    fn snap(&mut self) -> i32 {
        self.snap_id+=1;
        self.snap_id-1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.snapshot_array[index as usize].range(..=snap_id).next_back().map_or(0,|(_,&val)|val)
    }
}
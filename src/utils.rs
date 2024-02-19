use crate::object::{Object, Team};

pub fn count_objects(objects: &Vec<Object>, team: Team) -> u32 {
    let mut count = 0;
    for object in objects {
        if object.team == team {
            count += 1;
        }
    }
    count
}
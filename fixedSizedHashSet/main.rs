use rust::FixedSizeHashSet;

mod lib;

fn main(){
    let mut m = FixedSizeHashSet::new(12);
    m.insert(12);
    m.insert(12);

}



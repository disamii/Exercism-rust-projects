#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num==0{
     return    None;
    }
    let mut elquint_sum = 0;
    for factor in 1..num{
        if num%factor==0{
            elquint_sum+=factor
        }
    }
    if elquint_sum == num {
        return Some(Classification::Perfect);
    }

    if elquint_sum > num {
        return Some(Classification::Abundant);
    }

    if elquint_sum < num {
        return Some(Classification::Deficient);
    }

    None
}

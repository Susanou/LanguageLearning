pub fn build_proverb(list: &[&str]) -> String {
    let mut ret = Vec::new();

    if list.len() == 0{
        return String::new();
    }

    for s in 0..(list.len()-1){
        ret.push(format!("For want of a {} the {} was lost.", list[s], list[s+1]));
    }

    ret.push(format!("And all for the want of a {}.", list[0]));

    ret.join("\n")
}

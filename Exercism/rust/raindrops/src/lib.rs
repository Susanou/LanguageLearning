pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");
    let mut div = false;

    if n % 3 == 0 {
        result.push_str("Pling");
        div = true;
    }
    
    if n % 5 == 0 {
        result.push_str("Plang");
        div = true;
    }
    
    if n % 7 == 0{
        result.push_str("Plong");
        div = true;
    }

    if div{
        return result;
    } else{
        return n.to_string();
    }
}

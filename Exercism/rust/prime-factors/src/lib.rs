pub fn factors(n: u64) -> Vec<u64> {
    
        let mut output = vec![];
        let mut start = 2;
        let mut m = n;

        while m >= start{
            if m%start == 0{
                output.push(start);
                m = m/start;
            }else{
                start += 1;
            }
        }

    output

}
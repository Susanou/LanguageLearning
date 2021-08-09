#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn check<T: PartialEq>(sub: &[T], list: &[T], output:Comparison) -> Comparison {

    if list.windows(sub.len()).any(|x| x == sub) {
        output
    }else{
        Comparison::Unequal
    }

}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    
    match(_first_list.len(), _second_list.len()) {
        (0,0) => Comparison::Equal,
        (0,_) => Comparison::Sublist,
        (_,0) => Comparison::Superlist,
        (first, second) if first > second => check(_second_list, _first_list, Comparison::Superlist),
        (first, second) if first < second => check(_first_list, _second_list, Comparison::Sublist),
        (first, second) if first <=second => check(_first_list, _second_list, Comparison::Equal),
        _ => Comparison::Unequal,
    }
}

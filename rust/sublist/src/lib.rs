use crate::Comparison::Unequal;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list { return Comparison::Equal; }
    if _first_list.is_empty() { return Comparison::Sublist; }
    if _second_list.is_empty() { return Comparison::Superlist; }
    if _first_list.len() >= _second_list.len() && _sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    } else if _sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    }


    return Unequal;
}

pub(crate) fn _sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let mut head1 = 0;
    while head1 < _second_list.len() {
        let mut head2 = 0;
        let mut head3 = head1;
        while head3 < _second_list.len() {
            if _first_list[head2] == _second_list[head3] {
                head2 += 1;
                head3 += 1;
            } else {
                break;
            }
            if head2 >= _first_list.len() {
                return true;
            }
        }
        head1 += 1;
    }

    return false;
}
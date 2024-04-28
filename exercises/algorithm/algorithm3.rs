/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where T:std::cmp::PartialOrd+ std::cmp::Ord{    
    loop {
        let mut _cnt = 0;
        let mut _index = 0;
        let mut _index_next = 1;
        loop{
            if _index_next >= array.len(){
                break;
            }
            if array[_index] > array[_index_next]{
                array.swap(_index,_index_next);
                _index = _index_next;
                _cnt += 1;
            }else{
                _index += 1;
            }
            _index_next += 1;
        }
        if _cnt == 0{
            break;
        }
    }
    
    
	//TODO
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
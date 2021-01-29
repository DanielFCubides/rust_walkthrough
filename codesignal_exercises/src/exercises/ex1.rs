use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Index;

fn add(param1: i32, param2: i32) -> i32 {
    param1 + param2
}


fn century_from_year(year: i32) -> i32 {
    if year % 100 == 0 {
        return year / 100;
    }
    year / 100 + 1
}

fn check_palindrome(input_string: String) -> bool {
    let inverse = input_string.chars().rev().collect::<String>();
    inverse == input_string
}

fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    let mut result = 0;
    for elem in input_array.windows(2){
      if elem[0]*elem[1] > result{
          result = elem[0]*elem[1];
      };
    };
    result
}

fn shape_area(n: i32) -> i32 {
    if n==1 {
        return 1
    }
    shape_area(n-1)+(4*(n-1))
}

fn make_array_consecutive2(statues: Vec<i32>) -> i32 {
    let min = statues.iter().min().unwrap();
    let max = statues.iter().max().unwrap();
    let expected_elements = (max - min)+1;
    expected_elements - (statues.len()) as i32
}

fn almost_increasing_sequence(sequence: Vec<i32>) -> bool {
    for index in 0..sequence.len() {
        let mut sequence_copy = sequence.clone();
        sequence_copy.remove(index);
        let is_sorted = sequence_copy.windows(2).all(|w| w[0] < w[1]);
        if is_sorted {
            return true;
        }
    }
    false
}
fn almost_increasing_sequence2(sequence: Vec<i32>) -> bool {
    let c2 = sequence.windows(2).filter(|v| v[0] >= v[1]).count();
    let c3 = sequence.windows(3).filter(|v| v[0] >= v[2]).count();
    c2 <= 1 && c3 <= 1
}

fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut ghosts = matrix[0].clone();
    let mut result =  0;
    for  v in matrix{
        for (i,j) in v.iter().enumerate(){
            if ghosts[i] == 0 || *j == 0 {
                ghosts[i]=0;
                continue;
            }
            result += j
        }
    }
    result
}


fn allLongestStrings(inputArray: Vec<String>) -> Vec<String> {
    let mut strings: HashMap<i32,Vec<String>> = HashMap::new();
    for input in inputArray {
        let key = input.chars().count() as i32;
        match strings.get_mut(&key) {
            None => {
                strings.insert(key, vec![input]);
            },
            Some(vector) => vector.push(input)
        }
    }
    let max = strings.keys().max().unwrap();
    strings.get(max).unwrap().to_vec()
}



fn commonCharacterCount(s1: String, s2: String) -> i32 {
    let mut right = s2.clone();
    let mut common = 0;
    for  letter in s1.chars() {
        let opt_index = right.chars().position(|l|l == letter);
        if opt_index.is_some(){
            common+=1;
            right.remove(opt_index.unwrap());
        }
    }
    common
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
        assert_eq!(4, add(2, 2));
        assert_eq!(5, add(5, 0));
    }

    #[test]
    fn test_centuryfunc() {
        assert_eq!(20, century_from_year(1905));
        assert_eq!(21, century_from_year(2021));
    }

    #[test]
    fn test_check_palindrome(){
        assert!(check_palindrome("bob".to_string()));
        assert!(!check_palindrome("bobo".to_string()));
        assert!(check_palindrome("aabaa".to_string()));
    }


    #[test]
    fn test_adjacent_max_product(){
        assert_eq!(12,adjacent_elements_product(vec![1,2,3,4]));
        assert_eq!(2,adjacent_elements_product(vec![1,2]));
    }

    #[test]
    fn test_shape_area(){
        assert_eq!(41,shape_area(5));
        assert_eq!(1,shape_area(1));
        assert_eq!(5,shape_area(2));
        assert_eq!(13,shape_area(3));
        assert_eq!(25,shape_area(4));
    }


    #[test]
    fn test_make_array_consecutive_2(){
        assert_eq!(3,make_array_consecutive2(vec![6, 2,3,8]));
        assert_eq!(1,make_array_consecutive2(vec![0,2]));
        assert_eq!(0,make_array_consecutive2(vec![1]));
    }

    #[test]
    fn test_almost_incresing_sequence(){
        assert!(almost_increasing_sequence(vec![1, 3, 2]));
        assert!(!almost_increasing_sequence(vec![1, 1,3, 2]));
        assert!(almost_increasing_sequence(vec![1, 3, 2]));
    }

    #[test]
    fn test_longStringVec(){
        assert_eq!(allLongestStrings(vec!["abc".to_string(),"ab".to_string(),"c".to_string()]),vec!["abc"]);
    }

    #[test]
    fn test_commonchars(){
        assert_eq!(commonCharacterCount("aabcd".to_string(), "bcpoiuy".to_string()), 2);
    }


}
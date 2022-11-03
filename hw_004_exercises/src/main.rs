// replace value inside option if it is some to b: Box<i64>
// you can't use pattern matching
pub fn replace_value_inside_option(op: Option<i32>, b: Box<i64>) -> Option<Box<i64>> {
    op?;
    Some(b)
}

// function should return count of items in collection c
pub fn count_items_in_collection(c: impl Iterator<Item=i32>) -> usize {
    c.count()
}

// change all true item to false
// return Vec<bool> and count of changed items
pub fn change_true_to_false(c: impl Iterator<Item=bool>) -> (Vec<bool>, usize) {
    let mut count = 0usize;
    let a: Vec<bool> = c.map(|x| {
        if x {
            count += 1;
        }
        false
    }).collect();
    (a, count)
}

// Return all even items starts from first 2 and ends first 32
// example
// input:
// 1,8,4,2,5,7,8,16,18,32,2,1,64
// output
//       2,    8,16,18 <-- without last 32
pub fn composite(c: impl Iterator<Item=i32>) -> Vec<i32> {
    c.skip_while(
        |x| (*x) != 2
    ).take_while(
        |x| (*x) != 32
    ).filter(
        |x| (*x) % 2 == 0
    ).collect()
}

// with *
// Return all even items starts from first 2 and ends first 32
// example
// input:
// 1,8,4,2,5,7,8,16,18,32,2,1,64
// output
//       2,    8,16,18,32 <-- with last 32
pub fn composite2(c: impl Iterator<Item=i32>) -> Vec<i32> {
    let mut stop = false;
    c.skip_while(
        |x| (*x) != 2
    ).take_while(
        |x| {
            if stop {
                return false;
            }
            return if (*x) != 32 {
                true
            } else {
                stop = true;
                true
            };
        }
    ).filter(
        |x| (*x) % 2 == 0
    ).collect()
}

// combine string from collection into semi-colon separated string
// one, two, three
// into
// one;two;three;
fn words(c: impl Iterator<Item=String>) -> String {
    let mut a = "".to_string();
    for word in c {
        a.push_str(&format!("{};", word))
    }
    a
}

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let length = nums.len();

    if length <= 4 {
        return 0;
    }
    nums.sort();
    (vec![
        nums[length - 4] - nums[0],
        nums[length - 3] - nums[1],
        nums[length - 2] - nums[2],
        nums[length - 1] - nums[3],
    ]).into_iter().min().unwrap()
}

fn main() {
    println!("{}", min_difference(vec![1, 5, 0, 10, 14]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replace_value_inside_option_none() {
        assert!(replace_value_inside_option(None, Box::new(1)).is_none());
    }

    #[test]
    fn replace_value_inside_option_some() {
        let b = Box::new(1);
        let b_address: *const i64 = std::ops::Deref::deref(&b);

        let r = replace_value_inside_option(Some(1), b);

        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(b_address, std::ops::Deref::deref(&r) as *const i64);
    }

    #[test]
    fn test_count_items_in_collection() {
        assert_eq!(
            count_items_in_collection(Option::<i32>::None.into_iter()),
            0usize
        );
        assert_eq!(count_items_in_collection(Some(1).into_iter()), 1usize);
        assert_eq!(
            count_items_in_collection(vec![1, 2, 3, 4, 5].into_iter()),
            5usize
        );
    }

    #[test]
    fn test_change_true_to_false() {
        let (c, s) = change_true_to_false(Vec::<bool>::new().into_iter());
        assert_eq!(c.len(), 0);
        assert_eq!(s, 0usize);

        let (c, s) = change_true_to_false(vec![false, false, false].into_iter());
        assert_eq!(c, vec![false, false, false]);
        assert_eq!(s, 0usize);

        let (c, s) = change_true_to_false(vec![true, false, false, false, true, true].into_iter());
        assert_eq!(c, vec![false, false, false, false, false, false]);
        assert_eq!(s, 3usize);
    }

    #[test]
    fn test_composite() {
        let r = composite(vec![1, 8, 4, 2, 5, 7, 8, 16, 18, 32, 2, 1, 64].into_iter());
        assert_eq!(r, vec![2, 8, 16, 18])
    }

    #[test]
    fn test_composite2() {
        let r = composite2(vec![1, 8, 4, 2, 5, 7, 8, 16, 18, 32, 2, 1, 64].into_iter());
        assert_eq!(r, vec![2, 8, 16, 18, 32])
    }

    #[test]
    fn test_words() {
        assert_eq!(words(vec![].into_iter()), String::from(""));
        assert_eq!(
            words(vec![String::from("one")].into_iter()),
            String::from("one;")
        );
        assert_eq!(
            words(vec![String::from("one"), String::from("two")].into_iter()),
            String::from("one;two;")
        );
    }
}

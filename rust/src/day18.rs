use crate::utils::Part;

#[derive(Debug)]
enum SnailFishItem {
    Num(u8),
    Pair(Box<SnailFishItem>, Box<SnailFishItem>),
}

impl SnailFishItem {
    fn build_pair(left: SnailFishItem, right: SnailFishItem) -> SnailFishItem {
        SnailFishItem::Pair(Box::new(left), Box::new(right))
    }

    fn clone(&self) -> SnailFishItem {
        match &self {
            Self::Num(x) => SnailFishItem::Num(*x),
            SnailFishItem::Pair(left, right) => {
                SnailFishItem::build_pair(left.as_ref().clone(), right.as_ref().clone())
            }
        }
    }
}

impl std::fmt::Display for SnailFishItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            &Self::Num(x) => write!(f, "{}", x),
            &Self::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

enum ParsingStackItem {
    Num(u8),
    Pair(SnailFishItem),
    Unknown,
}

impl ParsingStackItem {
    fn to_snail_fish_pair_item(self) -> Result<SnailFishItem, ParsingError> {
        match self {
            Self::Num(x) => Ok(SnailFishItem::Num(x)),
            Self::Pair(pair) => Ok(pair),
            Self::Unknown => Err(ParsingError::BadConvertionToPairItem),
        }
    }

    fn to_snail_fish_pair(self) -> Result<SnailFishItem, ParsingError> {
        match self {
            Self::Pair(pair) => Ok(pair),
            _ => Err(ParsingError::NotValidPair),
        }
    }

    fn increment(&self, by: u8) -> Result<ParsingStackItem, ParsingError> {
        if let Self::Num(x) = self {
            return Ok(ParsingStackItem::Num(x * 10 + by));
        } else if let Self::Unknown = self {
            return Ok(ParsingStackItem::Num(by));
        }
        return Err(ParsingError::NumberInsidePair);
    }
}

#[derive(Debug)]
enum ParsingError {
    BadNumber,
    BadConvertionToPairItem,
    EmptyStackForNumber,
    EmptyStackForPairFirst,
    EmptyStackForPairSecond,
    EmptyStackForEndingPair,
    NumberInsidePair,
    BadBalancedPairs,
    NotValidPair,
}

fn parse_line(line: &str) -> Result<SnailFishItem, ParsingError> {
    let mut stack: Vec<ParsingStackItem> = vec![];
    for car in line.chars() {
        match car {
            '[' => match stack.last() {
                Some(&ParsingStackItem::Unknown) => {}
                _ => stack.push(ParsingStackItem::Unknown),
            },
            ',' => stack.push(ParsingStackItem::Unknown),
            ']' => {
                let second: SnailFishItem = stack
                    .pop()
                    .ok_or(ParsingError::EmptyStackForPairSecond)
                    .and_then(|item| item.to_snail_fish_pair_item())?;
                let first: SnailFishItem = stack
                    .pop()
                    .ok_or(ParsingError::EmptyStackForPairFirst)
                    .and_then(|item| item.to_snail_fish_pair_item())?;
                stack.push(ParsingStackItem::Pair(SnailFishItem::build_pair(
                    first, second,
                )))
            }
            c => {
                let curr_item = stack.pop().ok_or(ParsingError::EmptyStackForNumber)?;
                let num = c.to_digit(10).ok_or(ParsingError::BadNumber)? as u8;
                stack.push(curr_item.increment(num)?)
            }
        }
    }
    if stack.len() > 1 {
        return Err(ParsingError::BadBalancedPairs);
    }

    return stack
        .pop()
        .ok_or(ParsingError::EmptyStackForEndingPair)
        .and_then(|item| item.to_snail_fish_pair());
}

fn parse(lines: &Vec<String>) -> Result<Vec<SnailFishItem>, ParsingError> {
    return lines.iter().map(|string| parse_line(&string)).collect();
}

#[derive(PartialEq, Eq, Debug)]
enum ExplodeAction {
    PendingLeft(u8),
    PendingRight(u8),
}

#[derive(PartialEq, Eq, Debug)]
enum ExplodeResult {
    Explode(u8, u8),
    Exploded(Option<ExplodeAction>, Option<ExplodeAction>),
}

fn manage_explode_apply_action_pair(
    left: &mut SnailFishItem,
    right: &mut SnailFishItem,
    parent_action: ExplodeAction,
) -> Option<ExplodeAction> {
    return match parent_action {
        ExplodeAction::PendingLeft(x) => {
            manage_explode_apply_action(right, ExplodeAction::PendingLeft(x))
                .and_then(|action| manage_explode_apply_action(left, action))
        }
        ExplodeAction::PendingRight(x) => {
            manage_explode_apply_action(left, ExplodeAction::PendingRight(x))
                .and_then(|action| manage_explode_apply_action(right, action))
        }
    };
}

fn manage_explode_apply_action(
    item: &mut SnailFishItem,
    action: ExplodeAction,
) -> Option<ExplodeAction> {
    return match item {
        SnailFishItem::Num(val) => {
            match action {
                ExplodeAction::PendingLeft(x) | ExplodeAction::PendingRight(x) => (*val) += x,
            }
            return None;
        }
        SnailFishItem::Pair(left, right) => {
            manage_explode_apply_action_pair(left.as_mut(), right.as_mut(), action)
        }
    };
}

fn explode_right_item(
    left: &mut SnailFishItem,
    right: &mut SnailFishItem,
    depth: u8,
) -> Option<ExplodeResult> {
    let result = explode(right, depth)?;
    let explode_result = match result {
        ExplodeResult::Exploded(a, b) => (a, b),
        ExplodeResult::Explode(x, y) => {
            *right = SnailFishItem::Num(0);
            (
                Some(ExplodeAction::PendingLeft(x)),
                Some(ExplodeAction::PendingRight(y)),
            )
        }
    };
    return Some(ExplodeResult::Exploded(
        explode_result
            .0
            .and_then(|action| manage_explode_apply_action(left, action)),
        explode_result.1,
    ));
}

fn explode_left_item(
    left: &mut SnailFishItem,
    right: &mut SnailFishItem,
    depth: u8,
) -> Option<ExplodeResult> {
    let result = explode(left, depth)?;
    let explode_result = match result {
        ExplodeResult::Exploded(a, b) => (a, b),
        ExplodeResult::Explode(x, y) => {
            *left = SnailFishItem::Num(0);
            (
                Some(ExplodeAction::PendingLeft(x)),
                Some(ExplodeAction::PendingRight(y)),
            )
        }
    };
    return Some(ExplodeResult::Exploded(
        explode_result.0,
        explode_result
            .1
            .and_then(|action| manage_explode_apply_action(right, action)),
    ));
}

fn explode(item: &mut SnailFishItem, depth: u8) -> Option<ExplodeResult> {
    return match item {
        SnailFishItem::Num(_) => None,
        SnailFishItem::Pair(left, right) => {
            if let (SnailFishItem::Num(x), SnailFishItem::Num(y)) = (left.as_ref(), right.as_ref())
            {
                if depth >= 4 {
                    return Some(ExplodeResult::Explode(*x, *y));
                } else {
                    return None;
                }
            } else {
                return explode_left_item(left.as_mut(), right.as_mut(), depth + 1)
                    .or_else(|| explode_right_item(left.as_mut(), right.as_mut(), depth + 1));
            }
        }
    };
}

#[derive(PartialEq, Eq)]
enum SplitAction {
    Split(u8),
    Splitted,
}

fn split_number(val: u8) -> SnailFishItem {
    return SnailFishItem::build_pair(
        SnailFishItem::Num(val >> 1),
        SnailFishItem::Num((val >> 1) + (val % 2)),
    );
}

fn split_apply(item: &mut SnailFishItem, action: SplitAction) -> Option<SplitAction> {
    match action {
        SplitAction::Splitted => Some(SplitAction::Splitted),
        SplitAction::Split(val) => {
            *item = split_number(val);
            Some(SplitAction::Splitted)
        }
    }
}

fn split(item: &mut SnailFishItem) -> Option<SplitAction> {
    return match item {
        SnailFishItem::Num(x) if *x >= 10 => Some(SplitAction::Split(*x)),
        SnailFishItem::Pair(left, right) => split(left)
            .and_then(|action| split_apply(left, action))
            .or_else(|| split(right))
            .and_then(|action| split_apply(right, action)),
        _ => None,
    };
}

fn reduce(item: &mut SnailFishItem, print_intermediates: bool) {
    loop {
        if let None = explode(item, 0) {
            if let None = split(item) {
                break;
            }
        }
        if print_intermediates {
            let intermediate_result = item.to_string();
            println!("Intermediate Result {}", intermediate_result)
        }
    }
}

fn magnitude(item: &SnailFishItem) -> u32 {
    match &item {
        SnailFishItem::Num(x) => *x as u32,
        SnailFishItem::Pair(left, right) => {
            3 * magnitude(left.as_ref()) + 2 * magnitude(right.as_ref())
        }
    }
}

fn sum(left: SnailFishItem, right: SnailFishItem, print_intermediates: bool) -> SnailFishItem {
    let mut new_pair = SnailFishItem::build_pair(left, right);
    reduce(&mut new_pair, print_intermediates);
    return new_pair;
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let snailfish_pairs = parse(lines).unwrap();
    match part {
        Part::Part1 => {
            let item = snailfish_pairs
                .into_iter()
                .reduce(|src, dest| sum(src, dest, false))
                .unwrap();

            let mag = magnitude(&item);
            println!("Result Version {}", mag)
        }
        Part::Part2 => {
            let snailfish_pairs_ref = &snailfish_pairs;
            let combinations: Vec<u32> = snailfish_pairs_ref
                .iter()
                .flat_map(|i1| {
                    snailfish_pairs_ref.iter().map(move |i2| {
                        if std::ptr::eq(i1, i2) {
                            0
                        } else {
                            magnitude(&sum(i1.clone(), i2.clone(), false))
                        }
                    })
                })
                .collect();
            let max = combinations.iter().fold(0u32,|curr,val| std::cmp::max(curr,*val));
            println!("Result Eval {}", max);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_explode_simple_left() {
        let mut value = parse_line("[[[[[9,8],1],2],3],4]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(
            res,
            Some(ExplodeResult::Exploded(
                Some(ExplodeAction::PendingLeft(9)),
                None
            ))
        );
        assert_eq!(value.to_string(), "[[[[0,9],2],3],4]")
    }

    #[test]
    fn test_explode_simple_right() {
        let mut value = parse_line("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(
            res,
            Some(ExplodeResult::Exploded(
                None,
                Some(ExplodeAction::PendingRight(2))
            ))
        );
        assert_eq!(value.to_string(), "[7,[6,[5,[7,0]]]]")
    }

    #[test]
    fn test_explode_simple_middle() {
        let mut value = parse_line("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(res, Some(ExplodeResult::Exploded(None, None)));
        assert_eq!(value.to_string(), "[[6,[5,[7,0]]],3]")
    }
    #[test]
    fn test_explode_priority_left() {
        let mut value = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let res = explode(&mut value, 0);
        assert_eq!(res, Some(ExplodeResult::Exploded(None, None)));
        assert_eq!(value.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        //Second time
        let res2 = explode(&mut value, 0);
        assert_eq!(
            res2,
            Some(ExplodeResult::Exploded(
                None,
                Some(ExplodeAction::PendingRight(2))
            ))
        );
        assert_eq!(value.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_sum() {
        let sum_res = sum(
            parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap(),
            parse_line("[1,1]").unwrap(),
            false,
        );
        assert_eq!(sum_res.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_complex() {
        let complex_test: Vec<&str> = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let items_res: Result<Vec<SnailFishItem>, ParsingError> =
            complex_test.iter().map(|line| parse_line(*line)).collect();
        let items = items_res.unwrap();
        let mut intermediates: Vec<String> = Vec::with_capacity(items.len());
        let final_result = items
            .into_iter()
            .reduce(|src, dest| {
                let intermediate = sum(src, dest, false);
                intermediates.push(intermediate.to_string());
                intermediate
            })
            .unwrap();
        assert_eq!(
            intermediates,
            vec![
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ]
        );

        assert_eq!(
            final_result.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude_calc() {
        let item =
            parse_line("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap();
        let result = magnitude(&item);

        assert_eq!(result, 4140);
    }

    impl SnailFishItem {
        fn to_string(&self) -> String {
            format!("{}", self)
        }
    }
}

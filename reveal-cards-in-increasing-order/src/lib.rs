pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    let n = deck.len();
    let mut result = vec![0; n];

    let mut skip = false;
    let mut index_in_deck = 0;
    let mut index_in_result = 0;

    deck.sort_unstable();

    while index_in_deck < n {
        if result[index_in_result] == 0 {
            if !skip {
                result[index_in_result] = deck[index_in_deck];
                index_in_deck += 1;
            }
            skip = !skip;
        }

        index_in_result = (index_in_result + 1) % n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    2 -> 3 -> 5 -> 7
    always even positions are sorted ascen
    Example 1:

    // sorted
    2, 3, 5, 7, 11, 13, 17
    2, 13, 3, 11, 5,   17,  7

    Input:  [17,13,11,2,3,5,7]
    Output: [2,13,3,11,5,17,7]

    2
    13, 3
    11, 5
    17, 7

    Input: deck = [17,13,11,2,3,5,7]
    Output: [2,13,3,11,5,17,7]
    Explanation:
    We get the deck in the order [17,13,11,2,3,5,7] (this order does not matter), and reorder it.
    After reordering, the deck starts as [2,13,3,11,5,17,7], where 2 is the top of the deck.
    We reveal 2, and move 13 to the bottom.  The deck is now [3,11,5,17,7,13].
    We reveal 3, and move 11 to the bottom.  The deck is now [5,17,7,13,11].
    We reveal 5, and move 17 to the bottom.  The deck is now [7,13,11,17].
    We reveal 7, and move 13 to the bottom.  The deck is now [11,17,13].
    We reveal 11, and move 17 to the bottom.  The deck is now [13,17].
    We reveal 13, and move 17 to the bottom.  The deck is now [17].
    We reveal 17.
    Since all the cards revealed are in increasing order, the answer is correct.
    */

    #[test]
    fn it_works1() {
        let result = deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]);
        assert_eq!(result, vec![2, 13, 3, 11, 5, 17, 7]);
    }

    #[test]
    fn it_works2() {
        let result = deck_revealed_increasing(vec![1, 1000]);
        assert_eq!(result, vec![1, 1000]);
    }

    #[test]
    fn it_works3() {
        let result = deck_revealed_increasing(vec![1]);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn it_works4() {
        // 1, 2, 3, 4, 5
        // 1
        // 4, 2
        // 5, 3
        let result = deck_revealed_increasing(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, vec![1, 5, 2, 4, 3]);
    }
}

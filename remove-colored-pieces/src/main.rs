fn main() {
    // Example 1:
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // Input: colors = "AAABABB"
    // Output: true
    // Explanation:
    // AAABABB -> AABABB
    // Alice moves first.
    // She removes the second 'A' from the left since that is the only 'A' whose neighbors are both 'A'.
    //
    // Now it's Bob's turn.
    // Bob cannot make a move on his turn since there are no 'B's whose neighbors are both 'B'.
    // Thus, Alice wins, so return true.
    // Example 2:
    //
    // Input: colors = "AA"
    // Output: false
    // Explanation:
    // Alice has her turn first.
    // There are only two 'A's and both are on the edge of the line, so she cannot move on her turn.
    // Thus, Bob wins, so return false.
    // Example 3:
    //
    // Input: colors = "ABBBBBBBAAA"
    // Output: false
    // Explanation:
    // ABBBBBBBAAA -> ABBBBBBBAA
    // Alice moves first.
    // Her only option is to remove the second to last 'A' from the right.
    //
    // ABBBBBBBAA -> ABBBBBBAA
    // Next is Bob's turn.
    // He has many options for which 'B' piece to remove. He can pick any.
    //
    // On Alice's second turn, she has no more pieces that she can remove.
    // Thus, Bob wins, so return false.
}

pub fn winner_of_game(colors: String) -> bool {
    let mut acount = 0;
    let mut alice_count = 0;
    let mut bcount = 0;
    let mut bob_count = 0;

    for c in colors.chars() {
        if c == 'A' {
            bcount = 0;
            acount += 1;
            if acount >= 3 {
                alice_count += 1;
            }
        } else {
            acount = 0;
            bcount += 1;
            if bcount >= 3 {
                bob_count += 1;
            }
        }
    }

    if bob_count >= alice_count {
        false
    } else {
        true
    }
}

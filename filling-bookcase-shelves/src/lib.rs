pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut memory = vec![-1; books.len()];
    _min_height_shelves(&books, shelf_width, 0, &mut memory)
}

pub fn _min_height_shelves(
    books: &Vec<Vec<i32>>,
    shelf_width: i32,
    idx: usize,
    memory: &mut Vec<i32>,
) -> i32 {
    if idx == books.len() {
        return 0;
    }

    if memory[idx] != -1 {
        return memory[idx];
    }

    let mut result = i32::MAX;
    let mut cur_block_width = 0;
    let mut cur_block_height = 0;

    for i in idx..books.len() {
        let width = books[i][0];
        let height = books[i][1];

        if width + cur_block_width <= shelf_width {
            cur_block_width = cur_block_width + width;
            cur_block_height = cur_block_height.max(height);
        } else {
            // this block is invalid
            break;
        }

        let total_block = cur_block_height + _min_height_shelves(books, shelf_width, i + 1, memory);
        result = result.min(total_block);
    }

    memory[idx] = result;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let books = vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ];
        let shelf_width = 4;
        let output = 6;
        let result = min_height_shelves(books, shelf_width);
        assert_eq!(result, output);
    }
}

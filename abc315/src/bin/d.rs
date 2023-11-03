use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        strings: [String; h]
    };

    let colors = strings
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // 各行について、各色が残り何個か
    let mut row_color_number = vec![vec![0 as i32; 26]; h];
    // 各列について、各色が残り何個か
    let mut col_color_number = vec![vec![0 as i32; 26]; w];
    // 各行について、削除されたかどうか
    let mut row_alive = vec![true; h];
    // 各列について、削除されたかどうか
    let mut col_alive = vec![true; w];

    // 各行/各列について、各色が何個かを計算する
    for i in 0..h {
        for j in 0..w {
            let c = colors[i][j];
            let c_index = (c as u8 - 'a' as u8) as usize;
            row_color_number[i][c_index] += 1;
            col_color_number[j][c_index] += 1;
        }
    }

    // 'a'から'z'までループを回す
    loop {
        let len_row = row_alive.iter().filter(|&&b| b).count();
        let len_col = col_alive.iter().filter(|&&b| b).count();
        let mut checked_row: HashMap<usize, usize> = HashMap::new();
        let mut checked_col: HashMap<usize, usize> = HashMap::new();

        if len_row <= 1 || len_col <= 1 {
            break;
        }

        // 各行について、その色の残り個数がcolorsの列数と同じであればcheckする
        for i in 0..h {
            for ci in 0..26 {
                if row_alive[i] && len_col >= 2 && row_color_number[i][ci] == len_col as i32 {
                    checked_row.insert(i, ci as usize);
                }
            }
        }

        // 各列について、その色の残り個数がcolorsの行数と同じであればcheckする
        for i in 0..w {
            for ci in 0..26 {
                if col_alive[i] && len_row >= 2 && col_color_number[i][ci] == len_row as i32 {
                    checked_col.insert(i, ci as usize);
                }
            }
        }

        // checkした行/列がなければ終了
        if checked_row.is_empty() && checked_col.is_empty() {
            break;
        }

        // checkした行について操作を行う
        for (i, ci) in checked_row {
            row_alive[i] = false;
            col_color_number.iter_mut().for_each(|color_number| {
                color_number[ci] -= 1;
            });
        }

        // checkした列について操作を行う
        for (i, ci) in checked_col {
            col_alive[i] = false;
            row_color_number.iter_mut().for_each(|color_number| {
                color_number[ci] -= 1;
            });
        }
    }

    let len_row = row_alive.iter().filter(|&&b| b).count();
    let len_col = col_alive.iter().filter(|&&b| b).count();
    println!("{}", len_row * len_col);
}

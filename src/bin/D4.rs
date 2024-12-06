use core::str;

const XMAS: &[u8] = "XMAS".as_bytes();
const XMAS_REV: &[u8] = "SAMX".as_bytes();
const WLEN: usize = XMAS.len();

const MAS: &[u8] = "MAS".as_bytes();
const MAS_REV: &[u8] = "SAM".as_bytes();
const MASLEN: usize = MAS.len();

fn get(i: usize, j: usize, lines: &Vec<&[u8]>) -> u8 {
    if i >= lines.len() || j >= lines[0].len() {
        0
    } else {
        lines[i][j]
    }
}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D4.txt").expect("Error Reading File!");

    part1(&fstr);

    part2(&fstr);

}

fn part1(fstr: &String) {

    let lines: Vec<&[u8]> = fstr.lines().map(|s| s.as_bytes()).collect();

    let mut found_cnt: usize = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {

            let mut row_wrd: [u8; WLEN] = [42; 4];
            let mut col_wrd: [u8; WLEN] = [42; 4];
            let mut diag_wrd1: [u8; WLEN] = [42; 4];
            let mut diag_wrd2: [u8; WLEN] = [42; 4];

            for k in 0..WLEN {
                row_wrd[k]  = get(i,j+k, &lines);
                col_wrd[k]  = get(i+k, j, &lines);
                diag_wrd1[k] = get(i+k, j+k, &lines);
                diag_wrd2[k] = get(i+k, j+WLEN-1-k, &lines);
            }

            if row_wrd.eq(XMAS) || row_wrd.eq(XMAS_REV) {
                found_cnt += 1;
            }

            if col_wrd.eq(XMAS) || col_wrd.eq(XMAS_REV) {
                found_cnt += 1;
            }

            if diag_wrd1.eq(XMAS) || diag_wrd1.eq(XMAS_REV) {
                found_cnt += 1;
            }

            if diag_wrd2.eq(XMAS) || diag_wrd2.eq(XMAS_REV) {
                found_cnt += 1;
            }
        }
    }

    println!("Found {} instances of {}/{}!", 
        found_cnt,
        str::from_utf8(XMAS).unwrap(), str::from_utf8(XMAS_REV).unwrap());

}

fn part2(fstr: &String) {

    let lines: Vec<&[u8]> = fstr.lines().map(|s| s.as_bytes()).collect();

    let mut found_cnt: usize = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {

            let mut diag_wrd1: [u8; MASLEN] = [42; MASLEN];
            let mut diag_wrd2: [u8; MASLEN] = [42; MASLEN];

            for k in 0..MASLEN {
                diag_wrd1[k] = get(i+k, j+k, &lines);
                diag_wrd2[k] = get(i+k, j+MASLEN-1-k, &lines);
            }

            let mut cnt = 0;

            if diag_wrd1.eq(MAS) || diag_wrd1.eq(MAS_REV) {
                cnt += 1;
            }

            if diag_wrd2.eq(MAS) || diag_wrd2.eq(MAS_REV) {
                cnt += 1;
            }

            if cnt==2 {
                found_cnt += 1;
            }
        }
    }

    println!("Found {} instances of X-MAS", 
        found_cnt);

}

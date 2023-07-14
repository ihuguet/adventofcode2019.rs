const PASSWORD_MIN: [u8; 6] = [2, 4, 8, 3, 4, 5];
const PASSWORD_MAX: [u8; 6] = [7, 4, 6, 3, 1, 5];

fn main() {
    let mut valid_pwds_part1 = 0;
    let mut valid_pwds_part2 = 0;

    let mut pwd = PASSWORD_MIN;

    loop {
        if is_valid_pwd_part1(&pwd) {
            valid_pwds_part1 += 1;
        }
        if is_valid_pwd_part2(&pwd) {
            valid_pwds_part2 += 1;
        }

        pwd = next_pwd(pwd);

        if pwd > PASSWORD_MAX {
            break;
        }
    }

    println!("Part 1: valid passwords = {}", valid_pwds_part1);
    println!("Part 2: valid passwords = {}", valid_pwds_part2);
}

fn is_valid_pwd_part1(pwd: &[u8; 6]) -> bool {
    for vals in pwd.windows(2) {
        if vals[0] > vals[1] {
            return false;
        }
    }

    for vals in pwd.windows(2) {
        if vals[0] == vals[1] {
            return true
        }
    }

    false
}

fn is_valid_pwd_part2(pwd: &[u8; 6]) -> bool {
    for vals in pwd.windows(2) {
        if vals[0] > vals[1] {
            return false;
        }
    }

    let mut i = 0;
    while i < 5 {
        let next = pwd[i..].iter()
            .position(|&digit| digit != pwd[i])
            .unwrap_or(6 - i);

        if next == 2 {
            return true;
        } else {
            i += next;
        }
    }

    false
}

fn next_pwd(mut pwd: [u8; 6]) -> [u8; 6] {
    // increase pwd by 1
    for digit in pwd.iter_mut().rev() {
        *digit += 1;
        if *digit > 9 {
            *digit = 0;
        } else {
            break;
        }
    }

    // jump values with decreasing digits (i.e. 1431 -> 1444)
    for i in 0..5 {
        if pwd[i] > pwd[i + 1] {
            pwd[i + 1] = pwd[i];
        }
    }

    pwd
}

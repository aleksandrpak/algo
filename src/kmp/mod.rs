#[cfg(test)] mod test;

pub fn build_table<T: PartialEq>(pattern: &[T]) -> Box<[isize]> {
    let m = pattern.len();
    let mut table = Vec::with_capacity(m);
    let mut k: isize;

    table.push(-1);
    for i in 1..m {
        k = table[i - 1];
        while k >= 0 {
            if pattern[k as usize] == pattern[i - 1] {
                break;
            }

            k = table[k as usize];
        }

        table.push(k + 1);
    }

    table.into_boxed_slice()
}

pub fn find_first<T: PartialEq>(target: &[T], pattern: &[T]) -> Option<usize> {
    let table = build_table(pattern);
    find_first_with_table(target, pattern, &table)
}

pub fn find_first_with_table<T: PartialEq>(target: &[T], pattern: &[T], table: &[isize]) -> Option<usize> {
    let n = target.len();
    let m = pattern.len() as isize;

    let mut i = 0;
    let mut k: isize = 0;

    while i < n {
        if k == -1 {
            i += 1;
            k = 0;
        } else if target[i] == pattern[k as usize] {
            i += 1;
            k += 1;

            if k == m {
                return Some(i - m as usize);
            }
        } else {
            k = table[k as usize];
        }
    }

    None
}

pub fn find_all<T: PartialEq>(target: &[T], pattern: &[T]) -> Vec<usize> {
    let table = build_table(pattern);
    find_all_with_table(target, pattern, &table)
}

pub fn find_all_with_table<T: PartialEq>(target: &[T], pattern: &[T], table: &[isize]) -> Vec<usize> {
    let mut results = vec![];
    let mut index;

    match find_first_with_table(target, pattern, table) {
        Some(result) => {
            results.push(result);
            index = result;
        },
        None => return results
    }

    let n = target.len();
    let m = pattern.len();

    while index + m < n {
        let next = index + 1;
        match find_first_with_table(&target[next..], pattern, table) {
            Some(result) => {
                results.push(next + result);
                index = next + result;
            },
            None => break
        }
    }

    results
}

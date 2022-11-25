fn merge(vec: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let mut t = Vec::new();
    for i in start..=end {
        t.push(vec[i]);
    }
    let mut i: usize = 0;
    let new_mid = mid - start;
    let mut j = new_mid + 1;
    let mut k = start;
    while i <= new_mid && j < t.len() {
        if t[i] <= t[j] {
            vec[k] = t[i];
            k += 1;
            i += 1;
        } else {
            vec[k] = t[j];
            k += 1;
            j += 1;
        }
    }

    while i <= new_mid {
        vec[k] = t[i];
        k += 1;
        i += 1;
    }

    while j < t.len() {
        vec[k] = t[j];
        k += 1;
        j += 1;
    }
}

fn merge_sort(vec: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mid = start + (end - start) / 2;
    merge_sort(vec, start, mid);
    merge_sort(vec, mid + 1, end);
    merge(vec, start, mid, end);
}
#[derive(Debug)]
struct MergeParam {
    start: usize,
    end: usize,
}

fn merge_sort_nonrecursive(vec: &mut Vec<i32>, start: usize, end: usize) {
    let mut param_stack: Vec<MergeParam> = vec![];
    let mut exec_stack: Vec<MergeParam> = vec![];
    param_stack.push(MergeParam { start, end });
    exec_stack.push(MergeParam { start, end });
    while param_stack.len() > 0 {
        let merge_param = param_stack.pop().unwrap();
        let start = merge_param.start;
        let end = merge_param.end;
        let mid = start + (end - start) / 2;

        if start < mid {
            param_stack.push(MergeParam { start, end: mid });
            exec_stack.push(MergeParam { start, end: mid });
            // merge(vec, start, mid, end);
        }

        if mid < end {
            param_stack.push(MergeParam {
                start: mid + 1,
                end,
            });
            exec_stack.push(MergeParam {
                start: mid + 1,
                end,
            });
        }
    }
    while let Some(param) = exec_stack.pop() {
        let start = param.start;
        let end = param.end;
        let mid = start + (end - start) / 2;
        merge(vec, param.start, mid, end)
    }
    println!("{:?}  ", exec_stack);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let mut v = vec![8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
        let len = v.len();
        merge_sort(&mut v, 0, len - 1);

        // for n in v.iter() {
        //     print!("{},", n);
        // }
        println!("{:?}  ", v);
        assert_eq!(v, vec![-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100])
    }
    #[test]
    fn test_merge_sort_nonrecursive() {
        let mut v = vec![8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
        let len = v.len();
        merge_sort_nonrecursive(&mut v, 0, len - 1);

        println!("{:?}  ", v);
        assert_eq!(v, vec![-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100])
    }
}

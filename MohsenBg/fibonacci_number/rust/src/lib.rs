pub fn multiply_matrix(a: Vec<Vec<i128>>, b: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let mut matrix_multiply = vec![vec![0, 0], vec![0, 0]];

    matrix_multiply[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
    matrix_multiply[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
    matrix_multiply[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
    matrix_multiply[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];

    matrix_multiply
}

pub fn power_matrix(mut matrix: Vec<Vec<i128>>, mut power: i32) -> Vec<Vec<i128>> {
    let mut result = vec![vec![1, 0], vec![0, 1]]; 
    
    while power > 0 {
        if power & 1 == 1 { 
            result = multiply_matrix(result.clone(), matrix.clone());
        }
        matrix = multiply_matrix(matrix.clone(), matrix.clone());
        power /= 2;
    }

    result
}

pub fn fibonacci(n: i32) -> i128 {
    if n == 0 {
        return 0;
    }
    let base = vec![vec![1, 1], vec![1, 0]]; 
    power_matrix(base, n - 1)[0][0] 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_0() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fib_10() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fib_20() {
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_fib_93() {
        assert_eq!(fibonacci(93), 12200160415121876738);
    }
}


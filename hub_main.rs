mod helper;
use helper::*;

fn main() {
    let edges = read_file("cal.cedge");

    let n = 21048;
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0;n];n];

    for (u, v) in edges {
        matrix[u][v] = 1.0;
        matrix[v][u] = 1.0;
    }
    
    let mut vector = vec![0.0;n];

    for i in 0..n {
        for j in 0..n {
            vector[j] += matrix[i][j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] /= vector[j];
        }
    }

    let mut r = vec![1.0/n as f64;n];

    for i in 0..100 {
        println!("{}", i);
        r = matmul(&matrix, &r);
    }

    // for i in 0..n {
    //     if r[i] > 0.00005 {
    //         println!("{:?}", i);
    //     }
    // }
    println!("{:?}", r);
}

fn matmul(matrix: &Vec<Vec<f64>>, r: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let mut ret = vec![0.0;n];
    for i in 0..n {
        for j in 0..n {
            ret[i] += matrix[i][j]*r[j];
        }
    }
    ret
}

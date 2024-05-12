use std::ops::Mul;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tensor {
    pub data: Vec<Vec<f64>>,
    pub m: usize,
    pub n: usize,
}

impl Tensor {
    pub fn of(data: &Vec<Vec<f64>>) -> Tensor {
        Tensor { 
            data: data.to_vec(),
            m: data.len(),
            n: data[0].len()
        }
    }

    pub fn singleton(value: f64) -> Tensor {
        Tensor {
            data: vec![vec![value]],
            m: 1,
            n: 1
        }
    }

    pub fn fill(m: usize, n: usize, c: f64) -> Tensor {
        let data = vec![vec![c; n]; m];
        Tensor { data, m, n }
    }

    pub fn zeros(m: usize, n: usize) -> Tensor {
        Tensor::fill(m, n, 0.0)
    }
    pub fn ones(m: usize, n: usize) -> Tensor {
        Tensor::fill(m, n, 1.0)
    }

    pub fn concat(&self, x: &Tensor) -> Tensor {
        if self.n != x.n || x.m != 1 {
            panic!("Expected {} got {}", self.n, x.n);
        }
        let mut result = self.clone(); // TODO: is data the same pointer?
        result.data.push(x.data[0].clone());
        result.m += 1;
        result
    }
}

impl Mul<Tensor> for Tensor {
    type Output = Tensor;
    fn mul(self, right: Tensor) -> Tensor {
        let (n, m) = (self.m, self.n);
        let (m_2, p) = (right.m, right.n);
 
        // [n x m][m x p] => [n x p]
        if m != m_2 {
            panic!("Incompatible dimensions")
        }
        let mut data = vec![vec![0.0; p]; n];
        //println!("{}, {}, {}, {:?}", n, m, p, data);
        for i in 0..n {
            for j in 0..p {
                for k in 0..m {
                    println!("{}, {}, {}, {:?}", i, j, k, data);
                    data[i][j] += self.data[i][k] * right.data[k][j];
                }
            }
        }
        println!("{:?}", data);
        Tensor { data, m: n, n: p }
    }
}


// TODO: impl
impl Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
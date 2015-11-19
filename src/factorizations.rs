use super::{Matrix, SVD};
use std::cmp::{min,max};
use lapack::*;
use matrixerror::MatrixError;
use scalars::*;

pub fn lu(a : &mut Matrix<f64>) -> Result<(&mut Matrix<f64>, Vec<i32>), MatrixError>{
    let m = a.row_size;
    let n = a.col_size;
    let mut ipiv = vec![0; min(m,n)];
    let mut info = 0;
    dgetrf(m, n, &mut a.elements, m, &mut ipiv, &mut info);
    match info {
        1 => Err(MatrixError::LapackComputationError),
        0 => Ok((a, ipiv)),
        -1 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }


}


pub fn qr(a : &mut Matrix<f64>) ->Result<Matrix<f64>,MatrixError>{
    let m = a.row_size;
    let n = a.col_size;
    let mut tau = vec![0.0; min(m,n)];
    let mut work = vec![0.0; 4*n];
    let lwork = 4*n as isize;
    let mut info = 0;
    dgeqrf(m, n, &mut a.elements, m, &mut tau,
    &mut work, lwork, &mut info);
    match info {
        1 => Err(MatrixError::LapackComputationError),
        0 => Ok(Matrix {
            elements : a.elements.to_owned(),
            row_size : m,
            col_size : n,
            transpose : false
        }),
        -1 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }
}


pub fn svd(a : &mut Matrix<f64>) -> Result <SVD, MatrixError> {
    let m = a.row_size;
    let n = a.col_size;

    let s = singular_values(a);
    let ldu = m;
    let mut u = vec![0.0; ldu*min(m,n)];

    let ldvt = n;
    let mut vt = vec![0.0;ldvt*n];

    let lwork = max(max(1,3*min(m,n)+min(m,n)),5*min(m,n)) +1 ;
    let mut work = vec![0.0; lwork];

    let mut info = 0;
    let mut s_elem = s.unwrap().elements;
    dgesvd(b'A', b'A',m,n,&mut a.elements,m,&mut s_elem, &mut u,ldu, &mut vt, ldvt, &mut work, lwork as isize, &mut info);

    match info {
        1 => Err(MatrixError::LapackComputationError),
        0 => Ok((
            Matrix {
                elements : u,
                row_size : n,
                col_size : ldu,
                transpose : false,
            },
            Matrix :: diag_mat(s_elem)
            ,
            Matrix {
                elements : vt,
                row_size : n,
                col_size : ldvt,
                transpose : true,
            }
        )

),
        -1 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }


}
use super::{Matrix, Eig, SVD, Triangular, Trans};
use std::cmp::{min,max};
use lapack::*;
use matrixerror::MatrixError;
use eigenvalues::*;
use operations::*;

/// Compute the LU factorization.
pub fn lufact(a : &mut Matrix<f64>) -> Result<(&mut Matrix<f64>, Vec<i32>), MatrixError>{
    let m = a.row_size;
    let n = a.col_size;
    let mut ipiv = vec![0; min(m,n)];
    let mut info = 0;
    dgetrf(m, n, &mut a.elements, m, &mut ipiv, &mut info);
    match info {
        x if x > 0 => Err(MatrixError::LapackComputationError),
        0 => Ok((a, ipiv)),
        x if x < 0 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }


}


/// Compute the QR Factorization.
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
        x if x > 0 => Err(MatrixError::LapackComputationError),
        0 => Ok(Matrix {
            elements : a.elements.to_owned(),
            row_size : m,
            col_size : n,
            transpose : Trans :: Regular,
        }),
        x if x < 0 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }
}

/// Compute the SVD Factorization.
pub fn svd(a : &mut Matrix<f64>) -> Result <SVD, MatrixError> {
    let m = a.row_size;
    let n = a.col_size;
    let lda = a.row_size;
    let s = singular_values(a);
    let ldu = a.row_size;
    let ldvt = a.col_size;

    let mut u = vec![0.0; ldu*m];
    let mut vt = vec![0.0;ldvt*n];

    let lwork = max(max(1,3*min(m,n)+min(m,n)),5*min(m,n)) +10 ;
    let mut work = vec![0.0; lwork];
    let mut info = 0;

    if let Ok(mut s) = singular_values(a){
        dgesvd(b'A', b'A',m,n,&mut a.elements,lda,&mut s.elements, &mut vt,ldu, &mut u,
        ldvt, &mut work, lwork as isize, &mut info);

        match info {
            x if x > 0 => return Err(MatrixError::LapackComputationError),
            0 => return Ok((
                Matrix {
                    elements : u,
                    row_size : ldu,
                    col_size : min(m,n),
                    transpose : Trans :: Regular,
                },
                Matrix :: diag_mat(s.elements),
                Matrix {
                    elements : vt,
                    row_size : ldvt,
                    col_size : n,
                    transpose : Trans :: Transpose,
                }
            )

    ),
            x if x < 0 => return Err(MatrixError::LapackInputError),
            _ => return Err(MatrixError::UnknownError)
        }

    }
    Err(MatrixError::LapackComputationError)


}


/// Get the singular values of a matrix.
pub fn singular_values(a : &mut Matrix<f64>) -> Result<Matrix<f64>, MatrixError> {
        let mut at =  a.transpose();
        let mut adjoint_operator = try!(dot(&mut at,a));
        let mut e = try!(eigenvalues(&mut adjoint_operator, Eig :: Eigenvalues, Triangular::Upper));
        let s = try!(matrix_map(&|x : &f64| x.sqrt(), &mut e));
        Ok(s)
}

use super::{Matrix};
use lapack::*;
use factorizations::*;
use matrixerror::MatrixError;




pub fn lusolve(lu : (&mut Matrix<f64>, Vec<i32>), b : &mut Matrix<f64>) ->  Result<Matrix<f64>,MatrixError>{
    let (a,ipiv) = lu;
    let lda = a.row_size;
    let n = a.col_size;
    let ldb = b.row_size;
    let nrhs = b.col_size;
    let mut info = 0;
    dgetrs(b'N', n, nrhs, &a.elements, lda, &ipiv, &mut b.elements, ldb , &mut info);

    match info {
        1 => Err(MatrixError::LapackComputationError),
        0 => Ok(Matrix {
            elements : b.elements.to_owned(),
            row_size : ldb,
            col_size : nrhs,
            transpose : false
        }),
        -1 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }


}

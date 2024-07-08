const EPSILON_DIV: f32 = 1e-10;

///
///    Cuda function for resolving the 2x2 system A*X = B
///    by using the analytical formula
///
fn solve_2x2_f32(a: mat2x2f, b: vec2f) -> vec2f {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];

    return vec2f(
        (a[1][1] * b[0] - a[0][1] * b[1]) / det,
        (a[0][0] * b[1] - a[1][0] * b[0]) / det,
    );
}

///
///    inverts the 2x2 M array
///
fn invert_2x2_f32(m: mat2x2f) -> mat2x2f {
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];

    if abs(det) > EPSILON_DIV {
        let det_i = 1.0 / det;

        return mat2x2f(
            m[1][1] * det_i,
            -m[0][1] * det_i,
            -m[1][0] * det_i,
            m[0][0] * det_i,
        );
    } else {
        return mat2x2f(
            1.0, 0.0,
            0.0, 1.0
        );
    }
}

///
///    With X = [X1, X2], performs the quadratique form :
///        X.transpose() @ A @ X
///
fn quad_mat_prod(a: mat2x2f, x1: f32, x2: f32) -> f32 {
    return a[0][0] * x1 * x1 + x1 * x2 * (a[0][1] + a[1][0]) + a[1][1] * x2 * x2;
}

///
///     Returns the two roots of the polynom a*X^2 + b*X + c = 0 for a, b and c
///     real numbers. The function only returns real roots : make sure they exist
///     before calling the function. l[0] contains the root with the biggest module
///     and l[1] the smallest
///
fn get_real_polyroots_2(a: f32, b: f32, c: f32) -> vec2f {
    // numerical instabilities can cause delta to be slightly negative despite
    // the equation admitting 2 real roots.
    let delta_sqrt = sqrt(max(b * b - 4 * a * c, 0));

    let r1 = (-b + delta_sqrt) / (2 * a);
    let r2 = (-b - delta_sqrt) / (2 * a);
    if abs(r1) >= abs(r2) {
        return vec2f(r1, r2);
    } else {
        return vec2f(r2, r1);
    }
}

fn get_eigen_val_2x2(m: mat2x2f, l: f32) -> vec2f {
    let a = 1;
    let b = -(m[0][0] + m[1][1]);
    let c = m[0][0] * m[1][1] - m[0][1] * m[1][0];

    get_real_polyroots_2(a, b, c, l)
}


///
///    return the eigen vectors with norm 1 for the eigen values l
///    M.e1 = l1.e1 ; M.e2 = l2.e2
///
fn get_eigen_vect_2x2(m: mat2x2f, l: vec2f) -> mat2x2f {
    // 2x2 algorithm : https://en.wikipedia.org/wiki/Eigenvalue_algorithm (9 August 2022 version)

    if m[0][1] == 0 && m[0][0] == m[1][1] {

        // m is multiple of identity, picking 2 ortogonal eigen vectors.
        return mat2x2f(
            1.0, 0.0,
            0.0, 1.0
        );
    } else {
        // averaging 2 for increased reliability
        let e10 = m[0][0] + m[0][1] - l[1];
        let e11 = m[1][0] + m[1][1] - l[1];

        if e10 == 0f {
            return mat2x2f(
                e10, 1f,
                1f, 0f,
            );
        } else if e11 == 0f {
            return mat2x2f(
                1f, e11,
                0f, 1f,
            );
        } else {
            let norm_ = sqrt(e10 * e10 + e11 * e11);
            e10 /= norm_;
            e11 /= norm_;

            let sign = copysign(1, e10) // for whatever reason, python has no sign func

            return mat2x2f(
                e10, e11,
                -e11 * sign,
                abs(e10)
            );
        }
    }
}


fn get_eigen_elmts_2x2(m: mat2x2f, l: vec2f) -> mat2x2f {
    // get_eigen_val_2x2(m, l);
    return get_eigen_vect_2x2(m, l);
}

fn interpolate_cov(covs: vec2f, center_pos: vec2f) -> mat2x2f {
    let reframed_posx = mod(center_pos[1]); // these positions are between 0 and 1
    let reframed_posy = mod(center_pos[0]);

    let out = mat2x2f(0f);
    
    // cov 00 is in (0,0) ; cov 01 in (0, 1) ; cov 01 in (1, 0), cov 11 in (1, 1)

    for (let i = 0; i < 2; i++) {
        for (let j = 0; j < 2; j++) {
            out[i][j] = covs[0][0][i][j] * (1f - reframed_posx) * (1f - reframed_posy) + covs[0][1][i][j] * reframed_posx * (1f - reframed_posy) + covs[1][0][i][j] * (1f - reframed_posx) * reframed_posy + covs[1][1][i][j] * reframed_posx * reframed_posy;
        }
    }

    out
}

fn bilinear_interpolation(values: mat2x2f, pos: vec2f) -> f32 {
    let posy = pos[0];
    let posx = pos[1];

    let val = values[0][0] * (1f - posx) * (1f - posy) + values[0][1] * posx * (1f - posy) + values[1][0] * (1f - posx) * posy + values[1][1] * posx * posy;

    return val;
}

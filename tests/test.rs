extern crate dislog_hal_curve25519;

use dislog_hal::{Bytes, DisLogPoint, Point, Scalar};
use dislog_hal_curve25519::{PointInner, ScalarInner};
use rand::thread_rng;

fn get_sim_scalar25519(a: u8) -> Scalar<ScalarInner> {
    let mut vec = [0u8; 32];
    vec[0] = a;

    Scalar {
        inner: ScalarInner::from_bytes(vec).unwrap(),
    }
}

#[test]
fn test_scalar() {
    let scalar_innera = ScalarInner::from_bytes([
        216, 154, 179, 139, 210, 121, 2, 71, 69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50,
        219, 153, 57, 249, 28, 82, 31, 197, 100, 165, 192, 8,
    ])
    .unwrap();
    let scalar_innerb = ScalarInner::from_bytes([
        216, 154, 179, 139, 210, 121, 2, 71, 69, 99, 158, 216, 23, 173, 63, 100, 204, 0, 91, 50,
        219, 153, 57, 249, 28, 82, 31, 197, 100, 165, 192, 8,
    ])
    .unwrap();

    let scalar_a = Scalar {
        inner: scalar_innera,
    };
    let scalar_b = Scalar {
        inner: scalar_innerb,
    };
    assert_eq!(scalar_a.clone(), scalar_b.clone());
    assert_eq!(
        scalar_a.clone() + scalar_a.clone() + scalar_a.clone(),
        scalar_b.clone() * get_sim_scalar25519(3)
    );

    assert_eq!(
        scalar_a.clone() * get_sim_scalar25519(2),
        scalar_a.clone() + scalar_b.clone()
    );

    assert_eq!(
        scalar_a.clone() * &get_sim_scalar25519(2),
        scalar_a.clone() + &scalar_b
    );

    assert_eq!(
        &scalar_a * get_sim_scalar25519(2),
        &scalar_a + scalar_b.clone()
    );

    assert_eq!(&scalar_a * &get_sim_scalar25519(2), &scalar_a + &scalar_b);

    //assert_eq!(get_sim_scalar25519(0), scalar_a - scalar_b);

    //assert_eq!(get_sim_scalar25519(0), &scalar_a - scalar_b);

    //assert_eq!(get_sim_scalar25519(0), scalar_a - &scalar_b);

    assert_eq!(get_sim_scalar25519(0), &scalar_a - &scalar_b);

    let inv_a = scalar_a.inv();

    assert_eq!(inv_a * scalar_a, get_sim_scalar25519(1));

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::order());

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::zero());

    println!("inv_a:{:?}\n", Scalar::<ScalarInner>::one());
}

#[test]
fn test_point() {
    let point_innerone = PointInner::one();
    let point_innerzero = PointInner::zero();

    let point_innera = PointInner::from_bytes([
        0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66,
    ])
    .unwrap();
    assert_eq!(point_innera, point_innerone);

    let point_innerb = PointInner::one();

    assert_eq!(
        point_innerone,
        PointInner::from_bytes(point_innerone.to_bytes()).unwrap()
    );

    let point_a = Point {
        inner: point_innera,
    };
    let point_b = Point {
        inner: point_innerb,
    };

    let point_one = Point {
        inner: point_innerone,
    };
    let point_zero = Point {
        inner: point_innerzero,
    };

    assert_eq!(
        Point::<PointInner>::one() + Point::<PointInner>::one() + Point::<PointInner>::one(),
        point_a * get_sim_scalar25519(3)
    );

    let point_last = point_b * (Scalar::<ScalarInner>::order() + (-get_sim_scalar25519(1)));

    assert_eq!(point_one.clone() + point_last.clone(), point_zero.clone());

    assert_eq!(&point_one + point_last.clone(), point_zero.clone());

    assert_eq!(point_one.clone() + &point_last, point_zero.clone());

    assert_eq!(&point_one + &point_last, point_zero.clone());

    assert_eq!(&point_last - &point_last, point_zero.clone());

    //assert_eq!(&point_last - point_last, point_last - point_last);

    //assert_eq!(point_last - &point_last, point_last - point_last);

    //assert_eq!(&point_last - &point_last, point_last - point_last);

    assert_eq!(
        Point {
            inner: PointInner::from_bytes([
                212, 180, 245, 120, 72, 104, 195, 2, 4, 3, 36, 103, 23, 236, 22, 159, 247, 158, 38,
                96, 142, 161, 38, 161, 171, 105, 238, 119, 209, 177, 103, 18
            ])
            .unwrap(),
        },
        point_one.clone() * get_sim_scalar25519(3)
    );

    //assert_eq!(&point_one * get_sim_scalar25519(3), point_one * get_sim_scalar25519(3));

    assert_eq!(
        point_one.clone() * &get_sim_scalar25519(3),
        point_one.clone() * get_sim_scalar25519(3)
    );

    assert_eq!(
        &point_one * &get_sim_scalar25519(3),
        point_one.clone() * get_sim_scalar25519(3)
    );

    assert_eq!(
        Point {
            inner: PointInner::from_bytes([
                212, 180, 245, 120, 72, 104, 195, 2, 4, 3, 36, 103, 23, 236, 22, 159, 247, 158, 38,
                96, 142, 161, 38, 161, 171, 105, 238, 119, 209, 177, 103, 18
            ])
            .unwrap(),
        },
        get_sim_scalar25519(3) * point_one.clone()
    );

    assert_eq!(
        &get_sim_scalar25519(3) * point_one.clone(),
        point_one.clone() * get_sim_scalar25519(3)
    );

    assert_eq!(
        get_sim_scalar25519(3) * &point_one,
        point_one.clone() * get_sim_scalar25519(3)
    );

    assert_eq!(
        &get_sim_scalar25519(3) * &point_one,
        point_one.clone() * get_sim_scalar25519(3)
    );

    // 4493907448824000747700850167940867464579944529806937181821189941592931634714
    let scalar_ax = Scalar {
        inner: ScalarInner::from_bytes([
            0x1a, 0x0e, 0x97, 0x8a, 0x90, 0xf6, 0x62, 0x2d, 0x37, 0x47, 0x02, 0x3f, 0x8a, 0xd8,
            0x26, 0x4d, 0xa7, 0x58, 0xaa, 0x1b, 0x88, 0xe0, 0x40, 0xd1, 0x58, 0x9e, 0x7b, 0x7f,
            0x23, 0x76, 0xef, 0x09,
        ])
        .unwrap(),
    };

    assert_eq!(
        scalar_ax.clone() * get_sim_scalar25519(5) * get_sim_scalar25519(3),
        scalar_ax.clone() * get_sim_scalar25519(15)
    );

    let mut rng = thread_rng();
    let rand_a = Scalar::<ScalarInner>::random(&mut rng);
    println!("{:?}", rand_a);
}

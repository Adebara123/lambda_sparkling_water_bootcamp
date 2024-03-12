use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;


/// This function calculates the public key derived from a provided private key, functioning within the parameters of the BLS12_381 elliptic curve.
/// So, the public key is determined by the scalar multiplication of the private key and the generator point.
/// The formula utilized for computing the public key is: pub_k = priv_k * G mod p.
/// 
pub fn generate_public_key (private_key: U256) -> ShortWeierstrassProjectivePoint<BLS12381Curve> 

{
    let generator = BLS12381Curve::generator();
    let public_key = generator.operate_with_self(private_key);

    public_key
}



fn main() {
    let private_key_hex = U256::from_hex_unchecked("6C616D6264617370");
    let generated_public_key = generate_public_key(private_key_hex);
    let public_key_bytes = generated_public_key.as_bytes();
    let public_key_u256 = U256::from_bytes_be(&public_key_bytes).expect("Failed to convert public key to U256");
    

    println!("public key: {:?}", public_key_u256.to_hex());
    //  This is the result "EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711"
}

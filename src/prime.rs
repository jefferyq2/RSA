///! Prime implements probabilistic prime checkers.
use num_bigint::BigUint;

pub fn probably_prime(n: &BigUint) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref primes: Vec<&'static str> = vec![
        "2",
        "3",
        "5",
        "7",
        "11",

        "13756265695458089029",
        "13496181268022124907",
        "10953742525620032441",
        "17908251027575790097",

        // https://golang.org/issue/638
        "18699199384836356663",

        "98920366548084643601728869055592650835572950932266967461790948584315647051443",
        "94560208308847015747498523884063394671606671904944666360068158221458669711639",

        // http://primes.utm.edu/lists/small/small3.html
        "449417999055441493994709297093108513015373787049558499205492347871729927573118262811508386655998299074566974373711472560655026288668094291699357843464363003144674940345912431129144354948751003607115263071543163",
        "230975859993204150666423538988557839555560243929065415434980904258310530753006723857139742334640122533598517597674807096648905501653461687601339782814316124971547968912893214002992086353183070342498989426570593",
        "5521712099665906221540423207019333379125265462121169655563495403888449493493629943498064604536961775110765377745550377067893607246020694972959780839151452457728855382113555867743022746090187341871655890805971735385789993",
        "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123",
        // ECC primes: http://tools.ietf.org/html/draft-ladd-safecurves-02
        "3618502788666131106986593281521497120414687020801267626233049500247285301239",                                                                                  // Curve1174: 2^251-9
        "57896044618658097711785492504343953926634992332820282019728792003956564819949",                                                                                 // Curve25519: 2^255-19
        "9850501549098619803069760025035903451269934817616361666987073351061430442874302652853566563721228910201656997576599",                                           // E-382: 2^382-105
        "42307582002575910332922579714097346549017899709713998034217522897561970639123926132812109468141778230245837569601494931472367",                                 // Curve41417: 2^414-17
        "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151", // E-521: 2^521-1
        ];
    }

    #[test]
    fn test_primes() {
        for prime in primes.iter() {
            assert!(
                probably_prime(&BigUint::parse_bytes(prime.as_bytes(), 10).unwrap()),
                "{} is a prime",
                prime
            );
        }
    }
}

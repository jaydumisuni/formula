use formula_check::rsa_public::{
    FixedWidthDecodeOperation, FixedWidthEncodeOperation, PublicRepresentationError,
    RsaPublicOperation, check_fixed_width_decode_result, check_fixed_width_encode_result,
    check_rsa_public_result,
};
use num_bigint::BigInt;
use std::str::FromStr;

fn n(value: &str) -> BigInt {
    BigInt::from_str(value).expect("valid bigint")
}

#[test]
fn fixed_width_encode_preserves_leading_zeroes() {
    let zero = FixedWidthEncodeOperation::new(n("0"), 4);
    let receipt = check_fixed_width_encode_result(&zero, "00000000").expect("zero");
    assert_eq!(receipt.result(), "00000000");

    let value = FixedWidthEncodeOperation::new(n("4660"), 4);
    assert!(check_fixed_width_encode_result(&value, "00001234").is_ok());
}

#[test]
fn fixed_width_decode_is_exact_and_canonical() {
    let operation = FixedWidthDecodeOperation::new("00001234", 4);
    let receipt = check_fixed_width_decode_result(&operation, "4660").expect("decode");
    assert_eq!(receipt.result(), "4660");
    assert_eq!(receipt.operation_digest(), operation.structural_digest());

    let uppercase = FixedWidthDecodeOperation::new("0000ABCD", 4);
    assert_eq!(
        check_fixed_width_decode_result(&uppercase, "43981"),
        Err(PublicRepresentationError::MalformedHex)
    );
}

#[test]
fn invalid_width_negative_value_and_overflow_fail_closed() {
    let zero_width = FixedWidthEncodeOperation::new(n("0"), 0);
    assert_eq!(
        check_fixed_width_encode_result(&zero_width, ""),
        Err(PublicRepresentationError::InvalidWidth)
    );

    let negative = FixedWidthEncodeOperation::new(n("-1"), 1);
    assert_eq!(
        check_fixed_width_encode_result(&negative, "ff"),
        Err(PublicRepresentationError::NegativeValue)
    );

    let overflow = FixedWidthEncodeOperation::new(n("65536"), 2);
    assert_eq!(
        check_fixed_width_encode_result(&overflow, "0000"),
        Err(PublicRepresentationError::WidthOverflow)
    );
}

#[test]
fn malformed_or_wrong_length_hex_fails_closed() {
    for bad in ["0", "000", "gg", "FF"] {
        let operation = FixedWidthDecodeOperation::new(bad, 1);
        assert_eq!(
            check_fixed_width_decode_result(&operation, "0"),
            Err(PublicRepresentationError::MalformedHex)
        );
    }
}

#[test]
fn rsa_public_operation_preserves_modulus_width() {
    let operation = RsaPublicOperation::new(n("2"), n("1"), n("257"));
    let receipt = check_rsa_public_result(&operation, "2", "0002").expect("rsa public");
    assert_eq!(receipt.result_decimal(), "2");
    assert_eq!(receipt.result_hex(), "0002");
    assert_eq!(receipt.width_bytes(), 2);
}

#[test]
fn rsa_public_operation_checks_2048_bit_65537_vector() {
    let operation = RsaPublicOperation::new(
        n(
            "1622971513381356112497908780401501818393532959521959230355352329062061531887234122609948577588520999574331754483506031170341435746919219550614058164548024742303384378267304830537752941148558442062195986716586461870561443769511939451700400156940274809180810610663755949654785957581908499400242712258990438406670115684112586087155650808814883540717890728850544710842452010084652608974726217326596327005445295243303109213395257558202867574028452179305826487072350140423171206354044965239845983137338673891396833598691884424204611167706355532893774362256298900121482306790505783991068646659324201294614649318799699097145",
        ),
        n("65537"),
        n(
            "19076043798060371941559465302453866169504163389425602492912251598253440742698694216939646292357740713747036398975753758933042895927533899096797324507908998842267335248882773101513693630118289484713512383296958217011449417049337316547985586308114378355884757699258805308685657944380999662689381166690279488797393512053106994364905902080695221743864208341582944757584837247414500416217099226860311785775525542307631752507361275451077246390532793259703876845074638192811866148775613103119461834891674906835159876220737964957641363380889938223812389190402835514196026544400588210743495647502436690084993354197379203081291",
        ),
    );

    let expected_decimal = "18204893117620222089877026006510965368894822555009917120031065664860289624418984161359687733742442933343388508810812445492329793023204951794836913710277679267811860055282238671563425551709765216492877840497094694775018172759839713124704944958913052522833151589098604608533374333673641456291477323102993451485109917244802442407077923167017256624509365300280144005489612509532569723784556838093348996933402360931242136787950339639299695627761147134702934254799342786856439263740534611690996728572317895671017668637877696864410518402264129605106654897975905590781110873267201857732227700335651348450848542129095021803540";
    let expected_hex = "9035e56ccc19dbf9d2960534c4b8c9fce4bdaf156d7f5e856f103c87b9c9624a4afb1d00ec90704e0e42072bc9037c954909d7ffdb81584a9c9bc5b28989c671d8ad5ce9de290e4dca793cc079fb591be61ec199713f4b8d5451a5818c801611d2914fe03b95a5d5621107efb35b531402d1e8cc20f18abb90aab6fc34593836dbe9bbd664d36c6ce8b5cbdfac4a238fcaf0f38358bf927aaf88da7843cd6da165fc935ea9bce1ac3ce47d259a0643fc77ce39480e4f157d24a082b90f96ca018fd7e34441d38bf044092357b91d7760dd71ecced2fdacd17a8cb4ced6ab35b448223acdeb6c0046ce62f381e47d547d01c68ea749eee66c8f53e87087adb814";

    let receipt =
        check_rsa_public_result(&operation, expected_decimal, expected_hex).expect("2048-bit");
    assert_eq!(receipt.width_bytes(), 256);
}

#[test]
fn rsa_domain_errors_fail_closed() {
    let bad_modulus = RsaPublicOperation::new(n("0"), n("65537"), n("1"));
    assert_eq!(
        check_rsa_public_result(&bad_modulus, "0", "00"),
        Err(PublicRepresentationError::InvalidModulus)
    );

    let bad_exponent = RsaPublicOperation::new(n("0"), n("0"), n("17"));
    assert_eq!(
        check_rsa_public_result(&bad_exponent, "0", "00"),
        Err(PublicRepresentationError::InvalidExponent)
    );

    for representative in ["-1", "17"] {
        let operation = RsaPublicOperation::new(n(representative), n("3"), n("17"));
        assert_eq!(
            check_rsa_public_result(&operation, "0", "00"),
            Err(PublicRepresentationError::RepresentativeOutOfRange)
        );
    }
}

#[test]
fn rsa_wrong_decimal_or_hex_is_rejected() {
    let operation = RsaPublicOperation::new(n("4"), n("3"), n("17"));
    assert_eq!(
        check_rsa_public_result(&operation, "14", "0d"),
        Err(PublicRepresentationError::IncorrectResult)
    );
    assert_eq!(
        check_rsa_public_result(&operation, "13", "0e"),
        Err(PublicRepresentationError::IncorrectResult)
    );
}

#[test]
fn rsa_result_decimal_must_be_canonical() {
    let operation = RsaPublicOperation::new(n("4"), n("3"), n("17"));
    for malformed in ["013", "+13", "13 ", " 13"] {
        assert_eq!(
            check_rsa_public_result(&operation, malformed, "0d"),
            Err(PublicRepresentationError::MalformedDecimal)
        );
    }
}

#[test]
fn structural_identity_binds_rsa_parameters() {
    let a = RsaPublicOperation::new(n("4"), n("3"), n("17"));
    let b = RsaPublicOperation::new(n("4"), n("5"), n("17"));
    let c = RsaPublicOperation::new(n("4"), n("3"), n("19"));

    assert_ne!(a.structural_digest(), b.structural_digest());
    assert_ne!(a.structural_digest(), c.structural_digest());
}

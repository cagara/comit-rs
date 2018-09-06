extern crate lightning_rpc;

use lightning_rpc::{macaroon::Macaroon, FromFile};

#[test]
fn read_from_file_to_hex() {
    let macaroon = Macaroon::from_file("./sample/admin.macaroon");
    assert!(macaroon.is_ok());
    assert_eq!(macaroon.unwrap().to_hex(), "0201036c6e6402bb01030a100d98a58d4e42ccf96433527fd51f84f41201301a160a0761646472657373120472656164120577726974651a130a04696e666f120472656164120577726974651a170a08696e766f69636573120472656164120577726974651a160a076d657373616765120472656164120577726974651a170a086f6666636861696e120472656164120577726974651a160a076f6e636861696e120472656164120577726974651a140a0570656572731204726561641205777269746500000620dca37ca2e7828f7e9b6a0d28b8cde41c75edd92205d66ea9b7e7169cfa718d85");
}
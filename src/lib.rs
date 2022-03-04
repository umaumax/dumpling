#[macro_export]
macro_rules! dump_hex {
    ($x:expr) => {
        $x.to_be_bytes()
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_HEX {
    ($x:expr) => {
        $x.to_be_bytes()
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_iter_hex {
    ($x:expr) => {
        format!(
            "[{}]",
            $x.iter()
                .map(|n| format!("{}, ", dump_hex!(n)))
                .collect::<String>()
                .strip_suffix(", ")
                .unwrap()
        )
    };
}

#[macro_export]
macro_rules! dump_iter_HEX {
    ($x:expr) => {
        format!(
            "[{}]",
            $x.iter()
                .map(|n| format!("{}, ", dump_HEX!(n)))
                .collect::<String>()
                .strip_suffix(", ")
                .unwrap()
        )
    };
}

#[macro_export]
macro_rules! dump_binary_hex {
    ($x:expr) => {
        $x.iter()
            .enumerate()
            .map(|(i, n)| format!("{:02x}{}", n, if i % 8 == 7 { " " } else { "" }))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_binary_HEX {
    ($x:expr) => {
        $x.iter()
            .enumerate()
            .map(|(i, n)| format!("{:02X}{}", n, if i % 8 == 7 { " " } else { "" }))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_sha1 {
    ($x:expr) => {
        sha1_smol::Sha1::from($x).digest().to_string()
    };
}

#[macro_export]
macro_rules! dump_SHA1 {
    ($x:expr) => {
        sha1_smol::Sha1::from($x)
            .digest()
            .to_string()
            .to_uppercase()
    };
}

#[macro_export]
macro_rules! dump_sha256 {
    ($x:expr) => {
        hmac_sha256::Hash::hash($x)
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_SHA256 {
    ($x:expr) => {
        hmac_sha256::Hash::hash($x)
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<String>()
    };
}

#[macro_export]
macro_rules! dump_md5 {
    ($x:expr) => {
        format!("{:x}", md5::compute($x))
    };
}

#[macro_export]
macro_rules! dump_MD5 {
    ($x:expr) => {
        format!("{:X}", md5::compute($x))
    };
}

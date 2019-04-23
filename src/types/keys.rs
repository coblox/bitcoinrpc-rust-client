use bitcoin::util::key;
use serde::{de, export::fmt, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt as std_fmt, str::FromStr};

#[derive(PartialEq)]
pub struct PrivateKey(key::PrivateKey);

impl From<key::PrivateKey> for PrivateKey {
    fn from(p: key::PrivateKey) -> Self {
        PrivateKey(p)
    }
}

impl From<PrivateKey> for key::PrivateKey {
    fn from(p: PrivateKey) -> Self {
        p.0
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'vde> de::Visitor<'vde> for Visitor {
            type Value = PrivateKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                formatter.write_str("a Wallet-Import-Format encoded value")
            }

            fn visit_str<E>(self, v: &str) -> Result<PrivateKey, E>
            where
                E: de::Error,
            {
                let privkey =
                    key::PrivateKey::from_str(v).map_err(|err| E::custom(format!("{}", err)))?;
                Ok(PrivateKey(privkey))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl std_fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut std_fmt::Formatter) -> std_fmt::Result {
        write!(f, "PrivateKey {{ #REDACTED# }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_private_key() {
        let private_key = PrivateKey(
            key::PrivateKey::from_str("cQ1DDxScq1rsYDdCUBywawwNVWTMwnLzCKCwGndC6MgdNtKPQ5Hz")
                .unwrap(),
        );

        let se_private_key = serde_json::to_string(&private_key).unwrap();
        let de_private_key = serde_json::from_str::<PrivateKey>(se_private_key.as_str()).unwrap();

        let priv_key: key::PrivateKey = private_key.into();
        let de_priv_key: key::PrivateKey = de_private_key.into();

        assert_eq!(priv_key.key, de_priv_key.key);
    }
}

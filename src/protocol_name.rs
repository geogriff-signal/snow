// Noise_NK_25519_ChaChaPoly_BLAKE2s
use std::str::FromStr;
use patterns::*;

#[derive(PartialEq, Debug)]
pub enum BaseChoice {
    Noise,
    NoisePSK,
}

impl FromStr for BaseChoice {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::BaseChoice::*;
        match s {
            "Noise"    => Ok(Noise),
            "NoisePSK" => Ok(NoisePSK),
            _          => Err("base type unsupported"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum DHChoice {
    Curve25519,
    Curve448,
}

impl FromStr for DHChoice {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::DHChoice::*;
        match s {
            "25519" => Ok(Curve25519),
            "448"   => Ok(Curve448),
            _       => Err("DH type unsupported")
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CipherChoice {
    ChaChaPoly,
    AESGCM,
}

impl FromStr for CipherChoice {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::CipherChoice::*;
        match s {
            "ChaChaPoly" => Ok(ChaChaPoly),
            "AESGCM"     => Ok(AESGCM),
            _            => Err("cipher type unsupported")
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum HashChoice {
    SHA256,
    SHA512,
    Blake2s,
    Blake2b,
}

impl FromStr for HashChoice {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::HashChoice::*;
        match s {
            "SHA256"  => Ok(SHA256),
            "SHA512"  => Ok(SHA512),
            "BLAKE2s" => Ok(Blake2s),
            "BLAKE2b" => Ok(Blake2b),
            _         => Err("hash type unsupported")
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct NoiseParams {
    pub base: BaseChoice,
    pub handshake: HandshakePattern,
    pub dh: DHChoice,
    pub cipher: CipherChoice,
    pub hash: HashChoice,
}

impl NoiseParams {
    pub fn new(base: BaseChoice,
               handshake: HandshakePattern,
               dh: DHChoice,
               cipher: CipherChoice,
               hash: HashChoice) -> Self
    {
         NoiseParams {
            base: base,
            handshake: handshake,
            dh: dh,
            cipher: cipher,
            hash: hash,
        }
    }
}

impl FromStr for NoiseParams {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('_');
        static TOO_FEW: &'static str = "too few parameters";
        Ok(NoiseParams::new(split.next().ok_or(TOO_FEW)?.parse()?,
                            split.next().ok_or(TOO_FEW)?.parse()?,
                            split.next().ok_or(TOO_FEW)?.parse()?,
                            split.next().ok_or(TOO_FEW)?.parse()?,
                            split.next().ok_or(TOO_FEW)?.parse()?))
    }
}

//let params = NoiseParams::new(BaseChoice::Noise,
//    HandshakePattern::NK,
//    DHChoice::Curve25519,
//    CipherChoice::ChaChaPoly,
//    HashChoice::Blake2s);
//
//// or...
//
//NoiseBuilder::new("Noise_NK_25519_ChaChaPoly_BLAKE2s")
//.psk(psk)
//.prologue(prologue)
//.local_private_key(privkey)
//.remote_public_key(pubkey)
//.initialize();
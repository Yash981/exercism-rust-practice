
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    // todo!("Convert the values {values:?} to a list of bytes")
    let mut bytes: Vec<u8> = vec![];
    for &x in values {
        bytes.push(x as u8);
    }
    return bytes;
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    // todo!("Convert the list of bytes {bytes:?} to a list of numbers")
    if bytes.len() == 1 {
        return Err(Error::IncompleteNumber);
    }
    let mut value = 0;
    for &x in  bytes{
        value = (value << 8) | (x as u32);
    }
    Ok(vec![value])

}

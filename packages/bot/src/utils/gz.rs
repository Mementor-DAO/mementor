use std::io::Read;
use flate2::read::GzDecoder;

#[allow(unused)]
pub fn decompress_to_str(
    data: &[u8]
) -> Result<String, String> {
    let mut e = GzDecoder::new(data);
    
    let mut s = String::new();
    e.read_to_string(&mut s)
        .map_err(|err| err.to_string())?;
        
    Ok(s)
}

pub fn decompress_to_vec(
    data: &[u8]
) -> Result<Vec<u8>, String> {
    let mut e = GzDecoder::new(data);
    
    let mut buf = Vec::new();
    e.read_to_end(&mut buf)
        .map_err(|err| err.to_string())?;
    
    Ok(buf)
}

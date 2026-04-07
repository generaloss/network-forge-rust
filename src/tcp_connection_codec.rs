
pub trait Codec: Send + Sync + 'static {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn try_decode(&self, buffer: &mut Vec<u8>) -> Option<Vec<u8>>;
}


pub struct PacketCodec;

impl Codec for PacketCodec {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + data.len());
        let len = data.len() as u32;
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(data);
        out
    }

    fn try_decode(&self, buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
        if buffer.len() < 4 {
            return None;
        }

        let len = u32::from_be_bytes(buffer[0..4].try_into().unwrap()) as usize;

        if buffer.len() < 4 + len {
            return None;
        }

        let data = buffer[4..4 + len].to_vec();
        buffer.drain(0..4 + len);
        Some(data)
    }
}


pub struct StreamCodec;

impl Codec for StreamCodec {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    fn try_decode(&self, buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
        if buffer.is_empty() {
            None
        } else {
            Some(std::mem::take(buffer))
        }
    }
}
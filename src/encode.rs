use std::io;
use std::io::Write;

pub(crate) trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()>;

    fn encode_to_vec(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }
}

impl Encode for u8 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for u16 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for u32 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for u64 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for u128 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for i8 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for i16 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for i32 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for i64 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for i128 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for f32 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Encode for f64 {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

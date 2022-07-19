use datastore::{Read, Reader, TypeWriter, Write, Writer};

use super::__Store;

impl Read<__Store> for bool {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_bool()
    }
}

impl Read<__Store> for i8 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_i8()
    }
}

impl Read<__Store> for i16 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_i16()
    }
}

impl Read<__Store> for i32 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_i32()
    }
}

impl Read<__Store> for i64 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_i64()
    }
}

impl Read<__Store> for u8 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_u8()
    }
}

impl Read<__Store> for u16 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_u16()
    }
}

impl Read<__Store> for u32 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_u32()
    }
}

impl Read<__Store> for u64 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_u64()
    }
}

impl Read<__Store> for f32 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_f32()
    }
}

impl Read<__Store> for f64 {
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<__Store>,
    {
        reader.read_f64()
    }
}

impl Write<__Store> for bool {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_bool(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_bool()
    }
}

impl Write<__Store> for i8 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_i8(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_i8()
    }
}

impl Write<__Store> for i16 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_i16(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_i16()
    }
}

impl Write<__Store> for i32 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_i32(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_i32()
    }
}

impl Write<__Store> for i64 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_i64(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_i64()
    }
}

impl Write<__Store> for u8 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_u8(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_u8()
    }
}

impl Write<__Store> for u16 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_u16(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_u16()
    }
}

impl Write<__Store> for u32 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_u32(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_u32()
    }
}

impl Write<__Store> for u64 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_u64(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_u64()
    }
}

impl Write<__Store> for f32 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_f32(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_f32()
    }
}

impl Write<__Store> for f64 {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<__Store>,
    {
        writer.write_f64(*self)
    }

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<__Store>,
    {
        writer.write_f64()
    }
}

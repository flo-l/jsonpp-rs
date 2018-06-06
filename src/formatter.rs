// This is an adaption of the PrettyFormatter from serde_json
use std::io;
use ::serde_json;

#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
    before_colon: &'a [u8],
    after_colon: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn new(indent: &'a [u8],
               before_colon: &'a [u8],
               after_colon: &'a [u8]
    ) -> Self
    {
        PrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
            before_colon,
            after_colon
        }
    }
}

impl<'a> serde_json::ser::Formatter for PrettyFormatter<'a> {
    #[inline]
    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            try!(writer.write_all(b"\n"));
            try!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: io::Write,
    {
        if first {
            try!(writer.write_all(b"\n"));
        } else {
            try!(writer.write_all(b",\n"));
        }
        try!(indent(writer, self.current_indent, self.indent));
        Ok(())
    }

    #[inline]
    fn end_array_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            try!(writer.write_all(b"\n"));
            try!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
            W: io::Write,
    {
        if first {
            try!(writer.write_all(b"\n"));
        } else {
            try!(writer.write_all(b",\n"));
        }
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        writer.write_all(self.before_colon)?;
        writer.write_all(b":")?;
        writer.write_all(self.after_colon)

    }

    #[inline]
    fn end_object_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}

fn indent<W: ?Sized>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
    where
        W: io::Write,
{
    for _ in 0..n {
        try!(wr.write_all(s));
    }

    Ok(())
}
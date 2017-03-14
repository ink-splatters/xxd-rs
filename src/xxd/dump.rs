//! The dump module contains code related for outputing/dumping data.
use std::fmt::Display;

/// Enum which provides all possible output value formats supported by the dump module.
#[derive(Debug)]
pub enum OutputFormat {
    Hex,
    Decimal,
    Octal,
    Binary,
}

/// The OutputLine struct contains all  information needed to dump/output a single line of data.
#[derive(Debug)]
pub struct OutputLine<'a> {
    start_address: u32,
    show_address: bool,
    data: &'a [u8], // TODO NiCo: Add member for data format (output format of data)
    group_size: usize,
    columns: usize,
    show_interpretation: bool,
    output_fmt: OutputFormat,
}

impl<'a> OutputLine<'a> {
    fn new(data: &[u8]) -> OutputLine {
        OutputLine {
            start_address: 0,
            show_address: true,
            data: data,
            group_size: 1,
            columns: 8,
            show_interpretation: true,
            output_fmt: OutputFormat::Hex,
        }
    }

    pub fn format(self, fmt: OutputFormat) -> Self {
        OutputLine {
            start_address: self.start_address,
            show_address: self.show_interpretation,
            data: self.data,
            group_size: self.group_size,
            columns: self.columns,
            show_interpretation: self.show_interpretation,
            output_fmt: fmt,
        }
    }

    fn write_address(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        write!(f, "{:08.X}: ", self.start_address)
    }

    fn write_bytes(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        let mut byte_count = 0;
        for b in self.data.iter() {
            byte_count += 1;
            let is_seperator_necessary = byte_count % self.group_size == 0;
            if is_seperator_necessary {
                self.write_formated_byte(f, b)?;
                write!(f, " ")?
            } else {
                self.write_formated_byte(f, b)?
            }
        }
        Ok(())
    }

    fn write_formated_byte(&self, f: &mut ::fmt::Formatter, byte: &u8) -> ::fmt::Result {
        match self.output_fmt {
            OutputFormat::Hex => write!(f, "{:02.X}", byte),
            OutputFormat::Octal => write!(f, "{:02.o}", byte),
            OutputFormat::Decimal => write!(f, "{:02}", byte),
            OutputFormat::Binary => write!(f, "{:02b}", byte),
        }
    }

    fn write_interpretation(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        write!(f, "   ");
        for b in self.data.iter() {
            match *b {
                character @ 20u8...126u8 => write!(f, "{}", character as char)?,
                _ => write!(f, "{}", ".")?,
            }
        }
        Ok(())
    }
}

impl<'a> ::fmt::Display for OutputLine<'a> {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        if self.show_address {
            self.write_address(f);
        }
        self.write_bytes(f);
        if self.show_interpretation {
            self.write_interpretation(f);
        }
        Ok(())
    }
}

mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn outputline_can_be_constructed() {
        let data = [1, 2, 3];
        let output_line = OutputLine::new(&data);
        assert!(true);
    }

    #[test]
    fn default_output_format_for_a_single_line() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected_output = "00000000: 01 02 03 04 05 06 07 08    ........";
        let output_line = OutputLine::new(&data);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_on_a_single_line() {
        let data = [8, 9, 10, 11, 12, 13, 14, 15];
        let expected_output = "00000000: 10 11 12 13 14 15 16 17    ........";
        let output_line = OutputLine::new(&data).format(OutputFormat::Octal);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_on_a_single_line() {
        let data = [65, 66, 67, 68, 126, 124, 60, 46];
        let expected_output = "00000000: 1000001 1000010 1000011 1000100 1111110 1111100 111100 101110    ABCD~|<.";
        let output_line = OutputLine::new(&data).format(OutputFormat::Binary);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn interpretation_for_default_settings() {
        let data = [65, 66, 67, 68, 126, 124, 60, 46];
        let expected_output = "00000000: 41 42 43 44 7E 7C 3C 2E    ABCD~|<.";
        let output_line = OutputLine::new(&data);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }
}
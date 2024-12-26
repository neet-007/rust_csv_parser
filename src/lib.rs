/*
    grammer as per "RFC 4180", "https://www.ietf.org/rfc/rfc4180.txt"
    file = [header CRLF] record *(CRLF record) [CRLF]

    header = name *(COMMA name)

    record = field *(COMMA field)

    name = field

    field = (escaped / non-escaped)

    escaped = DQUOTE *(TEXTDATA / COMMA / CR / LF / 2DQUOTE) DQUOTE

    non-escaped = *TEXTDATA

    COMMA = %x2C

    CR = %x0D ;as per section 6.1 of RFC 2234 [2]
* */
#![allow(warnings)] // At the top of the file
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum TokenType {
    Record,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Vec<String>,
    line: u64,
}

#[derive(Debug)]
pub struct CsvParser<R: Read> {
    tokens: Vec<Token>,
    reader: io::BufReader<R>,
    record_field_count: u64,
    line: u64,
    peek: Option<char>,
    has_header: bool,
    trim_space: bool,
    all_whitespace_empty: bool,
}

impl<R: Read> CsvParser<R> {
    pub fn new(reader: R) -> Self {
        let mut ret = CsvParser {
            tokens: Vec::<Token>::new(),
            reader: io::BufReader::new(reader),
            record_field_count: 0,
            line: 1,
            peek: None,
            has_header: true,
            trim_space: false,
            all_whitespace_empty: false,
        };
        ret.next_char();
        ret
    }

    pub fn with_header(mut self, flag: bool) -> Self {
        self.has_header = flag;
        self
    }

    pub fn trim_space(mut self, flag: bool) -> Self {
        self.trim_space = flag;
        self
    }

    pub fn all_whitespace_empty(mut self, flag: bool) -> Self {
        self.all_whitespace_empty = flag;
        self
    }

    fn next_char_(&mut self) -> io::Result<Option<char>> {
        let mut buf = [0u8; 4];
        let mut len = 0;

        match self.reader.read(&mut buf[0..1]) {
            Ok(0) => return Ok(None),
            Ok(_) => len += 1,
            Err(e) => return Err(e),
        }

        let utf8_len = match buf[0] {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
        };

        while len < utf8_len {
            match self.reader.read(&mut buf[len..len + 1]) {
                Ok(0) => break,
                Ok(_) => len += 1,
                Err(e) => return Err(e),
            }
        }

        match std::str::from_utf8(&buf[0..len]) {
            Ok(s) => Ok(s.chars().next()),
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
        }
    }

    fn next_char(&mut self) -> io::Result<Option<char>> {
        let ret = self.peek;
        match self.next_char_() {
            Ok(c) => self.peek = c,
            Err(err) => return Err(err),
        }

        return Ok(ret);
    }

    fn check_record_end(&mut self, record: &mut Vec<String>) -> io::Result<()> {
        if self.line == 1 {
            self.record_field_count = record.len() as u64;
        } else if record.len() as u64 != self.record_field_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "at line {:?} expect file to have {:?} fields found {:?}",
                    self.line,
                    self.record_field_count,
                    record.len()
                ),
            ));
        }
        self.tokens.push(Token {
            token_type: TokenType::Record,
            value: record.clone(),
            line: self.line,
        });
        self.line += 1;
        return Ok(());
    }

    fn match_char(&mut self, c: io::Result<Option<char>>, curr: &mut String) -> io::Result<bool> {
        match c {
            Ok(Some(c)) => match c {
                ',' => {
                    if self.all_whitespace_empty && curr.trim().is_empty() {
                        *curr = String::from("");
                    } else {
                        if self.trim_space {
                            *curr = curr.trim().to_owned().clone();
                        } else {
                            *curr = curr.clone();
                        }
                    }
                    return Ok(true);
                }
                '"' => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("at line {:?} found invalid char {c}", self.line),
                    ));
                }
                '\r' | '\n' => {
                    if self.all_whitespace_empty && curr.trim().is_empty() {
                        *curr = String::from("");
                    } else {
                        if self.trim_space {
                            *curr = curr.trim().to_owned().clone();
                        } else {
                            *curr = curr.clone();
                        }
                    }
                    return Ok(true);
                }
                _ => {
                    curr.push(c);
                    return Ok(false);
                }
            },
            Ok(None) => {
                return Ok(true);
            }
            Err(err) => return Err(err),
        }
    }

    fn parse_escaped(&mut self) -> io::Result<(String, bool)> {
        let mut curr = String::new();
        let mut count = 1;

        loop {
            match self.next_char() {
                Ok(Some(c)) => {
                    match c {
                        '"' => {
                            count += 1;
                            match self.peek {
                                Some(peek) => match peek {
                                    '"' => {
                                        count += 1;
                                        curr.push('"');
                                        self.next_char();
                                    }
                                    ',' => {
                                        if count % 2 != 0 {
                                            return Err(io::Error::new(
                                                io::ErrorKind::InvalidData,
                                                format!(
                                                    "at line {:?} unterninated escaped \"",
                                                    self.line
                                                ),
                                            ));
                                        }
                                        self.next_char();
                                        return Ok((curr, false));
                                    }
                                    '\r' | '\n' => {
                                        if count % 2 != 0 {
                                            return Err(io::Error::new(
                                                io::ErrorKind::InvalidData,
                                                format!(
                                                    "at line {:?} unterninated escaped \"",
                                                    self.line
                                                ),
                                            ));
                                        }
                                        return Ok((curr, true));
                                    }
                                    _ => {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            format!(
                                                "at line {:?} unterninated escaped \"",
                                                self.line
                                            ),
                                        ));
                                    }
                                },
                                None => {
                                    if count % 2 != 0 {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            format!(
                                                "at line {:?} unterninated escaped \"",
                                                self.line
                                            ),
                                        ));
                                    }
                                    return Ok((curr, true));
                                }
                            }
                        }
                        _ => {
                            curr.push(c);
                        }
                    };
                }
                Ok(None) => return Ok((curr, true)),
                Err(err) => return Err(err),
            }
        }

        Ok((curr, true))
    }

    fn parse_field(&mut self, first: char) -> io::Result<(String, Option<char>)> {
        let mut curr = String::new();
        let finshed = self.match_char(Ok(Some(first)), &mut curr)?;
        if finshed {
            return Ok((curr, Some(first)));
        }
        loop {
            let c = self.next_char();
            let ret_c = match c {
                Ok(c) => c,
                Err(err) => return Err(err),
            };
            let finshed = self.match_char(c, &mut curr)?;
            if finshed {
                return Ok((curr, ret_c));
            }
        }
    }

    fn parse_record(&mut self, first: char) -> io::Result<()> {
        let mut record = Vec::<String>::new();
        match first {
            '\n' | '\r' => {
                self.check_record_end(&mut record);
            }
            '"' => match self.parse_escaped() {
                Ok(res) => {
                    record.push(res.0);
                    if res.1 {
                        return self.check_record_end(&mut record);
                    }
                }
                Err(err) => return Err(err),
            },
            _ => match self.parse_field(first) {
                Ok(res) => {
                    record.push(res.0);
                    match res.1 {
                        Some(c) => match c {
                            ',' => match self.peek {
                                Some(peek) => {
                                    if peek == '\r' || peek == '\n' {
                                        record.push(String::from(""));
                                        return self.check_record_end(&mut record);
                                    }
                                }
                                None => {
                                    record.push(String::from(""));
                                    return self.check_record_end(&mut record);
                                }
                            },
                            '\r' | '\n' => {
                                return self.check_record_end(&mut record);
                            }
                            _ => {}
                        },
                        None => {
                            return self.check_record_end(&mut record);
                        }
                    }
                }
                Err(err) => return Err(err),
            },
        }
        loop {
            match self.next_char() {
                Ok(Some(c)) => match c {
                    '\n' | '\r' => {
                        self.check_record_end(&mut record);
                    }
                    '"' => match self.parse_escaped() {
                        Ok(res) => {
                            record.push(res.0);
                            if res.1 {
                                return self.check_record_end(&mut record);
                            }
                        }
                        Err(err) => return Err(err),
                    },
                    _ => match self.parse_field(c) {
                        Ok(res) => {
                            record.push(res.0);
                            match res.1 {
                                Some(c) => match c {
                                    ',' => match self.peek {
                                        Some(peek) => {
                                            if peek == '\r' || peek == '\n' {
                                                record.push(String::from(""));
                                                return self.check_record_end(&mut record);
                                            }
                                        }
                                        None => {
                                            record.push(String::from(""));
                                            return self.check_record_end(&mut record);
                                        }
                                    },
                                    '\r' | '\n' => {
                                        return self.check_record_end(&mut record);
                                    }
                                    _ => {}
                                },
                                None => {
                                    return self.check_record_end(&mut record);
                                }
                            }
                        }
                        Err(err) => return Err(err),
                    },
                },
                Ok(None) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
    }

    pub fn scan(&mut self) -> io::Result<Vec<Token>> {
        loop {
            match self.next_char() {
                Ok(Some(c)) => self.parse_record(c)?,
                Ok(None) => return Ok(self.tokens.clone()),
                Err(err) => return Err(err),
            };
        }
    }

    pub fn parse(&mut self) -> io::Result<Vec<Token>> {
        return self.scan();
    }
}

impl CsvParser<File> {
    pub fn from_path(file_path: &PathBuf) -> io::Result<Self> {
        match file_path.extension() {
            Some(extension) if extension != "csv" => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The file must have a .csv extension",
                ));
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The file must have an extension",
                ));
            }
            _ => {}
        }

        let file = File::open(file_path)?;
        Ok(Self::new(file))
    }
}

impl CsvParser<io::Cursor<String>> {
    pub fn from_string(input: String) -> Self {
        Self::new(io::Cursor::new(input))
    }
}

#[cfg(test)]
mod tests;

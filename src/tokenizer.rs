pub struct StreamInfo<'a> {
    file: &'a str,
    length: usize,
}

pub struct Stream<'a> {
    stream: &'a str,
    stream_info: StreamInfo<'a>
}

impl<'a> Stream<'a> {
    pub fn new(stream: &'a str, stream_info: StreamInfo<'a>) -> Self {
        Self { stream, stream_info }
    }
}

pub struct Location<'a> {
    stream_info: StreamInfo<'a>,
    line: usize,
    column: usize
}

struct TokenizerSetting<'a> {
    newline: Vec<&'a str>,
    reserved: Vec<&'a str>, 
    punctuator: Vec<&'a str>
}

struct Tokenizer<'a> {
    stream: Stream<'a>,
    next_location: Location<'a>,
    setting: TokenizerSetting<'a>
}
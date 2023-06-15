// ---------------------------------------------------------------------
// OpenMetrics parser
// @todo: parse labels
// @todo: parse timestamps
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use common::{AgentError, Label, Labels, Measure, Value};
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{alpha1, alphanumeric1, char, line_ending, space0, space1, u64},
    combinator::{eof, opt, recognize},
    multi::{many0, many0_count, separated_list0},
    number::complete::recognize_float_parts,
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Desc {
    metric_name: String,
    desc: String,
}

impl Desc {
    pub fn new<K: ToString, V: ToString>(name: K, value: V) -> Self {
        Desc {
            metric_name: name.to_string(),
            desc: value.to_string(),
        }
    }
    // Check if other description refers to different metrics
    pub fn is_differ(&self, other: Option<&Desc>) -> bool {
        match other {
            Some(x) => self.metric_name != x.metric_name,
            None => true,
        }
    }
}

#[derive(Debug, PartialEq)]
enum InternalValue {
    U64(u64),
    I64(i64),
    F32(f32),
}

#[derive(Debug, PartialEq)]
pub struct Metric {
    metric_name: String,
    labels: Labels,
    value: InternalValue,
    timestamp: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    DescType(Desc),
    DescHelp(Desc),
    DescUnit(Desc),
    Metric(Metric),
    Comment,
    Eof,
    EmptyLine,
}

// exposition = metricset HASH SP eof [ LF ]
// metricset = *metricfamily
// metricfamily = *metric-descriptor *metric
// metric-descriptor = HASH SP type SP metricname SP metric-type LF
// metric-descriptor =/ HASH SP help SP metricname SP escaped-string LF
// metric-descriptor =/ HASH SP unit SP metricname SP *metricname-char LF
// metric = *sample
// metric-type = counter / gauge / histogram / gaugehistogram / stateset
// metric-type =/ info / summary / unknown
// sample = metricname [labels] SP number [SP timestamp] [exemplar] LF
// exemplar = SP HASH SP labels SP number [SP timestamp]
// labels = "{" [label *(COMMA label)] "}"
// label = label-name EQ DQUOTE escaped-string DQUOTE
// number = realnumber
// ; Case insensitive
// number =/ [SIGN] ("inf" / "infinity")
// number =/ "nan"
// timestamp = realnumber
// ; Not 100% sure this captures all float corner cases.
// ; Leading 0s explicitly okay
// realnumber = [SIGN] 1*DIGIT
// realnumber =/ [SIGN] 1*DIGIT ["." *DIGIT] [ "e" [SIGN] 1*DIGIT ]
// realnumber =/ [SIGN] *DIGIT "." 1*DIGIT [ "e" [SIGN] 1*DIGIT ]
// ; RFC 5234 is case insensitive.
// ; Uppercase
// eof = %d69.79.70
// type = %d84.89.80.69
// help = %d72.69.76.80
// unit = %d85.78.73.84
// ; Lowercase
// counter = %d99.111.117.110.116.101.114
// gauge = %d103.97.117.103.101
// histogram = %d104.105.115.116.111.103.114.97.109
// gaugehistogram = gauge histogram
// stateset = %d115.116.97.116.101.115.101.116
// info = %d105.110.102.111
// summary = %d115.117.109.109.97.114.121
// unknown = %d117.110.107.110.111.119.110
// BS = "\"
// EQ = "="
// COMMA = ","
// HASH = "#"
// SIGN = "-" / "+"
// * [x] metricname = metricname-initial-char 0*metricname-char
// * [x] metricname-char = metricname-initial-char / DIGIT
// * [x] metricname-initial-char = ALPHA / "_" / ":"
// label-name = label-name-initial-char *label-name-char
// label-name-char = label-name-initial-char / DIGIT
// label-name-initial-char = ALPHA / "_"
// escaped-string = *escaped-char
// escaped-char = normal-char
// escaped-char =/ BS ("n" / DQUOTE / BS)
// escaped-char =/ BS normal-char
// ; Any unicode character, except newline, double quote, and backslash
// normal-char = %x00-09 / %x0B-21 / %x23-5B / %x5D-D7FF / %xE000-10FFFF

// Parse input to a vec of tokens
fn parse(input: &str) -> IResult<&str, Vec<Token>> {
    many0(line)(input)
}

// Parse single line and return Token
fn line(input: &str) -> IResult<&str, Token> {
    //alt((empty_line, hashed_line, metric_line))(input)
    alt((empty_line, hashed_line, metric_line))(input)
}

// Empty line
// Returns Token::EmptyLine
fn empty_line(input: &str) -> IResult<&str, Token> {
    let (rest, _) = pair(space0, line_ending)(input)?;
    Ok((rest, Token::EmptyLine))
}

// Parse line starting with `#`.
// May be Desc*, Eof or Comment
fn hashed_line(input: &str) -> IResult<&str, Token> {
    // Eat `#`
    let (input, _) = tag("#")(input)?;
    // HELP, TYPE, UNIT
    match alt((hash_help, hash_type, hash_unit, hash_eof))(input) {
        Ok(x) => Ok(x),
        Err(_) => hash_comment(input),
    }
}

// Parse comment until end of line
fn hash_comment(input: &str) -> IResult<&str, Token> {
    let (input, _) = pair(opt(is_not("\r\n")), alt((line_ending, eof)))(input)?;
    Ok((input, Token::Comment))
}

// Parse ...EOF -> Token::Eof
fn hash_eof(input: &str) -> IResult<&str, Token> {
    let (input, _) = space1(input)?;
    let (input, _) = tuple((tag("EOF"), space0, alt((line_ending, eof))))(input)?;
    Ok((input, Token::Eof))
}

// HELP <metric name> <help>
fn hash_help(input: &str) -> IResult<&str, Token> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("HELP")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = metric_name(input)?;
    let (input, _) = space1(input)?;
    let (input, help) = is_not("\r\n")(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Token::DescHelp(Desc::new(name, help))))
}
// TYPE <metric name> <type>
fn hash_type(input: &str) -> IResult<&str, Token> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("TYPE")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = metric_name(input)?;
    let (input, _) = space1(input)?;
    let (input, t) = alt((tag("counter"), tag("gauge")))(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Token::DescType(Desc::new(name, t))))
}
fn hash_unit(input: &str) -> IResult<&str, Token> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("UNIT")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = metric_name(input)?;
    let (input, _) = space1(input)?;
    let (input, unit) = is_not("\r\n")(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Token::DescUnit(Desc::new(name, unit))))
}
//
fn recognize_value(input: &str) -> IResult<&str, InternalValue> {
    let (input, parts) = recognize_float_parts(input)?;
    let value = match parts {
        (true, i, "", 0) => InternalValue::U64(i.parse().unwrap_or(0)),
        (false, i, "", 0) => InternalValue::I64(-i.parse().unwrap_or(0)),
        (true, i, f, e) => InternalValue::F32(format!("{}.{}e{}", i, f, e).parse().unwrap_or(0.0)),
        (false, i, f, e) => {
            InternalValue::F32(format!("-{}.{}e{}", i, f, e).parse().unwrap_or(0.0))
        }
    };
    Ok((input, value))
}
// <metric_name>[<labels>] SP <value>[ SP <timestamp>] <LF>
fn metric_line(input: &str) -> IResult<&str, Token> {
    // <metric_name>
    let (input, metric_name) = metric_name(input)?;
    // <labels>
    let (input, labels) = labels(input)?;
    // SP
    let (input, _) = space1(input)?;
    // <value>
    let (input, value) = recognize_value(input)?;
    // optional timestamp
    let (input, ts) = opt(tuple((space1, u64)))(input)?;
    let timestamp = ts.map(|(_, v)| v);
    // LF
    let (input, _) = alt((line_ending, eof))(input)?;
    // Result
    Ok((
        input,
        Token::Metric(Metric {
            metric_name: metric_name.into(),
            labels,
            value,
            timestamp,
        }),
    ))
}
// Match metric name:
// metricname = metricname-initial-char 0*metricname-char
// metricname-char = metricname-initial-char / DIGIT
// metricname-initial-char = ALPHA / "_" / ":"
fn metric_name(input: &str) -> IResult<&str, &str> {
    // metricname-initial-char 0*metricname-char
    recognize(pair(
        // metricname-initial-char = ALPHA / "_" / ":"
        alt((alpha1, tag("_"), tag(":"))),
        // metricname-char = metricname-initial-char / DIGIT
        many0_count(alt((alphanumeric1, tag("_"), tag(":")))),
    ))(input)
}

// label-name = label-name-initial-char *label-name-char
// label-name-char = label-name-initial-char / DIGIT
// label-name-initial-char = ALPHA / "_"
fn label_name(input: &str) -> IResult<&str, &str> {
    // label-name = label-name-initial-char *label-name-char
    recognize(pair(
        // label-name-initial-char = ALPHA / "_"
        alt((alpha1, tag("_"))),
        // label-name-char = label-name-initial-char / DIGIT
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

// labels = "{" [label *(COMMA label)] "}"
fn labels(input: &str) -> IResult<&str, Labels> {
    if !input.starts_with('{') {
        return Ok((input, Labels::default()));
    }
    let (input, _) = tag("{")(input)?;
    let (input, r) = separated_list0(tag(","), label)(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Labels::new(r)))
}

// label = label-name EQ DQUOTE escaped-string DQUOTE
fn label(input: &str) -> IResult<&str, Label> {
    // <label_name>
    let (input, name) = label_name(input)?;
    // ="
    let (input, _) = tag("=\"")(input)?;
    // Escaped string
    let (input, value) = escaped(is_not("\""), '\\', char('"'))(input)?;
    // "
    let (input, _) = tag("\"")(input)?;
    Ok((input, Label::new(name, value)))
}

#[derive(Default)]
pub struct ParsedMetrics(pub Vec<Measure>);

#[derive(Default)]
struct MetricDescriptor {
    metric_name: Option<String>,
    help: Option<String>,
    r#type: Option<InternalType>,
    units: Option<String>,
}

enum InternalType {
    Counter,
    Gauge,
}

impl MetricDescriptor {
    pub fn ensure_name(&mut self, name: String) {
        if let Some(x) = &self.metric_name {
            if *x == name {
                return;
            }
        }
        self.metric_name = Some(name);
        self.help = None;
        self.r#type = None;
        self.units = None;
    }

    pub fn set_help(&mut self, desc: Desc) {
        self.ensure_name(desc.metric_name);
        self.help = Some(desc.desc);
    }
    pub fn set_type(&mut self, desc: Desc) {
        self.ensure_name(desc.metric_name);
        self.r#type = Some(match desc.desc.as_str() {
            "counter" => InternalType::Counter,
            _ => InternalType::Gauge,
        });
    }
    pub fn set_units(&mut self, desc: Desc) {
        self.ensure_name(desc.metric_name);
        self.units = Some(desc.desc);
    }
    pub fn help(&self) -> String {
        self.help.clone().unwrap_or_else(|| "???".to_string())
    }
    pub fn value(&self, v: InternalValue) -> Result<Value, AgentError> {
        Ok(match self.r#type.as_ref().unwrap_or(&InternalType::Gauge) {
            InternalType::Counter => match v {
                InternalValue::U64(v) => Value::Counter(v),
                _ => return Err(AgentError::ParseError("invalid counter".to_string())),
            },
            InternalType::Gauge => match v {
                InternalValue::U64(v) => Value::Gauge(v),
                InternalValue::I64(v) => Value::GaugeI(v),
                InternalValue::F32(v) => Value::GaugeF(v),
            },
        })
    }
}

impl TryFrom<&str> for ParsedMetrics {
    type Error = AgentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, mut tokens) = parse(value).map_err(|e| AgentError::ParseError(e.to_string()))?;
        if tokens.is_empty() {
            return Ok(Self::default());
        }
        let mut desc = MetricDescriptor::default();
        let mut r = Vec::with_capacity(tokens.len());
        for t in tokens.drain(..) {
            match t {
                Token::DescType(d) => desc.set_type(d),
                Token::DescHelp(d) => desc.set_help(d),
                Token::DescUnit(d) => desc.set_units(d),
                Token::Metric(metric) => {
                    desc.ensure_name(metric.metric_name.clone());
                    let value = match desc.value(metric.value) {
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                    r.push(Measure {
                        name: metric.metric_name,
                        help: desc.help(),
                        value,
                        labels: metric.labels,
                    })
                }
                Token::Comment => {}
                Token::Eof => {}
                Token::EmptyLine => {}
            }
        }
        Ok(Self(r))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        empty_line, hash_comment, hash_eof, hash_help, hash_type, hash_unit, hashed_line,
        metric_name, parse, Desc, InternalValue, Labels, Metric, Token,
    };

    #[test]
    fn test_metricname() {
        assert!(metric_name("").is_err());
        assert!(metric_name("#").is_err());
        assert_eq!(metric_name("m"), Ok(("", "m")));
        assert_eq!(metric_name("_"), Ok(("", "_")));
        assert_eq!(metric_name(":"), Ok(("", ":")));
        assert!(metric_name("0").is_err());
        assert_eq!(metric_name("m1"), Ok(("", "m1")));
        assert_eq!(
            metric_name(":very_very:123:long"),
            Ok(("", ":very_very:123:long"))
        );
    }

    #[test]
    fn test_empty_line() {
        //assert!(empty_line("").is_ok());
        assert!(empty_line("\n").is_ok());
        assert!(empty_line("\r\n").is_ok());
        //assert!(empty_line(" ").is_ok());
        assert!(empty_line(" \n").is_ok());
        assert!(empty_line(" \r\n").is_ok());
        assert!(empty_line("     \n").is_ok());
        assert!(empty_line("     \r\n").is_ok());
        assert!(empty_line("   a   \n").is_err());
        assert_eq!(empty_line("  \n"), Ok(("", Token::EmptyLine)))
    }

    #[test]
    fn test_hash_eof() {
        assert_eq!(hash_eof(" EOF\n"), Ok(("", Token::Eof)));
        assert_eq!(hash_eof(" EOF   \r\n"), Ok(("", Token::Eof)));
        assert!(hash_eof(" EOF 123").is_err());
    }

    #[test]
    fn test_hash_comment() {
        assert_eq!(hash_comment(""), Ok(("", Token::Comment)));
        assert_eq!(hash_comment(" \n"), Ok(("", Token::Comment)));
        assert_eq!(hash_comment("zzzzzzz\n"), Ok(("", Token::Comment)));
    }

    #[test]
    fn test_hash_help() {
        assert_eq!(
            hash_help(" HELP mymetric spaced long help"),
            Ok((
                "",
                Token::DescHelp(Desc::new("mymetric", "spaced long help"))
            ))
        );
        assert_eq!(
            hash_help(" HELP mymetric spaced long help\n"),
            Ok((
                "",
                Token::DescHelp(Desc::new("mymetric", "spaced long help"))
            ))
        );
    }
    #[test]
    fn test_hash_type() {
        assert_eq!(
            hash_type(" TYPE mymetric counter"),
            Ok(("", Token::DescType(Desc::new("mymetric", "counter"))))
        );
        assert_eq!(
            hash_type(" TYPE mymetric gauge\n"),
            Ok(("", Token::DescType(Desc::new("mymetric", "gauge"))))
        );
        assert!(hash_type(" TYPE mymetric unknown").is_err());
    }
    #[test]
    fn test_hash_unit() {
        assert_eq!(
            hash_unit(" UNIT mymetric seconds"),
            Ok(("", Token::DescUnit(Desc::new("mymetric", "seconds"))))
        );
        assert_eq!(
            hash_unit(" UNIT mymetric seconds\n"),
            Ok(("", Token::DescUnit(Desc::new("mymetric", "seconds"))))
        );
        assert!(hash_unit(" TYPE mymetric unknown").is_err());
    }
    #[test]
    fn test_hash() {
        assert_eq!(
            hashed_line("# TYPE mymetric gauge"),
            Ok(("", Token::DescType(Desc::new("mymetric", "gauge"))))
        );
        assert_eq!(
            hashed_line("# UNIT mymetric seconds"),
            Ok(("", Token::DescUnit(Desc::new("mymetric", "seconds"))))
        );
        assert_eq!(
            hashed_line("# HELP mymetric long help"),
            Ok(("", Token::DescHelp(Desc::new("mymetric", "long help"))))
        );
        assert_eq!(hashed_line("# EOF"), Ok(("", Token::Eof)));
        assert_eq!(hashed_line("# long comment"), Ok(("", Token::Comment)));
    }
    #[test]
    fn test_parse() {
        let input = r#"# HELP metric1 first metric
# TYPE metric1 gauge
# UNIT metric1 seconds
metric1 12

# HELP metric2 second metric
# TYPE metric2 counter
# UNIT metric2 meters
metric2 -15

# EOF"#;
        assert_eq!(
            parse(input),
            Ok((
                "",
                vec![
                    Token::DescHelp(Desc::new("metric1", "first metric")),
                    Token::DescType(Desc::new("metric1", "gauge")),
                    Token::DescUnit(Desc::new("metric1", "seconds")),
                    Token::Metric(Metric {
                        metric_name: "metric1".into(),
                        labels: Labels::default(),
                        value: InternalValue::U64(12),
                        timestamp: None
                    }),
                    Token::EmptyLine,
                    Token::DescHelp(Desc::new("metric2", "second metric")),
                    Token::DescType(Desc::new("metric2", "counter")),
                    Token::DescUnit(Desc::new("metric2", "meters")),
                    Token::Metric(Metric {
                        metric_name: "metric2".into(),
                        labels: Labels::default(),
                        value: InternalValue::I64(-15),
                        timestamp: None
                    }),
                    Token::EmptyLine,
                    Token::Eof,
                ]
            ))
        );
    }
    #[test]
    fn test_parse_ts() {
        let input = r#"# HELP metric1 first metric
# TYPE metric1 gauge
# UNIT metric1 seconds
metric1 12 1686823614

# HELP metric2 second metric
# TYPE metric2 counter
# UNIT metric2 meters
metric2 -15 1686823614

# EOF"#;
        assert_eq!(
            parse(input),
            Ok((
                "",
                vec![
                    Token::DescHelp(Desc::new("metric1", "first metric")),
                    Token::DescType(Desc::new("metric1", "gauge")),
                    Token::DescUnit(Desc::new("metric1", "seconds")),
                    Token::Metric(Metric {
                        metric_name: "metric1".into(),
                        labels: Labels::default(),
                        value: InternalValue::U64(12),
                        timestamp: Some(1686823614),
                    }),
                    Token::EmptyLine,
                    Token::DescHelp(Desc::new("metric2", "second metric")),
                    Token::DescType(Desc::new("metric2", "counter")),
                    Token::DescUnit(Desc::new("metric2", "meters")),
                    Token::Metric(Metric {
                        metric_name: "metric2".into(),
                        labels: Labels::default(),
                        value: InternalValue::I64(-15),
                        timestamp: Some(1686823614),
                    }),
                    Token::EmptyLine,
                    Token::Eof,
                ]
            ))
        );
    }
}

// (C) Copyright 2019-2020 Hewlett Packard Enterprise Development LP

use std::convert::TryFrom;

use crate::dockerfile_parser::Instruction;
use crate::error::*;
use crate::parser::*;
use crate::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnvExeInstruction {
    pub span: Span,
}

impl EnvExeInstruction {
    pub(crate) fn from_record(record: Pair) -> Result<EnvExeInstruction> {
        let span = Span::from_pair(&record);
        Ok(Self { span })
    }
}

impl<'a> TryFrom<&'a Instruction> for &'a EnvExeInstruction {
    type Error = Error;

    fn try_from(instruction: &'a Instruction) -> std::result::Result<Self, Self::Error> {
        if let Instruction::EnvExeIsHostCwd(c) = instruction {
            Ok(c)
        } else {
            Err(Error::ConversionError {
                from: format!("{:?}", instruction),
                to: "EnvExeInstruction".into(),
            })
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//     use pretty_assertions::assert_eq;

//     use super::*;
//     use crate::test_util::*;
//     use crate::Span;

//     #[test]
//     fn cmd_basic() -> Result<()> {
//         assert_eq!(
//             parse_single(r#"CMD echo "hello world""#, Rule::cmd)?
//                 .as_cmd()
//                 .unwrap()
//                 .as_shell()
//                 .unwrap(),
//             &BreakableString::new((4, 22)).add_string((4, 22), "echo \"hello world\"")
//         );

//         assert_eq!(
//             parse_single(r#"CMD echo "hello world""#, Rule::cmd)?
//                 .as_cmd()
//                 .unwrap()
//                 .as_shell()
//                 .unwrap()
//                 .to_string(),
//             "echo \"hello world\""
//         );

//         assert_eq!(
//             parse_single(r#"cmd ["echo", "hello world"]"#, Rule::cmd)?,
//             EnvExeInstruction {
//                 span: Span::new(0, 27),
//                 expr: ShellOrExecExpr::Exec(StringArray {
//                     span: Span::new(4, 27),
//                     elements: vec![
//                         SpannedString {
//                             span: Span::new(5, 11),
//                             content: "echo".to_string(),
//                         },
//                         SpannedString {
//                             span: Span::new(13, 26),
//                             content: "hello world".to_string(),
//                         }
//                     ]
//                 }),
//             }
//             .into()
//         );

//         Ok(())
//     }

//     #[test]
//     fn cmd_multiline_exec() -> Result<()> {
//         assert_eq!(
//             parse_single(
//                 r#"cmd\
//         [\
//         "echo", \
//         "hello world"\
//         ]"#,
//                 Rule::cmd
//             )?,
//             LinkInstruction {
//                 span: Span::new(0, 66),
//                 expr: ShellOrExecExpr::Exec(StringArray {
//                     span: Span::new(13, 66),
//                     elements: vec![
//                         SpannedString {
//                             span: Span::new(24, 30),
//                             content: "echo".to_string(),
//                         },
//                         SpannedString {
//                             span: Span::new(42, 55),
//                             content: "hello world".to_string(),
//                         }
//                     ]
//                 }),
//             }
//             .into()
//         );

//         Ok(())
//     }

//     #[test]
//     fn cmd_multiline_shell() -> Result<()> {
//         assert_eq!(
//             parse_single(
//                 indoc!(
//                     r#"
//         cmd echo \
//           "hello world"
//       "#
//                 ),
//                 Rule::cmd
//             )?
//             .as_cmd()
//             .unwrap()
//             .as_shell()
//             .unwrap(),
//             &BreakableString::new((4, 26))
//                 .add_string((4, 9), "echo ")
//                 .add_string((11, 26), "  \"hello world\"")
//         );

//         assert_eq!(
//             parse_single(
//                 indoc!(
//                     r#"
//         cmd echo \
//           "hello world"
//       "#
//                 ),
//                 Rule::cmd
//             )?
//             .as_cmd()
//             .unwrap()
//             .as_shell()
//             .unwrap()
//             .to_string(),
//             "echo   \"hello world\""
//         );

//         Ok(())
//     }

//     #[test]
//     fn cmd_multiline_shell_large() -> Result<()> {
//         // note: the trailing `\` at the end is _almost_ nonsense and generates a
//         // warning from docker
//         let ins = parse_single(
//             indoc!(
//                 r#"
//         cmd set -x && \
//             # lorem ipsum
//             echo "hello world" && \
//             # dolor sit amet,
//             # consectetur \
//             # adipiscing elit, \
//             # sed do eiusmod
//             # tempor incididunt ut labore
//             echo foo && \
//             echo 'bar' \
//             && echo baz \
//             # et dolore magna aliqua."#
//             ),
//             Rule::cmd,
//         )?
//         .into_cmd()
//         .unwrap()
//         .into_shell()
//         .unwrap();

//         assert_eq!(
//             ins,
//             BreakableString::new((4, 266))
//                 .add_string((4, 14), "set -x && ")
//                 .add_comment((20, 33), "# lorem ipsum")
//                 .add_string((34, 60), "    echo \"hello world\" && ")
//                 .add_comment((66, 83), "# dolor sit amet,")
//                 .add_comment((88, 103), "# consectetur \\")
//                 .add_comment((108, 128), "# adipiscing elit, \\")
//                 .add_comment((133, 149), "# sed do eiusmod")
//                 .add_comment((154, 183), "# tempor incididunt ut labore")
//                 .add_string((184, 200), "    echo foo && ")
//                 .add_string((202, 217), "    echo 'bar' ")
//                 .add_string((219, 235), "    && echo baz ")
//                 .add_comment((241, 266), "# et dolore magna aliqua.")
//         );

//         assert_eq!(
//             ins.to_string(),
//             r#"set -x &&     echo "hello world" &&     echo foo &&     echo 'bar'     && echo baz "#
//         );

//         Ok(())
//     }
// }

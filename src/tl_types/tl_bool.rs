use crate::{tl_types::TLType, utils::MyResult};

const BOOL_TRUE: i32 = -1_720_552_011;
const BOOL_FALSE: i32 = -1_132_882_121;

impl TLType for bool {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let code = i32::tl_read(input)?;
        match code {
            BOOL_FALSE => Ok(false),
            BOOL_TRUE => Ok(true),
            _ => unreachable!(),
        }
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        if *self {
            BOOL_TRUE.tl_write(output)?;
        } else {
            BOOL_FALSE.tl_write(output)?;
        }
        Ok(4)
    }
}

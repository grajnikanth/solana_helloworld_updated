use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32) // This feild "Set" will represent the case that we want to store
    // u32 retrieved from the tranasaction sent by client/user
}

impl HelloInstruction {
    // The unpack function will define our custom logic of how the bytes sent
    // as instructions will be deserialized for our smart contract
    // we will take the argument with a u8 array slice as input for this function
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Returns the first and all the rest of the elements of the slice, 
        // or None if it is empty. It returns Option<&T, &[T]>

        // ok_or(E) transforms Option<T> into a Result<T,E> - None will result in Error
        // E is the error in Result if none
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        // tag will be a binary number. Does Rust automatically convert binary to
        // base 10? Below code assumes it would
        match tag {
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                // try_into() takes the array slice and converts into an array. In the
                // "val" definition we have to specify what type of array we want try_into
                // to convert to.
                // In this case we want 4 element array
                // Returns Result<T, self::Error>
                // Note array length is fixed at compile time
                let val: Result<[u8 ; 4], _> = rest[..4].try_into();
                match val {
                    // i is the array of u8s
                    // u32::from_le_bytes() converts an array into integer value
                    // using little endian left to right reading
                    Ok(i) => return Ok(HelloInstruction::Set(u32::from_le_bytes(i))),
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            }
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
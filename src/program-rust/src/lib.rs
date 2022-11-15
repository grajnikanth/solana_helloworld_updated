use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instruction;
use crate::instruction::HelloInstruction;


/// Define the type of state stored in accounts
/// GreetingAccount is an account for storing data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    // accounts is a slice of an array. It is a pointer to a slice of array stored in 
    // stack memory
    accounts: &[AccountInfo], // The account to say hello to
    // we will use the instruction_data to create custom instructions for this smart
    // contract
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    msg!("Instruction data {:?}", instruction_data);
    let instruction = HelloInstruction::unpack(instruction_data)?;
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    // owner is a field on the AccountInfo struct
    // the value of account.owner is created when the smart contract is first deployed
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize using borsh from binary data into GreetingAccount struct data
    // account was obtained from the client in this case. We take the account.data
    // and deserialize that to obtain the GreetingAccount Struct
    // Then we can access the counter field of the Rust struct
    // Increment and store the number of times the account has been greeted
    // account.data is in binary format
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        HelloInstruction::Increment => {greeting_account.counter += 1;},
        HelloInstruction::Decrement => {greeting_account.counter -= 1;},
        HelloInstruction::Set(val) => {greeting_account.counter = val;},
    }
  
    // Once the data in the account is updated, we save it by serializing it 
    // using Borsh library
    // [..] represents the entire slice I guess in this case
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        // Below code creates an account to test
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        // create a vector of the size equal to the struct GreetingAccount
        // in this case we only have one field "counter" which is u32. So
        // below the mem size is defining that data represting this is of the same size
        let mut data = vec![0; mem::size_of::<u32>()]; 
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        
        // creating an array of bytes from u32 = 100 using little endian format of creating
        // the byte array
        let arr = u32::to_le_bytes(100);
        println!("arr of bytes is {:?}", arr);
        // define an array of 5 elements all equal to 2 for now
        let mut instruction_data = [2;5];
        // add the arr[i] elements to the instruction_data array
        for i in 0..4 {
            instruction_data[i+1] = arr[i];
        }

        let accounts = vec![account];

        // Checking to verify that initially the counter == 0
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );

        // test that counter = 101 if a new insutruction of increment is sent now
        let instruction_data = [0; 5];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            101
        );

        // test that counter = 100 if a new insutruction of decrement is sent now
        let instruction_data = [1; 5];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );
    }

    // Test for crash
    #[test]
    #[should_panic]
    fn test_crash() {
        // Below code creates an account to test
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        // create a vector of the size equal to the struct GreetingAccount
        // in this case we only have one field "counter" which is u32. So
        // below the mem size is defining that data represting this is of the same size
        let mut data = vec![0; mem::size_of::<u32>()]; 
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        
        // creating an array of bytes from u32 = 100 using little endian format of creating
        // the byte array
        let arr = u32::to_le_bytes(100);
        println!("arr of bytes is {:?}", arr);
        // define an array of 5 elements all equal to 2 for now
        let mut instruction_data = [1;5];
        // add the arr[i] elements to the instruction_data array
        for i in 0..4 {
            instruction_data[i+1] = arr[i];
        }

        let accounts = vec![account];

        // Checking to verify that initially the counter == 0
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // the below should cause panic in smart contract as the counter
        // will be forced to be a negative number. The
        // [should_panic] macro is placed to pass this test that panic happens
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
 


    }


}

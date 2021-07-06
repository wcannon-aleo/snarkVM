// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{BlockError, BlockHeader, Transactions, testnet1::BaseDPCComponents};
use snarkvm_utilities::{
    bytes::{FromBytes, ToBytes},
    variable_length_integer::variable_length_integer,
};

use std::io::{Read, Result as IoResult, Write};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    /// First `HEADER_SIZE` bytes of the block as defined by the encoding used by "block" messages.
    pub header: BlockHeader,
    /// The block transactions.
    pub transactions: Transactions,
}

impl Block {
    pub fn write<C: BaseDPCComponents, W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.header.write(&mut writer)?;
        self.transactions.write::<C, _>(&mut writer)
    }

    pub fn read<C: BaseDPCComponents, R: Read>(mut reader: R) -> IoResult<Self> {
        let header: BlockHeader = FromBytes::read(&mut reader)?;
        let transactions = Transactions::read::<C, _>(&mut reader)?;

        Ok(Self { header, transactions })
    }

    pub fn serialize<C: BaseDPCComponents>(&self) -> Result<Vec<u8>, BlockError> {
        let mut serialization = vec![];
        serialization.extend(&self.header.serialize().to_vec());
        serialization.extend(&variable_length_integer(self.transactions.len() as u64));

        for transaction in self.transactions.iter() {
            transaction.write::<C, _>(&mut serialization)?;
        }

        Ok(serialization)
    }

    pub fn deserialize<C: BaseDPCComponents>(bytes: &[u8]) -> Result<Self, BlockError> {
        const HEADER_SIZE: usize = BlockHeader::size();
        let (header_bytes, transactions_bytes) = bytes.split_at(HEADER_SIZE);

        let mut header_array = [0u8; HEADER_SIZE];
        header_array.copy_from_slice(&header_bytes[0..HEADER_SIZE]);
        let header = BlockHeader::deserialize(&header_array);

        let transactions = Transactions::read::<C, _>(transactions_bytes)?;

        Ok(Block { header, transactions })
    }
}

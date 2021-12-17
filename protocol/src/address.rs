use crate::block::{Address, Block};
use crate::block_header::ChainBlock;
use crate::errors::ProtocolError;
use crate::errors::ProtocolError::NetParams;

// AddressParser parser node block to model block
struct AddressParser {
    net_params: String,
}

impl AddressParser {
    // new_address_parser new object
    fn new_address_parser(net_params: String) -> AddressParser {
        AddressParser { net_params }
    }

    // parse parser block to model
    fn parse(&self, block: impl ChainBlock, block_model: &mut Block) -> Result<(), ProtocolError> {
        for tx in block.transactions() {
            for input in tx.inputs() {
                self.update_addresses(input.control_program().as_str(), block_model)?;
            }

            for out in tx.outputs() {
                self.update_addresses(out.control_program().as_str(), block_model)?;
            }
        }

        Ok(())
    }

    fn update_addresses(&self, cp: &str, block_model: &mut Block) -> Result<(), ProtocolError> {
        let mut cps = cp;
        if !is_p2wpkh_script(cps) && !is_p2wsh_script(cps) {
            return Ok(());
        }

        let address = script_to_address(cps, self.net_params.clone())?;
        let mut script = hex::encode(&cps);
        let address = Address
        {
            script: script.clone(),
            address,
        };
        block_model.addresses.insert(script, address);

        Ok(())
    }
}


pub fn script_to_address(cps: &str, net_str: String) -> Result<String, ProtocolError> {
    if net_str.is_empty() {
        return Result::Err(NetParams("not find the network".to_string()));
    }

    Ok("".to_string())
}


pub fn is_p2wpkh_script(cp: &str) -> bool {
    false
}

pub fn is_p2wsh_script(cp: &str) -> bool {
    false
}
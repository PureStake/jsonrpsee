use jsonrpsee::{proc_macros::rpc, types::RpcResult};

#[rpc(client, server)]
pub trait DuplicatedSubAlias {
	#[subscription(name = "alias", item = String, aliases = ["hello_is_goodbye"], unsubscribe_aliases = ["hello_is_goodbye"])]
	fn async_method(&self) -> RpcResult<()>;
}

fn main() {}

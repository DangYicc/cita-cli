use client::basic::{Client, ClientExt};
use client::{remove_0x, TransactionOptions};

use std::str::{self, FromStr};

use abi::contract_encode_input;
use error::ToolError;
use ethabi::{Address, Contract};
use rpctypes::JsonRpcResponse;
use types::{traits::LowerHex, U256};

/// Group Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020009")]
#[contract(path = "../../contract_abi/Group.abi")]
#[contract(name = "GroupExt")]
pub struct GroupClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Call/SendTx to a contract method
pub trait ContractCall {
    /// Rpc response
    type RpcResult;

    /// Prepare contract call arguments
    fn prepare_call_args(
        &self,
        name: &str,
        values: &[&str],
        to_addr: Option<Address>,
    ) -> Result<(String, String), ToolError>;

    /// SendTx a contract method
    fn contract_send_tx(
        &mut self,
        name: &str,
        values: &[&str],
        quota: Option<u64>,
        to_addr: Option<Address>,
        blake2b: bool,
    ) -> Self::RpcResult;

    /// Call a contract method
    fn contract_call(
        &self,
        name: &str,
        values: &[&str],
        to_addr: Option<Address>,
        height: Option<&str>,
    ) -> Self::RpcResult;

    /// Call a contract method with a to_address
    fn contract_call_to_address(
        &self,
        function_name: &str,
        values: &[&str],
        address: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        let address = Address::from_str(remove_0x(address)).unwrap();
        self.contract_call(function_name, values, Some(address), height)
    }
}

/// Group System Contract
pub trait GroupExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Query the information of the group
    fn query_info(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryInfo", &[], address, height)
    }
    /// Query the name of the group
    fn query_name(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryName", &[], address, height)
    }
    /// Query the accounts of the group
    fn query_accounts(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryAccounts", &[], address, height)
    }
    /// Alias for query_child
    fn query_children(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.query_child(address, height)
    }
    /// Query the children of the group
    fn query_child(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryChild", &[], address, height)
    }
    /// Alias for query_child_length
    fn query_children_length(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.query_child_length(address, height)
    }
    /// Query the length of children of the group
    fn query_child_length(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryChildLength", &[], address, height)
    }
    /// Query the parent of the group
    fn query_parent(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryParent", &[], address, height)
    }
    /// Check the account in the group
    fn in_group(
        &self,
        address: &str,
        account_address: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        self.contract_call_to_address("inGroup", &[account_address], address, height)
    }
}

/// Group manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff02000a")]
#[contract(path = "../../contract_abi/GroupManagement.abi")]
#[contract(name = "GroupManagementExt")]
pub struct GroupManageClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// GroupManagement System Contract
pub trait GroupManagementExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Create a new group
    fn new_group(
        &mut self,
        origin: &str,
        name: &str,
        accounts: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(origin), name, accounts];
        self.contract_send_tx("newGroup", &values, quota, None, blake2b)
    }

    /// Delete the group
    fn delete_group(
        &mut self,
        origin: &str,
        target: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(origin), remove_0x(target)];
        self.contract_send_tx("deleteGroup", &values, quota, None, blake2b)
    }

    /// Update the group name
    fn update_group_name(
        &mut self,
        origin: &str,
        target: &str,
        name: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(origin), remove_0x(target), name];
        self.contract_send_tx("updateGroupName", &values, quota, None, blake2b)
    }

    /// Add accounts
    fn add_accounts(
        &mut self,
        origin: &str,
        target: &str,
        accounts: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(origin), remove_0x(target), accounts];
        self.contract_send_tx("addAccounts", &values, quota, None, blake2b)
    }

    /// Delete accounts
    fn delete_accounts(
        &mut self,
        origin: &str,
        target: &str,
        accounts: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(origin), remove_0x(target), accounts];
        self.contract_send_tx("deleteAccounts", &values, quota, None, blake2b)
    }

    /// Check the target group in the scope of the origin group
    ///   Scope: the origin group is the ancestor of the target group
    fn check_scope(&self, origin: &str, target: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(origin), remove_0x(target)];
        self.contract_call("checkScope", &values, None, height)
    }

    /// Query all groups
    fn query_groups(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("queryGroups", &[], None, height)
    }
}

/// Role Client
#[derive(ContractExt)]
#[contract(addr = "0x")]
#[contract(path = "../../contract_abi/Role.abi")]
#[contract(name = "RoleExt")]
pub struct RoleClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Role system contract
pub trait RoleExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Query the information of the role
    ///
    /// return The information of role: name and permissions
    fn query_role(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryRole", &[], address, height)
    }

    /// Query the name of the role
    ///
    /// return The name of role
    fn query_name(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryName", &[], address, height)
    }

    /// Query the permissions of the role
    ///
    /// return The permissions of role
    fn query_permissions(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryPermissions", &[], address, height)
    }

    /// Query the length of the permissions
    ///
    /// return The number of permission
    fn length_of_permissions(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("lengthOfPermissions", &[], address, height)
    }

    /// Check the duplicate permission
    ///
    /// return true if in permissions, otherwise false
    fn in_permissions(
        &self,
        address: &str,
        permission: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        let values = [remove_0x(permission)];
        self.contract_call_to_address("inPermissions", &values, address, height)
    }
}

/// Role manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020008")]
#[contract(path = "../../contract_abi/RoleManagement.abi")]
#[contract(name = "RoleManagementExt")]
pub struct RoleManageClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// RoleManagement system contract
pub trait RoleManagementExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Create a new role
    ///
    /// param name: The name of role
    /// param permissions: The permissions of role
    /// return New role's address
    fn new_role(
        &mut self,
        name: &str,
        permissions: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [name, permissions];
        self.contract_send_tx("newRole", &values, quota, None, blake2b)
    }

    /// Delete the role
    ///
    /// param role: The address of role
    /// return true if successed, otherwise false
    fn delete_role(&mut self, role: &str, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let values = [remove_0x(role)];
        self.contract_send_tx("deleteRole", &values, quota, None, blake2b)
    }

    /// Update role's name
    ///
    /// param role: The address of role
    /// param name: The new name of role
    /// return true if successed, otherwise false
    fn update_role_name(
        &mut self,
        role: &str,
        name: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(role), name];
        self.contract_send_tx("updateRoleName", &values, quota, None, blake2b)
    }

    /// Add permissions of role
    ///
    /// param role: The address of role
    /// param permissions: The permissions of role
    /// return true if successed, otherwise false
    fn add_permissions(
        &mut self,
        role: &str,
        permissions: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(role), permissions];
        self.contract_send_tx("addPermissions", &values, quota, None, blake2b)
    }

    /// Delete permissions of role
    ///
    /// param role: The address of role
    /// param permissions: The permissions of role
    /// return true if successed, otherwise false
    fn delete_permissions(
        &mut self,
        role: &str,
        permissions: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(role), permissions];
        self.contract_send_tx("deletePermissions", &values, quota, None, blake2b)
    }

    /// Set the role to the account
    ///
    /// param account: The account to be setted
    /// param role: The role to be setted
    /// return true if successed, otherwise false
    fn set_role(
        &mut self,
        account: &str,
        role: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account), remove_0x(role)];
        self.contract_send_tx("setRole", &values, quota, None, blake2b)
    }

    /// Cancel the account's role
    ///
    /// param account: The account to be canceled
    /// param role: The role to be canceled
    /// return true if successed, otherwise false
    fn cancel_role(
        &mut self,
        account: &str,
        role: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account), remove_0x(role)];
        self.contract_send_tx("cancelRole", &values, quota, None, blake2b)
    }

    /// Clear the account's role
    ///
    /// param account: The account to be cleared
    /// return true if successed, otherwise false
    fn clear_role(&mut self, account: &str, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let values = [remove_0x(account)];
        self.contract_send_tx("clearRole", &values, quota, None, blake2b)
    }

    /// Query the roles of the account
    ///
    /// param account: The account to be queried
    /// return The roles of the account
    fn query_roles(&self, account: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(account)];
        self.contract_call("queryRoles", &values, None, height)
    }

    /// Query the accounts that have the role
    ///
    /// param role: The role to be queried
    /// return The accounts that have the role
    fn query_accounts(&self, role: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(role)];
        self.contract_call("queryAccounts", &values, None, height)
    }
}

/// Role manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020006")]
#[contract(path = "../../contract_abi/Authorization.abi")]
#[contract(name = "AuthorizationExt")]
pub struct AuthorizationClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Authorization system contract
pub trait AuthorizationExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Query the account's permissions
    ///
    /// param account: The account to be queried
    /// return The permissions of account
    fn query_permissions(&self, account: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(account)];
        self.contract_call("queryPermissions", &values, None, height)
    }

    /// Query the permission's accounts
    ///
    /// param permission: The permission to be queried
    /// return The accounts of permission
    fn query_accounts(&self, permission: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(permission)];
        self.contract_call("queryAccounts", &values, None, height)
    }

    /// Query all accounts
    ///
    /// return All the accounts
    fn query_all_accounts(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("queryAllAccounts", &[], None, height)
    }

    /// Check Resource
    ///
    /// param account: The account to be checked
    /// param contract: The contract of resource
    /// param func: The function signature of resource
    /// return true if passed, otherwise false
    fn check_resource(
        &self,
        account: &str,
        contract: &str,
        func: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        let values = [remove_0x(account), remove_0x(contract), remove_0x(func)];
        self.contract_call("checkResource", &values, None, height)
    }

    /// Check account has a permission
    ///
    /// param _account The account to be checked
    /// param _permission The address of permission
    /// return true if passed, otherwise false
    fn check_permission(
        &self,
        account: &str,
        permission: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        let values = [remove_0x(account), remove_0x(permission)];
        self.contract_call("checkPermission", &values, None, height)
    }
}

/// Permission Client
#[derive(ContractExt)]
#[contract(addr = "0x")]
#[contract(path = "../../contract_abi/Permission.abi")]
#[contract(name = "PermissionExt")]
pub struct PermissionClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Permission system contract
pub trait PermissionExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Check resource in the permission
    ///
    /// param contract: The contract address of the resource
    /// param func: The function signature of the resource
    /// return true if in permission, otherwise false
    fn in_permission(
        &self,
        address: &str,
        contract: &str,
        func: &str,
        height: Option<&str>,
    ) -> Self::RpcResult {
        let values = [remove_0x(contract), remove_0x(func)];
        self.contract_call_to_address("inPermission", &values, address, height)
    }

    /// Query the information of the permission
    ///
    /// return The information of permission: name and resources
    fn query_info(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryInfo", &[], address, height)
    }

    /// Query the name of the permission
    ///
    /// return The name of permission
    fn query_name(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryName", &[], address, height)
    }

    /// Query the resource of the permission
    ///
    /// return The resources of permission
    fn query_resource(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call_to_address("queryResource", &[], address, height)
    }
}

/// Permission manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020004")]
#[contract(path = "../../contract_abi/PermissionManagement.abi")]
#[contract(name = "PermissionManagementExt")]
pub struct PermissionManageClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// PermissionManagement system contract
pub trait PermissionManagementExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Create a new permission
    ///
    /// param name: The name of permission
    /// param contracts: The contracts of resource
    /// param funcs: The function signature of the resource
    /// return New permission's address
    fn new_permission(
        &mut self,
        name: &str,
        contracts: &str,
        funcs: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [name, contracts, funcs];
        self.contract_send_tx("newPermission", &values, quota, None, blake2b)
    }

    /// Delete the permission
    ///
    /// param permission: The address of permission
    /// return true if successed, otherwise false
    fn delete_permission(
        &mut self,
        permission: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(permission)];
        self.contract_send_tx("deletePermission", &values, quota, None, blake2b)
    }

    /// Update the permission name
    ///
    /// param permission: The address of permission
    /// param name: The new name
    /// return true if successed, otherwise false
    fn update_permission_name(
        &mut self,
        permission: &str,
        name: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(permission), name];
        self.contract_send_tx("updatePermissionName", &values, quota, None, blake2b)
    }

    /// Add the resources of permission
    ///
    /// param permission: The address of permission
    /// param contracts: The contracts of resource
    /// param funcs: The function signature of resource
    /// return true if successed, otherwise false
    fn add_resources(
        &mut self,
        permission: &str,
        contracts: &str,
        funcs: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(permission), contracts, funcs];
        self.contract_send_tx("addResources", &values, quota, None, blake2b)
    }

    /// Delete the resources of permission
    ///
    /// param permission: The address of permission
    /// param contracts: The contracts of resource
    /// param funcs: The function signature of resource
    /// return true if successed, otherwise false
    fn delete_resources(
        &mut self,
        permission: &str,
        contracts: &str,
        funcs: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(permission), contracts, funcs];
        self.contract_send_tx("deleteResources", &values, quota, None, blake2b)
    }

    /// Set permission to the account
    ///
    /// param account: The account to be setted
    /// param permission: The permission to be setted
    /// return true if success, otherwise false
    fn set_authorization(
        &mut self,
        account_address: &str,
        permission: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account_address), remove_0x(permission)];
        self.contract_send_tx("setAuthorization", &values, quota, None, blake2b)
    }

    /// Set multiple permissions to the account
    ///
    /// param account: The account to be setted
    /// param permissions: The multiple permissions to be setted
    /// return true if success, otherwise false
    fn set_authorizations(
        &mut self,
        account_address: &str,
        permissions: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account_address), permissions];
        self.contract_send_tx("setAuthorizations", &values, quota, None, blake2b)
    }

    /// Cancel the account's permission
    ///
    /// param account: The account to be canceled
    /// param permissions: The permission to be canceled
    /// return true if success, otherwise false
    fn cancel_authorization(
        &mut self,
        account_address: &str,
        permission: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account_address), remove_0x(permission)];
        self.contract_send_tx("cancelAuthorization", &values, quota, None, blake2b)
    }

    /// Cancel the account's multiple permission
    ///
    /// param account: The account to be canceled
    /// param permissions: The multiple permissions to be canceled
    /// return true if success, otherwise false
    fn cancel_authorizations(
        &mut self,
        account_address: &str,
        permissions: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account_address), permissions];
        self.contract_send_tx("cancelAuthorizations", &values, quota, None, blake2b)
    }

    /// Clear the account's permissions
    ///
    /// param account: The account to be cleared
    /// return true if success, otherwise false
    fn clear_authorization(
        &mut self,
        account_address: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(account_address)];
        self.contract_send_tx("clearAuthorization", &values, quota, None, blake2b)
    }
}

/// Node manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020001")]
#[contract(path = "../../contract_abi/NodeManager.abi")]
#[contract(name = "NodeManagementExt")]
pub struct NodeManageClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// NodeManager system contract
pub trait NodeManagementExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Downgrade consensus node to ordinary node
    fn downgrade_consensus_node(
        &mut self,
        address: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_send_tx("deleteNode", &values, quota, None, blake2b)
    }

    /// Get node status
    fn node_status(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_call("getStatus", &values, None, height)
    }

    /// Get authorities
    fn get_authorities(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("listNode", &[], None, height)
    }

    /// Approve node upgrades to consensus nodes
    fn approve_node(
        &mut self,
        address: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_send_tx("approveNode", &values, quota, None, blake2b)
    }

    /// Node stake list
    fn list_stake(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("listStake", &[], None, height)
    }

    /// Set node stake
    fn set_stake(
        &mut self,
        address: &str,
        stake: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let values = [remove_0x(address), stake];
        self.contract_send_tx("setStake", &values, quota, None, blake2b)
    }

    /// Stake permillage
    fn stake_permillage(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("stakePermillage", &[remove_0x(address)], None, height)
    }
}

/// Node manage Client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020003")]
#[contract(path = "../../contract_abi/QuotaManager.abi")]
#[contract(name = "QuotaManagementExt")]
pub struct QuotaManageClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// QuotaManager system contract
pub trait QuotaManagementExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Get block quota upper limit
    fn get_bql(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getBQL", &[], None, height)
    }

    /// Get account quota upper limit of the specific account
    fn get_aql(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_call("getAQL", &values, None, height)
    }

    /// Get default account quota limit
    fn get_default_aql(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getDefaultAQL", &[], None, height)
    }

    /// Get accounts
    fn get_accounts(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getAccounts", &[], None, height)
    }

    /// Get quotas
    fn get_quotas(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getQuotas", &[], None, height)
    }

    /// Set block quota limit
    fn set_bql(&mut self, quota_limit: u64, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let quota_limit = format!("{}", quota_limit);
        let values = [quota_limit.as_str()];
        self.contract_send_tx("setBQL", &values, quota, None, blake2b)
    }

    /// Set default account quota limit
    fn set_default_aql(
        &mut self,
        quota_limit: u64,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let quota_limit = format!("{}", quota_limit);
        let values = [quota_limit.as_str()];
        self.contract_send_tx("setDefaultAQL", &values, quota, None, blake2b)
    }

    /// Set account quota upper limit of the specific account
    fn set_aql(
        &mut self,
        address: &str,
        quota_limit: u64,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let quota_limit = format!("{}", quota_limit);
        let values = [remove_0x(address), quota_limit.as_str()];
        self.contract_send_tx("setAQL", &values, quota, None, blake2b)
    }
}

/// Admin manage client
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff02000c")]
#[contract(path = "../../contract_abi/Admin.abi")]
#[contract(name = "AdminExt")]
pub struct AdminClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Admin system contract
pub trait AdminExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Get admin address
    fn admin(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("admin", &[], None, height)
    }

    /// Check if the account is admin
    fn is_admin(&self, address: &str, height: Option<&str>) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_call("isAdmin", &values, None, height)
    }

    /// Update admin account
    fn add_admin(&mut self, address: &str, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let values = [remove_0x(address)];
        self.contract_send_tx("update", &values, quota, None, blake2b)
    }
}

/// Batch transaction contract
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff02000e")]
#[contract(path = "../../contract_abi/BatchTx.abi")]
#[contract(name = "BatchTxExt")]
pub struct BatchTxClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// BatchTx system contract
pub trait BatchTxExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Multi transactions send once
    fn multi_transactions(
        &mut self,
        txs: Vec<&str>,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let combined_txs = txs
            .into_iter()
            .fold(String::with_capacity(100), |mut a, b| {
                let (address, parameters) = remove_0x(b).split_at(40);
                a.push_str(address);
                a.push_str(&format!("{:>08x}", parameters.len() / 2));
                a.push_str(parameters);
                a
            });
        let value = [combined_txs.as_ref()];
        self.contract_send_tx("multiTxs", &value, quota, None, blake2b)
    }
}

/// System config contract
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020000")]
#[contract(path = "../../contract_abi/SysConfig.abi")]
#[contract(name = "SysConfigExt")]
pub struct SysConfigClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// System config contract
pub trait SysConfigExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Get chain owner
    fn get_chain_owner(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getChainOwner", &[], None, height)
    }

    /// Get delay block number
    fn get_delay_block_number(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getDelayBlockNumber", &[], None, height)
    }

    /// Whether economic incentives are returned to operators
    fn get_feeback_platform_check(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getFeeBackPlatformCheck", &[], None, height)
    }

    /// Whether to open the charging mode
    fn get_economical_model(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getEconomicalModel", &[], None, height)
    }

    /// Whether to open the permission check
    fn get_permission_check(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getPermissionCheck", &[], None, height)
    }

    /// Whether to open the quota check
    fn get_quota_check(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getQuotaCheck", &[], None, height)
    }

    /// Set chain name
    fn set_chain_name(
        &mut self,
        chain_name: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let value = [chain_name];
        self.contract_send_tx("setChainName", &value, quota, None, blake2b)
    }

    /// Set operator
    fn set_operator(
        &mut self,
        operator: &str,
        quota: Option<u64>,
        blake2b: bool,
    ) -> Self::RpcResult {
        let value = [operator];
        self.contract_send_tx("setOperator", &value, quota, None, blake2b)
    }

    /// Set website
    fn set_website(&mut self, website: &str, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let value = [website];
        self.contract_send_tx("setWebsite", &value, quota, None, blake2b)
    }
}

/// Emergency brake contract
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff02000f")]
#[contract(path = "../../contract_abi/EmergencyBrake.abi")]
#[contract(name = "EmergencyBrakeExt")]
pub struct EmergencyBrakeClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Emergency brake contract
pub trait EmergencyBrakeExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Get state
    fn state(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("state", &[], None, height)
    }

    /// Set state
    fn set_state(&mut self, state: bool, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let state = state.to_string();
        let value = [state.as_str()];
        self.contract_send_tx("setState", &value, quota, None, blake2b)
    }
}

/// Price manager contract
#[derive(ContractExt)]
#[contract(addr = "0xffffffffffffffffffffffffffffffffff020010")]
#[contract(path = "../../contract_abi/PriceManager.abi")]
#[contract(name = "PriceManagerExt")]
pub struct PriceManagerClient {
    client: Client,
    address: Address,
    contract: Contract,
}

/// Price manager contract
pub trait PriceManagerExt: ContractCall {
    /// Create a ContractClient
    fn create(client: Option<Client>) -> Self;

    /// Get quota price
    fn price(&self, height: Option<&str>) -> Self::RpcResult {
        self.contract_call("getQuotaPrice", &[], None, height)
    }

    /// Set quota price
    fn set_price(&mut self, price: U256, quota: Option<u64>, blake2b: bool) -> Self::RpcResult {
        let price = format!("{:0>64}", price.lower_hex());
        let value = [price.as_str()];
        self.contract_send_tx("setQuotaPrice", &value, quota, None, blake2b)
    }
}

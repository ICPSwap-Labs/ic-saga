use certified_vars::hashtree::{fork, fork_hash};
use certified_vars::{Map, AsHashTree, Hash, HashTree};
use ic_kit::macros::*;
use ic_kit::ic;
use ic_kit::{Principal, RejectionCode};
use ic_kit::candid::{candid_method, CandidType, Deserialize};
use serde::Serialize;

mod upgrade;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EventNodeIns {
    pub name: String,
    pub cid: String,
    pub func_name: String,
    pub compensate_func_name: String,
    pub pre_node: String,
    pub next_node: String,
    pub args: Vec<(String, ArgValue)>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EventNode {
    pub name: String,
    pub cid: String,
    pub func_name: String,
    pub compensate_func_name: String,
    pub pre_node: String,
    pub next_node: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EventLang {
    pub name: String,
    pub comment: String,
    pub owner: Principal,
    pub time: u64,
    pub nodes: Vec<EventNode>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EventLangRequest {
    pub name: String,
    pub comment: String,
    pub nodes: Vec<EventNode>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ArgValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<ArgValue>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EventNodeRequest {
    pub name: String,
    pub args: Vec<(String, ArgValue)>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EventRequest {
    pub name: String,
    pub nodes: Vec<EventNodeRequest>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UserData {
    pub event_langs: Map<String, EventLang>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Data {
    pub user_events: Map<Principal, UserData>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct NodeLog {
    pub name: String,
    pub execute_status: bool,
    pub compensate_execute_status: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Log {
    pub time: u64,
    pub logs: Vec<NodeLog>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Logs {
    pub name: String,
    pub logs: Vec<Log>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct EventLogs {
    pub user_name: String,
    pub event_logs: Map<String, Logs>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UserEventLogs {
    pub user_logs: Map<Principal, EventLogs>,
}

impl AsHashTree for EventNodeIns {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &self.name.root_hash(), 
            &self.name.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            self.name.as_hash_tree(),
            self.name.as_hash_tree(),
        )
    }
}

impl AsHashTree for EventLang {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &self.name.root_hash(), 
            &self.name.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            self.name.as_hash_tree(),
            self.name.as_hash_tree(),
        )
    }
}

impl AsHashTree for UserData {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &self.event_langs.root_hash(), 
            &self.event_langs.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            self.event_langs.as_hash_tree(),
            self.event_langs.as_hash_tree(),
        )
    }
}

impl AsHashTree for Logs {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &self.name.root_hash(), 
            &self.name.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            self.name.as_hash_tree(),
            self.name.as_hash_tree(),
        )
    }
}

impl AsHashTree for EventLogs {
    fn root_hash(&self) -> Hash {
        fork_hash(
            &self.user_name.root_hash(), 
            &self.user_name.root_hash(),
        )
    }

    fn as_hash_tree(&self) -> HashTree<'_> {
        fork(
            self.user_name.as_hash_tree(),
            self.user_name.as_hash_tree(),
        )
    }
}

impl EventRequest {
    #[inline]
    pub fn to_event_node_ins_map(self, lang_nodes: Vec<EventNode>) -> Map<String, EventNodeIns> {
        let mut map = Map::<String, EventNodeIns>::new();
        for lang_node in lang_nodes {
            let mut args = Vec::new();
            for node in self.nodes.iter() {
                if lang_node.name == node.name {
                    args = node.args.clone();
                    break;
                };
            };
            let node_name = lang_node.name.clone();
            let node_ins = EventNodeIns {
                name: lang_node.name,
                cid: lang_node.cid,
                func_name: lang_node.func_name,
                compensate_func_name: lang_node.compensate_func_name,
                pre_node: lang_node.pre_node,
                next_node: lang_node.next_node,
                args: args,
            };
            map.insert(node_name, node_ins);
        };
        map
    }
}

impl EventLangRequest {
    #[inline]
    pub fn to_event_lang(self, owner: Principal, time: u64) -> EventLang {
        EventLang {
            owner,
            time,
            name: self.name,
            comment: self.comment,
            nodes: self.nodes,
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            user_events: Map::new(),
        }
    }
}

impl Default for UserEventLogs {
    fn default() -> Self {
        UserEventLogs {
            user_logs: Map::new(),
        }
    }
}

#[query]
#[candid_method(query)]
fn get(name: String) -> Option<EventLang>
{
    let caller = ic::caller();
    let data = ic::get_mut::<Data>();

    let result = match data.user_events.get(&caller) {
        Some(events) => {
            match events.event_langs.get(&name) {
                Some(event) => Some(event.clone()),
                None => None,
            }
        },
        None => None,
    };

    result
}

#[query]
#[candid_method(query)]
fn find() -> Vec<EventLang>
{
    let mut ret = Vec::<EventLang>::new();

    let caller = ic::caller();
    let data = ic::get_mut::<Data>();

    match data.user_events.get(&caller) {
        Some(events) => {
            for ele in events.event_langs.iter() {
                ret.push(ele.1.clone());
            };
        },
        None => {},
    };

    ret
}

#[update]
#[candid_method(update)]
fn insert(request: EventLangRequest) -> bool
{
    let caller = ic::caller();
    let data = ic::get_mut::<Data>();

    let event_name = request.name.clone();
    let event_lang = request.to_event_lang(caller, ic::time() / 1_000_000);

    match data.user_events.get_mut(&caller) {
        Some(events) => {
            events.event_langs.insert(event_name, event_lang);
        },
        None => {
            let mut new_user_data = UserData { 
                event_langs : Map::<String, EventLang>::new(), 
            };
            new_user_data.event_langs.insert(event_name, event_lang);
            data.user_events.insert(caller, new_user_data);
        },
    };

    true
}

#[update]
#[candid_method(update)]
fn delete(name: String) -> bool
{
    let caller = ic::caller();
    let data = ic::get_mut::<Data>();

    match data.user_events.get_mut(&caller) {
        Some(events) => {
            events.event_langs.remove(&name);
        },
        None => {},
    };

    true
}

#[query]
#[candid_method(query)]
fn find_log(name: String) -> Vec<Log>
{
    let caller = ic::caller();
    let user_event_logs = ic::get_mut::<UserEventLogs>();

    let ret = match user_event_logs.user_logs.get(&caller) {
        Some(event_logs) => {
            match event_logs.event_logs.get(&name) {
                Some(logs) => { logs.logs.clone() }
                None => { Vec::new() }
            }
        },
        None => {
            Vec::new()
        },
    };

    ret
}

#[update]
#[candid_method(update)]
pub async fn execute(request: EventRequest) -> Log
{
    let caller = ic::caller();
    let event_name = request.name.clone();

    let event_lang = match get(event_name.clone()) {
        Some(lang) => lang.clone(),
        None => ic::trap("Event lang is not found."),
    };

    let node_map = request.to_event_node_ins_map(event_lang.nodes);

    let log = &mut Log {
        time: ic::time() / 1_000_000,
        logs: Vec::new(),
    };

    let mut node_name = "root";

    while node_name != "end" {
        let status = match node_map.get(node_name) {
            Some(node) => {
                match exec_call(node.cid.clone(), node.func_name.clone(), node.args.clone()).await {
                    Ok(res) => {
                        if res {
                            node_name = &node.next_node;
                            log.logs.push(buildNodeLog(node.name.clone(), true, false));
                            true
                        } else {
                            execute_compensate(node_name, &node_map, log).await;
                            false
                        }
                    },
                    Err(_) => {
                        execute_compensate(node_name, &node_map, log).await;
                        false
                    },
                }
            },
            None => {
                execute_compensate(node_name, &node_map, log).await;
                false
            },
        };
        if !status {
            break;
        };
    };

    // node_name == "end"
    let user_event_logs = ic::get_mut::<UserEventLogs>();
    user_event_logs.user_logs.get_mut(&caller);
    match user_event_logs.user_logs.get_mut(&caller) {
        Some(event_logs) => {
            match event_logs.event_logs.get_mut(&event_name) {
                Some(logs) => { logs.logs.push(log.clone()) },
                None => { 
                    let mut logs = Logs {
                        name: event_name.clone(),
                        logs: Vec::<Log>::new(),
                    };
                    logs.logs.push(log.clone());
                },
            };
        },
        None => {
            let mut event_logs = EventLogs { 
                user_name: caller.to_text(),
                event_logs: Map::<String, Logs>::new(), 
            };
            match event_logs.event_logs.get_mut(&event_name) {
                Some(logs) => { logs.logs.push(log.clone()) },
                None => { 
                    let mut logs = Logs {
                        name: event_name.clone(),
                        logs: Vec::<Log>::new(),
                    };
                    logs.logs.push(log.clone());
                },
            };
            user_event_logs.user_logs.insert(caller, event_logs);
        },
    };
    
    log.clone()
}

async fn execute_compensate(node_name: &str, node_map: &Map<String, EventNodeIns>, log: &mut Log) -> ()
{
    let mut compensate_node_name = node_name;
    while compensate_node_name != "none" {
        match node_map.get(compensate_node_name) {
            Some(node) => {
                match exec_call(node.cid.clone(), node.compensate_func_name.clone(), node.args.clone()).await {
                    Ok(status) => {
                        compensate_node_name = &node.pre_node;
                        log.logs.push(buildNodeLog(node.name.clone(), false, status));
                    },
                    Err(_) => {},
                };
            },
            None => {},
        }
    };
}

#[update]
#[candid_method(update)]
pub async fn test(cid: String, func_name: String, args: Vec<(String, ArgValue)>) -> bool
{
    let response: Result<bool, (RejectionCode, String)> = exec_call(cid, func_name, args).await;

    let result = match response {
        Ok(_) => { true },
        Err(_) => { false },
    };

    result
}

async fn exec_call(cid: String, func_name: String, args: Vec<(String, ArgValue)>) -> Result<bool, (RejectionCode, String)>
{
    let response: (bool,) = ic::call(
        Principal::from_text(&cid).unwrap(), 
        func_name, 
        (args,)
    )
    .await?;

    Ok(response.0)
}

fn buildNodeLog(name: String, execute_status: bool, compensate_execute_status: bool) -> NodeLog {
    NodeLog { 
        name: name, 
        execute_status: execute_status, 
        compensate_execute_status: compensate_execute_status,
    }
}
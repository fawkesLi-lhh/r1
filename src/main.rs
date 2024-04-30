use std::{io::Read, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ParamPermission {
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,                 // 参数名
    pub desc: String,                 // 描述
    pub value: ParamValue,            // 配置的值
    pub default: ParamValue,          // 默认值
    pub enabled: bool,                // 是否授权
    pub allow_range: ParamValueRange, // 允许使用参数范围
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum ParamValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ParamValueRange {
    EnumList(Vec<ParamValue>),     // 白名单
    Range(ParamValue, ParamValue), // 区间
    Any,                           // 任意
}

fn main() {
    let mut pp = Vec::new();
    let mut params = Vec::new();
    params.push(Param {
        name: "reachlimit_continue".to_string(),
        desc: "涨跌停继续交易".to_string(),
        value: ParamValue::Bool(true),
        default: ParamValue::Bool(true),
        enabled: true,
        allow_range: ParamValueRange::Any,
    });
    params.push(Param {
        name: "overtime_continue".to_string(),
        desc: "超时后继续交易".to_string(),
        value: ParamValue::Bool(true),
        default: ParamValue::Bool(true),
        enabled: true,
        allow_range: ParamValueRange::Any,
    });
    params.push(Param {
        name: "strategy_type".to_string(),
        desc: "算法类型".to_string(),
        value: ParamValue::Int(100),
        default: ParamValue::Int(100),
        enabled: true,
        allow_range: ParamValueRange::EnumList(vec![
            ParamValue::Int(100),
            ParamValue::Int(200),
            ParamValue::Int(300),
            ParamValue::Int(1000),
        ]),
    });
    pp.push(ParamPermission {
        name: "交易参数".to_string(),
        params: params.clone(),
    });
    params.clear();
    params.push(Param {
        name: "self_match".to_string(),
        desc: "账户自成交".to_string(),
        value: ParamValue::Bool(false),
        default: ParamValue::Bool(true),
        enabled: true,
        allow_range: ParamValueRange::EnumList(vec![
            ParamValue::Bool(false),
            ParamValue::Bool(true),
        ]),
    });
    params.push(Param {
        name: "max_expo_amt".to_string(),
        desc: "最大净敞口价值".to_string(),
        value: ParamValue::Float(0.0),
        default: ParamValue::Float(0.0),
        enabled: true,
        allow_range: ParamValueRange::Range(ParamValue::Float(0.0), ParamValue::Float(1.0)),
    });
    params.push(Param {
        name: "pov".to_string(),
        desc: "市场占比".to_string(),
        value: ParamValue::Float(0.0),
        default: ParamValue::Float(0.0),
        enabled: true,
        allow_range: ParamValueRange::Range(ParamValue::Float(0.0), ParamValue::Float(1.0)),
    });
    pp.push(ParamPermission {
        name: "移仓".to_string(),
        params: params.clone(),
    });
    println!("{}", serde_json::to_string(&pp).unwrap());
}